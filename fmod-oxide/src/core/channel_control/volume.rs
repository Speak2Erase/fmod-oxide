// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_float;

use crate::ChannelControl;

#[cfg(doc)]
use crate::Channel;

impl ChannelControl {
    /// Retrieves an estimation of the output volume.
    ///
    /// Estimated volume is calculated based on 3D spatialization, occlusion, API volume levels and DSPs used.
    ///
    /// While this does not represent the actual waveform, [`Channel`]s playing FSB files will take into consideration the overall peak level of the file (if available).
    ///
    /// This value is used to determine which [`Channel`]s should be audible and which [`Channel`]s to virtualize when resources are limited.
    ///
    /// See the Virtual Voice System white paper for more details about how audibility is calculated.
    pub fn get_audibility(&self) -> Result<c_float> {
        let mut audibility = 0.0;
        unsafe {
            FMOD_ChannelControl_GetAudibility(self.inner.as_ptr(), &mut audibility).to_result()?;
        }
        Ok(audibility)
    }

    /// Sets the volume level.
    ///
    /// To define the volume per Sound use `Sound::setDefaults`.
    ///
    /// Setting volume at a level higher than 1 can lead to distortion/clipping.
    pub fn set_volume(&self, volume: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetVolume(self.inner.as_ptr(), volume).to_result() }
    }

    /// Retrieves the volume level.
    pub fn get_volume(&self) -> Result<c_float> {
        let mut volume = 0.0;
        unsafe {
            FMOD_ChannelControl_GetVolume(self.inner.as_ptr(), &mut volume).to_result()?;
        }
        Ok(volume)
    }

    /// Sets whether volume changes are ramped or instantaneous.
    ///
    /// Volume changes when not paused will be ramped to the target value to avoid a pop sound,
    /// this function allows that setting to be overridden and volume changes to be applied immediately.
    pub fn set_volume_ramp(&self, ramp: bool) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetVolumeRamp(self.inner.as_ptr(), ramp).to_result() }
    }

    /// Retrieves whether volume changes are ramped or instantaneous.
    pub fn get_volume_ramp(&self) -> Result<bool> {
        let mut ramp = false;
        unsafe {
            FMOD_ChannelControl_GetVolumeRamp(self.inner.as_ptr(), &mut ramp).to_result()?;
        }
        Ok(ramp)
    }

    /// Sets the mute state.
    ///
    /// Mute is an additional control for volume, the effect of which is equivalent to setting the volume to zero.
    ///
    /// An individual mute state is kept for each object,
    /// muting a parent `ChannelGroup` will effectively mute this object however when queried the individual mute state is returned.
    /// `ChannelControl::getAudibility` can be used to calculate overall audibility for a Channel or `ChannelGroup`.
    pub fn set_mute(&self, mute: bool) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetMute(self.inner.as_ptr(), mute).to_result() }
    }

    /// Retrieves the mute state.
    ///
    /// An individual mute state is kept for each object,
    /// muting a parent `ChannelGroup` will effectively mute this object however when queried the individual mute state is returned.
    /// `ChannelControl::getAudibility` can be used to calculate overall audibility for a Channel or `ChannelGroup`.
    pub fn get_mute(&self) -> Result<bool> {
        let mut mute = false;
        unsafe {
            FMOD_ChannelControl_GetMute(self.inner.as_ptr(), &mut mute).to_result()?;
        }
        Ok(mute)
    }
}
