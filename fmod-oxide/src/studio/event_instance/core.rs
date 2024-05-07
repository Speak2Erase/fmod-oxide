// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_int, c_uint},
    mem::MaybeUninit,
};

use fmod_sys::*;
use lanyard::Utf8CStr;

use crate::studio::{EventInstance, MemoryUsage, ParameterID};
use crate::ChannelGroup;

impl EventInstance {
    /// Retrieves the core [`ChannelGroup`].
    ///
    /// Until the event instance has been fully created this function will return [`FMOD_RESULT::FMOD_ERR_STUDIO_NOT_LOADED`].
    pub fn get_channel_group(&self) -> Result<ChannelGroup> {
        let mut channel_group = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetChannelGroup(self.inner, &mut channel_group)
                .to_result()?;
        }
        Ok(channel_group.into())
    }

    /// Sets the core reverb send level.
    ///          
    /// This function controls the send level for the signal from the event instance to a core reverb instance.
    pub fn set_reverb_level(&self, index: c_int, level: c_float) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetReverbLevel(self.inner, index, level).to_result() }
    }

    /// Retrieves the core reverb send level.
    pub fn get_reverb_level(&self, index: c_int) -> Result<c_float> {
        let mut level = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetReverbLevel(self.inner, index, &mut level).to_result()?;
        }
        Ok(level)
    }
}
