// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_int;

use crate::{ChannelMask, Dsp, SpeakerMode};

impl Dsp {
    /// Sets the PCM input format this [`Dsp`] will receive when processing.
    ///
    /// Setting the number of channels on a unit will force either a down or up mix to that channel count before processing the [`Dsp`] read/process callback.
    pub fn set_channel_format(
        &self,
        channel_mask: ChannelMask,
        channel_count: c_int,
        source_speaker_mode: SpeakerMode,
    ) -> Result<()> {
        unsafe {
            FMOD_DSP_SetChannelFormat(
                self.inner.as_ptr(),
                channel_mask.into(),
                channel_count,
                source_speaker_mode.into(),
            )
            .to_result()
        }
    }

    /// Retrieves the PCM input format this [`Dsp`] will receive when processing.
    pub fn get_channel_format(&self) -> Result<(ChannelMask, c_int, SpeakerMode)> {
        let mut channel_mask = 0;
        let mut channel_count = 0;
        let mut source_speaker_mode = 0;
        unsafe {
            FMOD_DSP_GetChannelFormat(
                self.inner.as_ptr(),
                &mut channel_mask,
                &mut channel_count,
                &mut source_speaker_mode,
            )
            .to_result()?;
        }
        let source_speaker_mode = source_speaker_mode.try_into()?;
        Ok((channel_mask.into(), channel_count, source_speaker_mode))
    }

    /// Retrieves the output format this [`Dsp`] will produce when processing based on the input specified.
    pub fn get_output_channel_format(
        &self,
        in_mask: ChannelMask,
        in_channels: c_int,
        in_speaker_mode: SpeakerMode,
    ) -> Result<(ChannelMask, c_int, SpeakerMode)> {
        let mut out_mask = 0;
        let mut out_channels = 0;
        let mut out_speaker_mode = 0;
        unsafe {
            FMOD_DSP_GetOutputChannelFormat(
                self.inner.as_ptr(),
                in_mask.into(),
                in_channels,
                in_speaker_mode.into(),
                &mut out_mask,
                &mut out_channels,
                &mut out_speaker_mode,
            )
            .to_result()?;
        }
        let out_speaker_mode = out_speaker_mode.try_into()?;
        Ok((out_mask.into(), out_channels, out_speaker_mode))
    }
}
