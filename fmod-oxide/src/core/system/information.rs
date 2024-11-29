// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::{
    ffi::{c_int, c_longlong, c_uint},
    mem::MaybeUninit,
    os::raw::c_void,
};

use crate::{CpuUsage, SpeakerMode, System};

#[cfg(doc)]
use crate::OutputType;

impl System {
    /// Retrieves the FMOD version number.
    ///
    /// The version is a 32 bit hexadecimal value formatted as 16:8:8, with the upper 16 bits being the product version,
    /// the middle 8 bits being the major version and the bottom 8 bits being the minor version.
    /// For example a value of 0x00010203 is equal to 1.02.03.
    ///
    /// Compare against [`crate::VERSION`] to make sure crate and runtime library versions match.
    pub fn get_version(&self) -> Result<c_uint> {
        let mut version = 0;
        unsafe {
            FMOD_System_GetVersion(self.inner.as_ptr(), &mut version).to_result()?;
        }
        Ok(version)
    }

    /// Retrieves an output type specific internal native interface.
    ///
    /// Reinterpret the returned handle based on the selected output type, not all types return something.
    ///   [`OutputType::WavWriter`] Pointer to stdio FILE is returned
    ///   [`OutputType::WavWriterNRT`] Pointer to stdio FILE is returned
    ///   [`OutputType::WASAPI`] Pointer to type `IAudioClient` is returned.
    ///   [`OutputType::Alsa`] Pointer to type `snd_pcm_t` is returned.
    ///   [`OutputType::CoreAudio`] Handle of type `AudioUnit` is returned.
    ///   [`OutputType::AudioOut`] Pointer to type int is returned. Handle returned from sceAudioOutOpen.
    ///
    ///
    /// NOTE: Calling this function is safe, but doing anything with the returned pointer is not!!
    pub fn get_output_handle(&self) -> Result<*mut c_void> {
        let mut handle = std::ptr::null_mut();
        unsafe {
            FMOD_System_GetOutputHandle(self.inner.as_ptr(), &mut handle).to_result()?;
        }
        Ok(handle)
    }

    /// Retrieves the number of currently playing Channels.
    ///
    /// For differences between real and virtual voices see the Virtual Voices guide for more information.
    pub fn get_playing_channels(&self) -> Result<(c_int, c_int)> {
        let mut channels = 0;
        let mut real_channels = 0;
        unsafe {
            FMOD_System_GetChannelsPlaying(self.inner.as_ptr(), &mut channels, &mut real_channels)
                .to_result()?;
        }
        Ok((channels, real_channels))
    }

    /// Retrieves the amount of CPU used for different parts of the Core engine.
    ///
    /// For readability, the percentage values are smoothed to provide a more stable output.
    pub fn get_cpu_usage(&self) -> Result<CpuUsage> {
        let mut cpu_usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_System_GetCPUUsage(self.inner.as_ptr(), cpu_usage.as_mut_ptr()).to_result()?;
            let cpu_usage = cpu_usage.assume_init().into();
            Ok(cpu_usage)
        }
    }

    /// Retrieves information about file reads.
    ///
    /// The values returned are running totals that never reset.
    pub fn get_file_usage(&self) -> Result<(c_longlong, c_longlong, c_longlong)> {
        let mut sample_read = 0;
        let mut stream_read = 0;
        let mut other_read = 0;
        unsafe {
            FMOD_System_GetFileUsage(
                self.inner.as_ptr(),
                &mut sample_read,
                &mut stream_read,
                &mut other_read,
            )
            .to_result()?;
        }
        Ok((sample_read, stream_read, other_read))
    }

    /// Retrieves the default matrix used to convert from one speaker mode to another.
    ///
    /// The gain for source channel 's' to target channel 't' is `matrix[t * <number of source channels> + s]`.
    ///
    /// If '`source_mode`' or '`target_mode`' is [`SpeakerMode::Raw`], this function will return [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`].
    /// The number of source channels can be found from [`System::get_speaker_mode_channels`].
    // FIXME: do we take an out slice param?
    pub fn get_default_mix_matrix(
        &self,
        source_mode: SpeakerMode,
        target_mode: SpeakerMode,
    ) -> Result<Vec<f32>> {
        let source_channels = self.get_speaker_mode_channels(source_mode)?;
        let target_channels = self.get_speaker_mode_channels(target_mode)?;
        debug_assert!(source_channels <= FMOD_MAX_CHANNEL_WIDTH as c_int);
        debug_assert!(target_channels <= FMOD_MAX_CHANNEL_WIDTH as c_int);
        let mut matrix = vec![0.0; source_channels as usize * target_channels as usize];

        unsafe {
            FMOD_System_GetDefaultMixMatrix(
                self.inner.as_ptr(),
                source_mode.into(),
                target_mode.into(),
                matrix.as_mut_ptr(),
                source_channels,
            )
            .to_result()?;
        }
        Ok(matrix)
    }

    /// Retrieves the channel count for a given speaker mode.
    pub fn get_speaker_mode_channels(&self, speaker_mode: SpeakerMode) -> Result<c_int> {
        let mut channels = 0;
        unsafe {
            FMOD_System_GetSpeakerModeChannels(
                self.inner.as_ptr(),
                speaker_mode.into(),
                &mut channels,
            )
            .to_result()?;
        }
        Ok(channels)
    }
}
