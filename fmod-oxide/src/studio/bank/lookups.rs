// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;
use std::{ffi::c_int, mem::MaybeUninit};

use crate::studio::{Bank, Bus, EventDescription, Vca};
use crate::Guid;
use fmod_sys::*;
use lanyard::Utf8CString;

impl Bank {
    /// Retrieves the number of buses in the bank.
    pub fn bus_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetBusCount(self.inner.as_ptr(), &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the buses in the bank.
    pub fn get_bus_list(&self) -> Result<Vec<Bus>> {
        let expected_count = self.bus_count()?;
        let mut count = 0;
        let mut list = vec![
            Bus {
                inner: NonNull::dangling(), // we can't store a *null* NonNull pointer so we use NonNull::dangling instead
            };
            expected_count as usize
        ];

        unsafe {
            FMOD_Studio_Bank_GetBusList(
                self.inner.as_ptr(),
                // bus is repr transparent and has the same layout as *mut FMOD_STUDIO_BUS, so this cast is ok
                list.as_mut_ptr().cast::<*mut FMOD_STUDIO_BUS>(),
                list.capacity() as c_int,
                &mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(list)
        }
    }

    /// Retrives the number of event descriptions in the bank.
    ///
    /// This function counts the events which were added to the bank by the sound designer.
    /// The bank may contain additional events which are referenced by event instruments but were not added to the bank, and those referenced events are not counted.
    pub fn event_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetEventCount(self.inner.as_ptr(), &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the event descriptions in the bank.
    ///
    /// This function counts the events which were added to the bank by the sound designer.
    /// The bank may contain additional events which are referenced by event instruments but were not added to the bank, and those referenced events are not counted.
    pub fn get_event_list(&self) -> Result<Vec<EventDescription>> {
        let expected_count = self.event_count()?;
        let mut count = 0;
        let mut list = vec![std::ptr::null_mut(); expected_count as usize];

        unsafe {
            FMOD_Studio_Bank_GetEventList(
                self.inner.as_ptr(),
                // bus is repr transparent and has the same layout as *mut FMOD_STUDIO_BUS, so this cast is ok
                list.as_mut_ptr(),
                list.capacity() as c_int,
                &mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(std::mem::transmute::<
                Vec<*mut fmod_sys::FMOD_STUDIO_EVENTDESCRIPTION>,
                Vec<EventDescription>,
            >(list))
        }
    }

    /// Retrieves the number of string table entries in the bank.
    pub fn string_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetStringCount(self.inner.as_ptr(), &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a string table entry.
    ///
    /// May be used in conjunction with [`Bank::string_count`] to enumerate the string table in a bank.
    pub fn get_string_info(&self, index: c_int) -> Result<(Guid, Utf8CString)> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_Bank_GetStringInfo(
                self.inner.as_ptr(),
                index,
                std::ptr::null_mut(),
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

        let mut guid = MaybeUninit::zeroed();
        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_Bank_GetStringInfo(
                self.inner.as_ptr(),
                index,
                guid.as_mut_ptr(),
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // even if fmod didn't write to guid, guid should be safe to zero initialize.
            let guid = guid.assume_init().into();
            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = Utf8CString::from_utf8_with_nul_unchecked(path);

            Ok((guid, path))
        }
    }

    /// Retrieves the number of VCAs in the bank.
    pub fn vca_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetVCACount(self.inner.as_ptr(), &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the VCAs in the bank.
    pub fn get_vca_list(&self) -> Result<Vec<Vca>> {
        let expected_count = self.event_count()?;
        let mut count = 0;
        let mut list = vec![
            Vca {
                inner: NonNull::dangling(), // same trick used here as getting the bus list
            };
            expected_count as usize
        ];

        unsafe {
            FMOD_Studio_Bank_GetVCAList(
                self.inner.as_ptr(),
                // bus is repr transparent and has the same layout as *mut FMOD_STUDIO_BUS, so this cast is ok
                list.as_mut_ptr().cast::<*mut FMOD_STUDIO_VCA>(),
                list.capacity() as c_int,
                &mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(list)
        }
    }
}
