// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::{c_float, c_int, c_uint};

use crate::{Dsp, DspParameterDataType, DspParameterDescription};

mod sealed {
    pub trait Seal {}
}
pub trait ParameterType: sealed::Seal + Sized {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()>;

    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self>;

    // TODO Strings are a max of FMOD_DSP_GETPARAM_VALUESTR_LENGTH so we don't need to heap allocate them
    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString>;
}

impl sealed::Seal for bool {}
impl ParameterType for bool {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let dsp = dsp.inner.as_ptr();
        unsafe { FMOD_DSP_SetParameterBool(dsp, index, self.into()).to_result() }
    }

    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let dsp = dsp.inner.as_ptr();
        unsafe {
            let mut value = FMOD_BOOL::FALSE;
            FMOD_DSP_GetParameterBool(dsp, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
            Ok(value.into())
        }
    }

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString> {
        let dsp = dsp.inner.as_ptr();
        let mut bytes = [0; FMOD_DSP_GETPARAM_VALUESTR_LENGTH as usize];
        unsafe {
            FMOD_DSP_GetParameterBool(
                dsp,
                index,
                std::ptr::null_mut(),
                bytes.as_mut_ptr().cast(),
                FMOD_DSP_GETPARAM_VALUESTR_LENGTH as i32,
            )
            .to_result()?;

            let string = Utf8CString::from_utf8_with_nul(bytes.to_vec()).unwrap();
            Ok(string)
        }
    }
}

impl sealed::Seal for c_int {}
impl ParameterType for c_int {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let dsp = dsp.inner.as_ptr();
        unsafe { FMOD_DSP_SetParameterInt(dsp, index, self).to_result() }
    }

    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let dsp = dsp.inner.as_ptr();
        unsafe {
            let mut value = 0;
            FMOD_DSP_GetParameterInt(dsp, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
            Ok(value)
        }
    }

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString> {
        let dsp = dsp.inner.as_ptr();
        let mut bytes = [0; FMOD_DSP_GETPARAM_VALUESTR_LENGTH as usize];
        unsafe {
            FMOD_DSP_GetParameterInt(
                dsp,
                index,
                std::ptr::null_mut(),
                bytes.as_mut_ptr().cast(),
                FMOD_DSP_GETPARAM_VALUESTR_LENGTH as i32,
            )
            .to_result()?;

            let string = Utf8CString::from_utf8_with_nul(bytes.to_vec()).unwrap();
            Ok(string)
        }
    }
}

impl sealed::Seal for c_float {}
impl ParameterType for c_float {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let dsp = dsp.inner.as_ptr();
        unsafe { FMOD_DSP_SetParameterFloat(dsp, index, self).to_result() }
    }

    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let dsp = dsp.inner.as_ptr();
        unsafe {
            let mut value = 0.0;
            FMOD_DSP_GetParameterFloat(dsp, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
            Ok(value)
        }
    }

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString> {
        let dsp = dsp.inner.as_ptr();
        let mut bytes = [0; FMOD_DSP_GETPARAM_VALUESTR_LENGTH as usize];
        unsafe {
            FMOD_DSP_GetParameterFloat(
                dsp,
                index,
                std::ptr::null_mut(),
                bytes.as_mut_ptr().cast(),
                FMOD_DSP_GETPARAM_VALUESTR_LENGTH as i32,
            )
            .to_result()?;

            let string = Utf8CString::from_utf8_with_nul(bytes.to_vec()).unwrap();
            Ok(string)
        }
    }
}

impl Dsp {
    /// Retrieve the index of the first data parameter of a particular data type.
    ///
    /// This function returns [`Ok`] if a parmeter of matching type is found and [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`] if no matches were found.
    ///
    /// The return code can be used to check whether the [`Dsp`] supports specific functionality through data parameters of certain types without the need to provide index.
    pub fn get_data_parameter_index(&self, data_type: DspParameterDataType) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_DSP_GetDataParameterIndex(self.inner.as_ptr(), data_type.into(), &mut index)
                .to_result()?;
        }
        Ok(index)
    }

    /// Retrieves the number of parameters exposed by this unit.
    ///
    /// Use this to enumerate all parameters of a [`Dsp`] unit with [`Dsp::get_parameter_info`].
    pub fn get_parameter_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_DSP_GetNumParameters(self.inner.as_ptr(), &mut count).to_result()? };
        Ok(count)
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
                self.inner.as_ptr(),
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
                self.inner.as_ptr(),
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

    /// Retrieve information about a specified parameter.
    pub fn get_parameter_info(&self, index: c_int) -> Result<DspParameterDescription> {
        let mut desc = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetParameterInfo(self.inner.as_ptr(), index, &mut desc).to_result()?;
            let desc = DspParameterDescription::from_ffi(*desc); // oh god this is *awful*
            Ok(desc)
        }
    }

    pub fn set_parameter<P: ParameterType>(&self, index: c_int, parameter: P) -> Result<()> {
        parameter.set_parameter(*self, index)
    }

    pub fn get_parameter<P: ParameterType>(&self, index: c_int) -> Result<P> {
        P::get_parameter(*self, index)
    }

    pub fn get_parameter_string<P: ParameterType>(&self, index: c_int) -> Result<Utf8CString> {
        P::get_parameter_string(*self, index)
    }
}
