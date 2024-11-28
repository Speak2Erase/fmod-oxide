// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int, c_uint};

use crate::{Dsp, DspParameterDataType, DspParameterDescription};

impl Dsp {
    /// Retrieve the index of the first data parameter of a particular data type.
    ///
    /// This function returns [`Ok`] if a parmeter of matching type is found and [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`] if no matches were found.
    ///
    /// The return code can be used to check whether the [`Dsp`] supports specific functionality through data parameters of certain types without the need to provide index.
    pub fn get_data_parameter_index(&self, data_type: DspParameterDataType) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_DSP_GetDataParameterIndex(self.inner, data_type.into(), &mut index).to_result()?;
        }
        Ok(index)
    }

    /// Retrieves the number of parameters exposed by this unit.
    ///
    /// Use this to enumerate all parameters of a [`Dsp`] unit with [`Dsp::get_parameter_info`].
    pub fn get_parameter_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_DSP_GetNumParameters(self.inner, &mut count).to_result()? };
        Ok(count)
    }

    /// Sets a boolean parameter by index.
    pub fn set_parameter_bool(&self, index: c_int, value: bool) -> Result<()> {
        unsafe { FMOD_DSP_SetParameterBool(self.inner, index, value.into()).to_result() }
    }

    /// Retrieves a boolean parameter by index.
    ///
    /// Note: FMOD also returns a string representation of the parameter value, but this is not currently exposed.
    /// You can just use [`ToString`] instead.
    // FIXME turn this into an enum sorta thing?
    pub fn get_parameter_bool(&self, index: c_int) -> Result<bool> {
        let mut value = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_DSP_GetParameterBool(self.inner, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
        }
        Ok(value.into())
    }

    /// Sets a binary data parameter by index.
    ///
    /// Certain data types are predefined by the system and can be specified via the `FMOD_DSP_PARAMETER_DESC_DATA`, see [`DspParameterDataType`]
    ///
    /// # Safety
    ///
    /// You must ensure that the data type passed in via `data` matches the data type expected by the [`Dsp`] unit.
    // FIXME: does FMOD copy the data?
    // FIXME: fmod has various predefined data types, should we expose them?
    pub unsafe fn set_parameter_data(&self, index: c_int, data: &[u8]) -> Result<()> {
        unsafe {
            FMOD_DSP_SetParameterData(
                self.inner,
                index,
                data.as_ptr() as *mut _,
                data.len() as c_uint,
            )
            .to_result()
        }
    }

    /// Retrieves a binary data parameter by index.
    ///
    /// Note: FMOD also returns a string representation of the parameter value, but this is not currently exposed.
    // FIXME is this safe???
    pub fn get_parameter_data(&self, index: c_int) -> Result<Vec<u8>> {
        let mut value = std::ptr::null_mut();
        let mut length = 0;
        unsafe {
            FMOD_DSP_GetParameterData(
                self.inner,
                index,
                &mut value,
                &mut length,
                std::ptr::null_mut(),
                0,
            )
            .to_result()?;

            let slice = std::slice::from_raw_parts_mut(value.cast(), length as usize);
            Ok(slice.to_vec())
        }
    }

    /// Sets a floating point parameter by index.
    pub fn set_parameter_float(&self, index: c_int, value: c_float) -> Result<()> {
        unsafe { FMOD_DSP_SetParameterFloat(self.inner, index, value).to_result() }
    }

    /// Retrieves a floating point parameter by index.
    ///
    /// Note: FMOD also returns a string representation of the parameter value, but this is not currently exposed.
    /// You can just use [`ToString`] instead.
    pub fn get_parameter_float(&self, index: c_int) -> Result<c_float> {
        let mut value = 0.0;
        unsafe {
            FMOD_DSP_GetParameterFloat(self.inner, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
        }
        Ok(value)
    }

    /// Sets an integer parameter by index.
    pub fn set_parameter_int(&self, index: c_int, value: c_int) -> Result<()> {
        unsafe { FMOD_DSP_SetParameterInt(self.inner, index, value).to_result() }
    }

    /// Retrieves an integer parameter by index.
    ///
    /// Note: FMOD also returns a string representation of the parameter value, but this is not currently exposed.
    /// You can just use [`ToString`] instead.
    pub fn get_parameter_int(&self, index: c_int) -> Result<c_int> {
        let mut value = 0;
        unsafe {
            FMOD_DSP_GetParameterInt(self.inner, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
        }
        Ok(value)
    }

    /// Retrieve information about a specified parameter.
    pub fn get_parameter_info(&self, index: c_int) -> Result<DspParameterDescription> {
        let mut desc = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetParameterInfo(self.inner, index, &mut desc).to_result()?;
            let desc = DspParameterDescription::from_ffi(*desc); // oh god this is *awful*
            Ok(desc)
        }
    }
}
