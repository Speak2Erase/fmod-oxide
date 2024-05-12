// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_float, c_int};

use fmod_sys::*;

use crate::ChannelControl;

impl ChannelControl {
    /// Sets the wet / send level for a particular reverb instance.
    ///
    /// Channels are automatically connected to all existing reverb instances due to the default wet level of 1.
    /// ChannelGroups however will not send to any reverb by default requiring an explicit call to this function.
    ///
    /// ChannelGroup reverb is optimal for the case where you want to send 1 mixed signal to the reverb, rather than a lot of individual Channel reverb sends.
    /// It is advisable to do this to reduce CPU if you have many Channels inside a ChannelGroup.
    ///
    /// When setting a wet level for a ChannelGroup, any Channels under that ChannelGroup will still have their existing sends to the reverb.
    /// To avoid this doubling up you should explicitly set the Channel wet levels to 0.
    pub fn set_reverb_properties(&self, instance: c_int, wet: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetReverbProperties(self.inner, instance, wet).to_result() }
    }

    /// Retrieves the wet / send level for a particular reverb instance.
    pub fn get_reverb_properties(&self, instance: c_int) -> Result<c_float> {
        let mut wet = 0.0;
        unsafe {
            FMOD_ChannelControl_GetReverbProperties(self.inner, instance, &mut wet).to_result()?;
        }
        Ok(wet)
    }

    /// Sets the gain of the dry signal when built in lowpass / distance filtering is applied.
    ///
    /// Requires the built in lowpass to be created with FMOD_INIT_CHANNEL_LOWPASS or FMOD_INIT_CHANNEL_DISTANCEFILTER.
    ///
    /// #### NOTE: Currently only supported for Channel, not ChannelGroup.
    pub fn set_low_pass_gain(&self, gain: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetLowPassGain(self.inner, gain).to_result() }
    }

    /// Retrieves the gain of the dry signal when built in lowpass / distance filtering is applied.
    ///
    /// Requires the built in lowpass to be created with FMOD_INIT_CHANNEL_LOWPASS or FMOD_INIT_CHANNEL_DISTANCEFILTER.
    ///
    /// #### NOTE: Currently only supported for Channel, not ChannelGroup.
    pub fn get_low_pass_gain(&self) -> Result<c_float> {
        let mut gain = 0.0;
        unsafe {
            FMOD_ChannelControl_GetLowPassGain(self.inner, &mut gain).to_result()?;
        }
        Ok(gain)
    }
}
