// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_int;

use crate::{Dsp, DspConnection, DspConnectionType};

impl Dsp {
    /// Adds a [`Dsp`] unit as an input to this object.
    ///
    /// When a [`Dsp`] has multiple inputs the signals are automatically mixed together, sent to the unit's output(s).
    ///
    /// The returned [`DspConnection`] will remain valid until the units are disconnected.
    pub fn add_input(&self, input: Dsp, kind: DspConnectionType) -> Result<DspConnection> {
        let mut connection = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_AddInput(self.inner, input.inner, &mut connection, kind.into()).to_result()?;
        };
        Ok(connection.into())
    }

    /// Retrieves the [`Dsp`] unit at the specified index in the input list.
    ///
    /// This will flush the [`Dsp`] queue (which blocks against the mixer) to ensure the input list is correct, avoid this during time sensitive operations.
    ///
    /// The returned [`DspConnection`] will remain valid until the units are disconnected.
    pub fn get_input(&self, index: c_int) -> Result<(Dsp, DspConnection)> {
        let mut connection = std::ptr::null_mut();
        let mut dsp = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetInput(self.inner, index, &mut dsp, &mut connection).to_result()?;
        };
        Ok((dsp.into(), connection.into()))
    }

    /// Retrieves the [`Dsp`] unit at the specified index in the output list.
    ///
    /// This will flush the [`Dsp`] queue (which blocks against the mixer) to ensure the output list is correct, avoid this during time sensitive operations.
    ///
    /// The returned [`DspConnection`] will remain valid until the units are disconnected.
    pub fn get_output(&self, index: c_int) -> Result<(Dsp, DspConnection)> {
        let mut connection = std::ptr::null_mut();
        let mut dsp = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetOutput(self.inner, index, &mut dsp, &mut connection).to_result()?;
        };
        Ok((dsp.into(), connection.into()))
    }

    /// Retrieves the number of [`Dsp`] units in the input list.
    ///
    /// This will flush the [`Dsp`] queue (which blocks against the mixer) to ensure the input list is correct, avoid this during time sensitive operations.
    pub fn get_input_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_DSP_GetNumInputs(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves the number of [`Dsp`] units in the output list.
    ///
    /// This will flush the [`Dsp`] queue (which blocks against the mixer) to ensure the output list is correct, avoid this during time sensitive operations.
    pub fn get_output_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_DSP_GetNumOutputs(self.inner, &mut count).to_result()? };
        Ok(count)
    }

    /// Disconnects all inputs and/or outputs.
    ///
    /// This is a convenience function that is faster than disconnecting all inputs and outputs individually.
    pub fn disconnect_all(&self, inputs: bool, outputs: bool) -> Result<()> {
        unsafe { FMOD_DSP_DisconnectAll(self.inner, inputs.into(), outputs.into()).to_result() }
    }

    /// Disconnect the specified input [`Dsp`].
    ///
    /// If target had only one output, after this operation that entire sub graph will no longer be connected to the [`Dsp`] network.
    ///
    /// After this operation `connection` is no longer valid.
    pub fn disconnect_from(
        &self,
        target: Option<Dsp>,
        connection: Option<DspConnection>,
    ) -> Result<()> {
        let target = target.map_or(std::ptr::null_mut(), Into::into);
        let connection = connection.map_or(std::ptr::null_mut(), Into::into);
        unsafe { FMOD_DSP_DisconnectFrom(self.inner, target, connection).to_result() }
    }
}
