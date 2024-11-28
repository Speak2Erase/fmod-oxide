// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_float;

use fmod_sys::*;

use crate::ChannelControl;

impl ChannelControl {
    /// Sets the left/right pan level.
    ///
    /// This is a convenience function to avoid passing a matrix, it will overwrite values set via `ChannelControl::setMixLevelsInput`,
    /// `ChannelControl::setMixLevelsOutput` and `ChannelControl::setMixMatrix`.
    ///
    /// Mono inputs are panned from left to right using constant power panning (non linear fade).
    ///  Stereo and greater inputs will isolate the front left and right input channels and fade them up and down based on the pan value (silencing other channels).
    /// The output channel count will always match the System speaker mode set via `System::setSoftwareFormat`.
    ///
    /// If the System is initialized with `FMOD_SPEAKERMODE_RAW` calling this function will produce silence.
    pub fn set_pan(&self, pan: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetPan(self.inner, pan).to_result() }
    }

    /// Sets the incoming volume level for each channel of a multi-channel signal.
    ///
    /// This is a convenience function to avoid passing a matrix, it will overwrite values set via `ChannelControl::setPan`,
    /// `ChannelControl::setMixLevelsOutput` and `ChannelControl::setMixMatrix`.
    ///
    /// #### NOTE: Currently only supported for Channel, not `ChannelGroup`.
    pub fn set_mix_levels_input(&self, levels: &mut [c_float]) -> Result<()> {
        // probably shouldn't be mutable but it's more safe that way?
        // FIXME do we need to enforce a max length?
        unsafe {
            FMOD_ChannelControl_SetMixLevelsInput(
                self.inner,
                levels.as_mut_ptr(),
                levels.len() as i32,
            )
            .to_result()
        }
    }

    /// Sets the outgoing volume levels for each speaker.
    ///
    /// Specify the level for a given output speaker, if the channel count of the input and output do not match,
    /// channels will be up/down mixed as appropriate to approximate the given speaker values.
    /// For example stereo input with 5.1 output will use the center parameter to distribute signal to the center speaker from front left and front right channels.
    ///
    /// This is a convenience function to avoid passing a matrix, it will overwrite values set via `ChannelControl::setPan`, `ChannelControl::setMixLevelsInput` and `ChannelControl::setMixMatrix`.
    ///
    /// The output channel count will always match the System speaker mode set via `System::setSoftwareFormat`.
    ///
    /// If the System is initialized with `FMOD_SPEAKERMODE_RAW` calling this function will produce silence.
    #[allow(clippy::too_many_arguments)] // no fixing this
    pub fn set_mix_levels_output(
        &self,
        front_left: c_float,
        front_right: c_float,
        center: c_float,
        lfe: c_float,
        surround_left: c_float,
        surround_right: c_float,
        back_left: c_float,
        back_right: c_float,
    ) -> Result<()> {
        unsafe {
            FMOD_ChannelControl_SetMixLevelsOutput(
                self.inner,
                front_left,
                front_right,
                center,
                lfe,
                surround_left,
                surround_right,
                back_left,
                back_right,
            )
            .to_result()
        }
    }

    // TODO: mix matrix
}
