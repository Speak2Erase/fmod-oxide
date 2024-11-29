// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::mem::MaybeUninit;

use crate::studio::System;
use crate::Guid;

impl System {
    /// Retrieves the Core System.
    pub fn get_core_system(&self) -> Result<crate::core::System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetCoreSystem(self.inner.as_ptr(), &mut system).to_result()?;
        }
        Ok(system.into())
    }

    /// Retrieves the ID for a bank, event, snapshot, bus or VCA.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    ///
    /// The path can be copied to the system clipboard from FMOD Studio using the "Copy Path" context menu command.
    pub fn lookup_id(&self, path: &Utf8CStr) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_LookupID(self.inner.as_ptr(), path.as_ptr(), guid.as_mut_ptr())
                .to_result()?;

            let guid = guid.assume_init().into();
            Ok(guid)
        }
    }

    /// Retrieves the path for a bank, event, snapshot, bus or VCA.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    pub fn lookup_path(&self, id: Guid) -> Result<Utf8CString> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_System_LookupPath(
                self.inner.as_ptr(),
                &id.into(),
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
            FMOD_Studio_System_LookupPath(
                self.inner.as_ptr(),
                &id.into(),
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

    /// Checks that the [`System`] reference is valid and has been initialized.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_System_IsValid(self.inner.as_ptr()).into() }
    }
}
