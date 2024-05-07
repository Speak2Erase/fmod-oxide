// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::mem::MaybeUninit;

use crate::studio::Bank;
use crate::Guid;

impl Bank {
    /// Retrieves the GUID.
    pub fn get_id(&self) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_Bank_GetID(self.inner, guid.as_mut_ptr()).to_result()?;

            let guid = guid.assume_init().into();

            Ok(guid)
        }
    }

    /// Retrieves the path.
    pub fn get_path(&self) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error =
                FMOD_Studio_Bank_GetPath(self.inner, std::ptr::null_mut(), 0, &mut string_len)
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
            FMOD_Studio_Bank_GetPath(
                self.inner,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }

    /// Checks that the Bank reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_Bank_IsValid(self.inner).into() }
    }
}
