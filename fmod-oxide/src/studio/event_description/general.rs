// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::c_int, mem::MaybeUninit};

use fmod_sys::*;
use lanyard::Utf8CString;

use crate::studio::EventDescription;
use crate::Guid;

impl EventDescription {
    /// Retrieves the GUID.
    pub fn get_id(&self) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetID(self.inner.as_ptr(), guid.as_mut_ptr())
                .to_result()?;

            let guid = guid.assume_init().into();

            Ok(guid)
        }
    }

    /// Retrieves the length of the timeline.
    ///
    /// A timeline's length is the largest of any logic markers, transition leadouts and the end of any trigger boxes on the timeline.
    pub fn get_length(&self) -> Result<c_int> {
        let mut length = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetLength(self.inner.as_ptr(), &mut length).to_result()?;
        }
        Ok(length)
    }

    /// Retrieves the path.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    // TODO: convert into possible macro for the sake of reusing code
    pub fn get_path(&self) -> Result<Utf8CString> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_EventDescription_GetPath(
                self.inner.as_ptr(),
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
            FMOD_Studio_EventDescription_GetPath(
                self.inner.as_ptr(),
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

    /// Checks that the [`EventDescription`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_EventDescription_IsValid(self.inner.as_ptr()).into() }
    }
}
