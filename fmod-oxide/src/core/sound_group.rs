// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_float, c_int};

use fmod_sys::*;
use lanyard::Utf8CString;

use crate::{Sound, SoundGroupBehavior, System};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct SoundGroup {
    pub(crate) inner: *mut FMOD_SOUNDGROUP,
}

unsafe impl Send for SoundGroup {}
unsafe impl Sync for SoundGroup {}

impl From<*mut FMOD_SOUNDGROUP> for SoundGroup {
    fn from(value: *mut FMOD_SOUNDGROUP) -> Self {
        SoundGroup { inner: value }
    }
}

impl From<SoundGroup> for *mut FMOD_SOUNDGROUP {
    fn from(value: SoundGroup) -> Self {
        value.inner
    }
}

impl SoundGroup {
    /// Sets the maximum number of playbacks to be audible at once in a sound group.
    ///
    /// If playing instances of sounds in this group equal or exceed number specified here, attepts to play more of the sounds with be met with [`FMOD_RESULT::FMOD_ERR_MAXAUDIBLE`] by default.
    /// Use SoundGroup::setMaxAudibleBehavior to change the way the sound playback behaves when too many sounds are playing.
    /// Muting, failing and stealing behaviors can be specified. See FMOD_SOUNDGROUP_BEHAVIOR.
    ///
    /// SoundGroup::getNumPlaying can be used to determine how many instances of the sounds in the SoundGroup are currently playing.
    pub fn set_max_audible(&self, max_audible: c_int) -> Result<()> {
        unsafe { FMOD_SoundGroup_SetMaxAudible(self.inner, max_audible).to_result() }
    }

    /// Retrieves the maximum number of playbacks to be audible at once in a sound group.
    pub fn get_max_audible(&self) -> Result<c_int> {
        let mut max_audible = 0;
        unsafe { FMOD_SoundGroup_GetMaxAudible(self.inner, &mut max_audible).to_result()? };
        Ok(max_audible)
    }

    /// This function changes the way the sound playback behaves when too many sounds are playing in a soundgroup.
    pub fn set_max_audible_behavior(&self, behavior: SoundGroupBehavior) -> Result<()> {
        unsafe { FMOD_SoundGroup_SetMaxAudibleBehavior(self.inner, behavior.into()).to_result() }
    }

    /// Retrieves the current max audible behavior.
    pub fn get_max_audible_behavior(&self) -> Result<SoundGroupBehavior> {
        let mut behavior = 0;
        unsafe { FMOD_SoundGroup_GetMaxAudibleBehavior(self.inner, &mut behavior).to_result()? };
        let behavior = behavior.try_into()?;
        Ok(behavior)
    }

    /// Sets a mute fade time.
    ///
    /// If a mode besides [`SoundGroupBehavior::Mute`] is used, the fade speed is ignored.
    ///
    /// When more sounds are playing in a [`SoundGroup`] than are specified with [`SoundGroup::set_max_audible`],
    /// the least important Sound (ie lowest priority / lowest audible volume due to 3D position, volume etc)
    /// will fade to silence if [`SoundGroupBehavior::Mute`] is used,
    /// and any previous sounds that were silent because of this rule will fade in if they are more important.
    pub fn set_mute_fade_speed(&self, speed: c_float) -> Result<()> {
        unsafe { FMOD_SoundGroup_SetMuteFadeSpeed(self.inner, speed).to_result() }
    }

    /// Retrieves the current mute fade time.
    pub fn get_mute_fade_speed(&self) -> Result<c_float> {
        let mut speed = 0.0;
        unsafe { FMOD_SoundGroup_GetMuteFadeSpeed(self.inner, &mut speed).to_result()? };
        Ok(speed)
    }

    /// Sets the volume of the sound group.
    pub fn set_volume(&self, volume: c_float) -> Result<()> {
        unsafe { FMOD_SoundGroup_SetVolume(self.inner, volume).to_result() }
    }

    /// Retrieves the volume of the sound group.
    pub fn get_volume(&self) -> Result<c_float> {
        let mut volume = 0.0;
        unsafe { FMOD_SoundGroup_GetVolume(self.inner, &mut volume).to_result()? };
        Ok(volume)
    }

    /// Retrieves the current number of sounds in this sound group.
    pub fn get_sound_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_SoundGroup_GetNumSounds(self.inner, &mut count).to_result()? };
        Ok(count)
    }

    /// Retrieves a sound.
    ///
    /// Use [`SoundGroup::get_sound_count`] in conjunction with this function to enumerate all sounds in a [`SoundGroup`].
    pub fn get_sound(&self, index: c_int) -> Result<Sound> {
        let mut sound = std::ptr::null_mut();
        unsafe { FMOD_SoundGroup_GetSound(self.inner, index, &mut sound).to_result()? };
        Ok(sound.into())
    }

    /// Retrieves the number of currently playing [`Channel`]s for the [`SoundGroup`].
    ///
    /// This routine returns the number of [`Channel`]s playing.
    /// If the [`SoundGroup`] only has one [`Sound`], and that [`Sound`] is playing twice, the figure returned will be two.
    pub fn get_playing_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_SoundGroup_GetNumPlaying(self.inner, &mut count).to_result()? };
        Ok(count)
    }

    /// Stops all sounds within this soundgroup.
    pub fn stop(&self) -> Result<()> {
        unsafe { FMOD_SoundGroup_Stop(self.inner).to_result() }
    }

    /// Retrieves the name of the sound group.
    pub fn get_name(&self) -> Result<Utf8CString> {
        let mut name = [0_i8; 512];
        unsafe {
            FMOD_SoundGroup_GetName(self.inner, name.as_mut_ptr(), name.len() as c_int)
                .to_result()?;

            // FIXME is this right?
            let name = name
                .into_iter()
                .take_while(|&v| v != 0)
                .map(|v| v as u8)
                .collect();
            let name = Utf8CString::from_utf8_with_nul_unchecked(name);
            Ok(name)
        }
    }

    /// Releases a soundgroup object and returns all sounds back to the master sound group.
    ///
    /// You cannot release the master [`SoundGroup`].
    pub fn release(&self) -> Result<()> {
        unsafe { FMOD_SoundGroup_Release(self.inner).to_result() }
    }

    // TODO userdata

    /// Retrieves the parent System object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_SoundGroup_GetSystemObject(self.inner, &mut system).to_result()? };
        Ok(system.into())
    }
}
