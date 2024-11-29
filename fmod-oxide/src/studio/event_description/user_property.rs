// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CStr;
use std::{ffi::c_int, mem::MaybeUninit};

use crate::studio::{EventDescription, UserProperty};

impl EventDescription {
    /// Retrieves a user property by name.
    pub fn get_user_property(&self, name: &Utf8CStr) -> Result<UserProperty> {
        let mut property = MaybeUninit::uninit();
        unsafe {
            FMOD_Studio_EventDescription_GetUserProperty(
                self.inner.as_ptr(),
                name.as_ptr(),
                property.as_mut_ptr(),
            )
            .to_result()?;

            let property = UserProperty::from_ffi(property.assume_init());
            Ok(property)
        }
    }

    /// Retrieves a user property by index.
    ///
    /// May be used in combination with [`EventDescription::user_property_count`] to enumerate event user properties.
    pub fn get_user_property_by_index(&self, index: c_int) -> Result<UserProperty> {
        let mut property = MaybeUninit::uninit();
        unsafe {
            FMOD_Studio_EventDescription_GetUserPropertyByIndex(
                self.inner.as_ptr(),
                index,
                property.as_mut_ptr(),
            )
            .to_result()?;

            let property = UserProperty::from_ffi(property.assume_init());
            Ok(property)
        }
    }

    /// Retrieves the number of user properties attached to the event.
    pub fn user_property_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetUserPropertyCount(self.inner.as_ptr(), &mut count)
                .to_result()?;
        }
        Ok(count)
    }
}
