// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_float;

use fmod_sys::*;

use crate::{Dsp, DspConnectionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct DspConnection {
    pub(crate) inner: *mut FMOD_DSPCONNECTION,
}

unsafe impl Send for DspConnection {}
unsafe impl Sync for DspConnection {}

impl From<*mut FMOD_DSPCONNECTION> for DspConnection {
    fn from(value: *mut FMOD_DSPCONNECTION) -> Self {
        DspConnection { inner: value }
    }
}

impl From<DspConnection> for *mut FMOD_DSPCONNECTION {
    fn from(value: DspConnection) -> Self {
        value.inner
    }
}

impl DspConnection {
    /// Sets the connection's volume scale.
    pub fn set_mix(&self, volume: c_float) -> Result<()> {
        unsafe { FMOD_DSPConnection_SetMix(self.inner, volume).to_result() }
    }

    /// Retrieves the connection's volume scale.
    pub fn get_mix(&self) -> Result<c_float> {
        let mut volume = 0.0;
        unsafe { FMOD_DSPConnection_GetMix(self.inner, &mut volume).to_result()? };
        Ok(volume)
    }

    // TODO mix matrix

    /// Retrieves the connection's input [`Dsp`] unit.
    ///
    /// If DSP::addInput was just called, the connection might not be ready because the [`Dsp`] system is still queued to be connected,
    /// and may need to wait several milliseconds for the next mix to occur.
    /// If so the function will return [`FMOD_RESULT::FMOD_ERR_NOTREADY`].
    pub fn get_input(&self) -> Result<Dsp> {
        let mut dsp = std::ptr::null_mut();
        unsafe { FMOD_DSPConnection_GetInput(self.inner, &mut dsp).to_result()? };
        Ok(dsp.into())
    }

    /// Retrieves the connection's output DSP unit.
    ///
    /// If DSP::addInput was just called, the connection might not be ready because the [`Dsp`] system is still queued to be connected,
    /// and may need to wait several milliseconds for the next mix to occur.
    /// If so the function will return [`FMOD_RESULT::FMOD_ERR_NOTREADY`].
    pub fn get_output(&self) -> Result<Dsp> {
        let mut dsp = std::ptr::null_mut();
        unsafe { FMOD_DSPConnection_GetOutput(self.inner, &mut dsp).to_result()? };
        Ok(dsp.into())
    }

    /// Retrieves the type of the connection between 2 DSP units.
    pub fn get_type(&self) -> Result<DspConnectionType> {
        let mut connection_type = 0;
        unsafe { FMOD_DSPConnection_GetType(self.inner, &mut connection_type).to_result()? };
        let connection_type = connection_type.try_into()?;
        Ok(connection_type)
    }

    // TODO userdata
}
