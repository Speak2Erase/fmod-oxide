// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::{ffi::c_int, mem::MaybeUninit};

use crate::studio::{EventDescription, ParameterDescription, ParameterID};

impl EventDescription {
    /// Retrieves an event parameter description by name.
    pub fn get_parameter_description_by_name(
        &self,
        name: &Utf8CStr,
    ) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetParameterDescriptionByName(
                self.inner.as_ptr(),
                name.as_ptr(),
                description.as_mut_ptr(),
            )
            .to_result()?;

            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves an event parameter description by id.
    pub fn get_parameter_description_by_id(&self, id: ParameterID) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetParameterDescriptionByID(
                self.inner.as_ptr(),
                id.into(),
                description.as_mut_ptr(),
            )
            .to_result()?;

            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves an event parameter description by index.
    ///
    /// May be used in combination with [`EventDescription::parameter_description_count`] to enumerate event parameters.
    ///
    /// Note: The order of parameters is not necessarily the same as what is shown in the FMOD Studio event editor.
    pub fn get_parameter_description_by_index(&self, index: c_int) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetParameterDescriptionByIndex(
                self.inner.as_ptr(),
                index,
                description.as_mut_ptr(),
            )
            .to_result()?;

            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves the number of parameters in the event.
    ///
    /// May be used in conjunction with [`EventDescription::get_parameter_description_by_index`] to enumerate event parameters.
    pub fn parameter_description_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetParameterDescriptionCount(
                self.inner.as_ptr(),
                &mut count,
            )
            .to_result()?;
        }
        Ok(count)
    }

    /// Retrieves an event parameter label by name or path.
    ///
    /// `name` can be the short name (such as `Wind`) or the full path (such as `parameter:/Ambience/Wind`).
    /// Path lookups will only succeed if the strings bank has been loaded.
    pub fn get_parameter_label_by_name(
        &self,
        name: &Utf8CStr,
        label_index: c_int,
    ) -> Result<Utf8CString> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_EventDescription_GetParameterLabelByName(
                self.inner.as_ptr(),
                name.as_ptr(),
                label_index,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_EventDescription_GetParameterLabelByName(
                self.inner.as_ptr(),
                name.as_ptr(),
                label_index,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = Utf8CString::from_utf8_with_nul_unchecked(path);

            Ok(path)
        }
    }

    /// Retrieves an event parameter label by ID.
    pub fn get_parameter_label_by_id(
        &self,
        id: ParameterID,
        label_index: c_int,
    ) -> Result<Utf8CString> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_EventDescription_GetParameterLabelByID(
                self.inner.as_ptr(),
                id.into(),
                label_index,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_EventDescription_GetParameterLabelByID(
                self.inner.as_ptr(),
                id.into(),
                label_index,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = Utf8CString::from_utf8_with_nul_unchecked(path);

            Ok(path)
        }
    }

    /// Retrieves an event parameter label by index.
    ///
    /// May be used in combination with [`EventDescription::parameter_description_count`] to enumerate event parameters.
    pub fn get_parameter_label_by_index(
        &self,
        index: c_int,
        label_index: c_int,
    ) -> Result<Utf8CString> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_EventDescription_GetParameterLabelByIndex(
                self.inner.as_ptr(),
                index,
                label_index,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_EventDescription_GetParameterLabelByIndex(
                self.inner.as_ptr(),
                index,
                label_index,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = Utf8CString::from_utf8_with_nul_unchecked(path);

            Ok(path)
        }
    }
}
