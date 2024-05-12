// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_int;

use fmod_sys::*;

use crate::{ChannelControl, Dsp};

impl ChannelControl {
    pub const DSP_HEAD: FMOD_CHANNELCONTROL_DSP_INDEX = FMOD_CHANNELCONTROL_DSP_HEAD;
    pub const DSP_TAIL: FMOD_CHANNELCONTROL_DSP_INDEX = FMOD_CHANNELCONTROL_DSP_TAIL;
    pub const DSP_FADER: FMOD_CHANNELCONTROL_DSP_INDEX = FMOD_CHANNELCONTROL_DSP_FADER;

    /// Adds a DSP unit to the specified index in the DSP chain.
    ///
    /// If dsp is already added to an existing object it will be removed and then added to this object.
    ///
    /// For detailed information on FMOD's DSP network, read the DSP Architecture and Usage white paper.
    pub fn add_dsp(&self, index: c_int, dsp: Dsp) -> Result<()> {
        unsafe { FMOD_ChannelControl_AddDSP(self.inner, index, dsp.inner).to_result() }
    }

    /// Removes the specified DSP unit from the DSP chain.
    pub fn remove_dsp(&self, dsp: Dsp) -> Result<()> {
        unsafe { FMOD_ChannelControl_RemoveDSP(self.inner, dsp.inner).to_result() }
    }

    /// Retrieves the number of DSP units in the DSP chain.
    pub fn get_dsp_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_ChannelControl_GetNumDSPs(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves the DSP unit at the specified index in the DSP chain.
    pub fn get_dsp(&self, index: c_int) -> Result<Dsp> {
        let mut dsp = std::ptr::null_mut();
        unsafe {
            FMOD_ChannelControl_GetDSP(self.inner, index, &mut dsp).to_result()?;
        }
        Ok(Dsp { inner: dsp })
    }

    /// Sets the index in the DSP chain of the specified DSP.
    ///
    /// This will move a DSP already in the DSP chain to a new offset.
    pub fn set_dsp_index(&self, dsp: Dsp, index: c_int) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetDSPIndex(self.inner, dsp.inner, index).to_result() }
    }

    /// Retrieves the index of a DSP inside the Channel or ChannelGroup's DSP chain.
    pub fn get_dsp_index(&self, dsp: Dsp) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_ChannelControl_GetDSPIndex(self.inner, dsp.inner, &mut index).to_result()?;
        }
        Ok(index)
    }
}
