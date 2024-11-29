// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_int;

use crate::{Sound, SoundGroup};

#[cfg(doc)]
use crate::Channel;

impl SoundGroup {
    /// Retrieves the current number of sounds in this sound group.
    pub fn get_sound_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_SoundGroup_GetNumSounds(self.inner.as_ptr(), &mut count).to_result()? };
        Ok(count)
    }

    /// Retrieves a sound.
    ///
    /// Use [`SoundGroup::get_sound_count`] in conjunction with this function to enumerate all sounds in a [`SoundGroup`].
    pub fn get_sound(&self, index: c_int) -> Result<Sound> {
        let mut sound = std::ptr::null_mut();
        unsafe { FMOD_SoundGroup_GetSound(self.inner.as_ptr(), index, &mut sound).to_result()? };
        Ok(sound.into())
    }

    /// Retrieves the number of currently playing [`Channel`]s for the [`SoundGroup`].
    ///
    /// This routine returns the number of [`Channel`]s playing.
    /// If the [`SoundGroup`] only has one [`Sound`], and that [`Sound`] is playing twice, the figure returned will be two.
    pub fn get_playing_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_SoundGroup_GetNumPlaying(self.inner.as_ptr(), &mut count).to_result()? };
        Ok(count)
    }

    /// Stops all sounds within this soundgroup.
    pub fn stop(&self) -> Result<()> {
        unsafe { FMOD_SoundGroup_Stop(self.inner.as_ptr()).to_result() }
    }
}
