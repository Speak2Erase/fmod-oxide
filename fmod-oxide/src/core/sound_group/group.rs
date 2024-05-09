// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int};

use crate::{SoundGroup, SoundGroupBehavior};

impl SoundGroup {
    /// Sets the maximum number of playbacks to be audible at once in a sound group.
    ///
    /// If playing instances of sounds in this group equal or exceed number specified here, attepts to play more of the sounds with be met with [`FMOD_RESULT::FMOD_ERR_MAXAUDIBLE`] by default.
    /// Use [`SoundGroup::set_max_audible_behavior`] to change the way the sound playback behaves when too many sounds are playing.
    /// Muting, failing and stealing behaviors can be specified. See [`SoundGroupBehavior`].
    ///
    /// [`SoundGroup::get_playing_count`] can be used to determine how many instances of the sounds in the [`SoundGroup`] are currently playing.
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
}
