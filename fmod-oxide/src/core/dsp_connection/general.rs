// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_void;

use fmod_sys::*;

use crate::{Dsp, DspConnection, DspConnectionType};

impl DspConnection {
    /// Retrieves the connection's input [`Dsp`] unit.
    ///
    /// If [`Dsp::add_input`] was just called, the connection might not be ready because the [`Dsp`] system is still queued to be connected,
    /// and may need to wait several milliseconds for the next mix to occur.
    /// If so the function will return [`FMOD_RESULT::FMOD_ERR_NOTREADY`].
    pub fn get_input(&self) -> Result<Dsp> {
        let mut dsp = std::ptr::null_mut();
        unsafe { FMOD_DSPConnection_GetInput(self.inner.as_ptr(), &mut dsp).to_result()? };
        Ok(dsp.into())
    }

    /// Retrieves the connection's output DSP unit.
    ///
    /// If [`Dsp::add_input`] was just called, the connection might not be ready because the [`Dsp`] system is still queued to be connected,
    /// and may need to wait several milliseconds for the next mix to occur.
    /// If so the function will return [`FMOD_RESULT::FMOD_ERR_NOTREADY`].
    pub fn get_output(&self) -> Result<Dsp> {
        let mut dsp = std::ptr::null_mut();
        unsafe { FMOD_DSPConnection_GetOutput(self.inner.as_ptr(), &mut dsp).to_result()? };
        Ok(dsp.into())
    }

    /// Retrieves the type of the connection between 2 DSP units.
    pub fn get_type(&self) -> Result<DspConnectionType> {
        let mut connection_type = 0;
        unsafe {
            FMOD_DSPConnection_GetType(self.inner.as_ptr(), &mut connection_type).to_result()?;
        };
        let connection_type = connection_type.try_into()?;
        Ok(connection_type)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_DSPConnection_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_DSPConnection_GetUserData(self.inner.as_ptr(), &mut userdata).to_result()?;
        }
        Ok(userdata)
    }
}
