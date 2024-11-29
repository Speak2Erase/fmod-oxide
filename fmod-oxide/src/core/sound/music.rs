// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_float, c_int};

use fmod_sys::*;

use crate::Sound;

impl Sound {
    /// Gets the number of music channels inside a MOD/S3M/XM/IT/MIDI file.
    pub fn get_music_channel_count(&self) -> Result<i32> {
        let mut num_channels = 0;
        unsafe {
            FMOD_Sound_GetMusicNumChannels(self.inner.as_ptr(), &mut num_channels).to_result()?;
        }
        Ok(num_channels)
    }

    /// Sets the volume of a MOD/S3M/XM/IT/MIDI music channel volume.
    pub fn set_music_channel_volume(&self, channel: c_int, volume: c_float) -> Result<()> {
        unsafe {
            FMOD_Sound_SetMusicChannelVolume(self.inner.as_ptr(), channel, volume).to_result()?;
        }
        Ok(())
    }

    /// Retrieves the volume of a MOD/S3M/XM/IT/MIDI music channel volume.
    pub fn get_music_channel_volume(&self, channel: c_int) -> Result<c_float> {
        let mut volume = 0.0;
        unsafe {
            FMOD_Sound_GetMusicChannelVolume(self.inner.as_ptr(), channel, &mut volume)
                .to_result()?;
        }
        Ok(volume)
    }

    /// Sets the relative speed of MOD/S3M/XM/IT/MIDI music.
    pub fn set_music_speed(&self, speed: c_float) -> Result<()> {
        unsafe {
            FMOD_Sound_SetMusicSpeed(self.inner.as_ptr(), speed).to_result()?;
        }
        Ok(())
    }

    /// Gets the relative speed of MOD/S3M/XM/IT/MIDI music.
    pub fn get_music_speed(&self) -> Result<c_float> {
        let mut speed = 0.0;
        unsafe {
            FMOD_Sound_GetMusicSpeed(self.inner.as_ptr(), &mut speed).to_result()?;
        }
        Ok(speed)
    }
}
