// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_void;

use crate::studio::{
    event_callback_impl, EventCallbackMask, EventDescription, EventInstanceCallback,
};

impl EventDescription {
    /// Sets the event user data.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe {
            FMOD_Studio_EventDescription_SetUserData(self.inner.as_ptr(), userdata).to_result()
        }
    }

    /// Retrieves the event user data.
    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventDescription_GetUserData(self.inner.as_ptr(), &mut userdata)
                .to_result()?;
        }
        Ok(userdata)
    }

    /// Sets the user callback.
    ///
    /// This function sets a user callback which will be assigned to all event instances subsequently created from the event.
    /// The callback for individual instances can be set with `EventInstance::set_callback`.
    pub fn set_callback<C: EventInstanceCallback>(&self, mask: EventCallbackMask) -> Result<()> {
        unsafe {
            FMOD_Studio_EventDescription_SetCallback(
                self.inner.as_ptr(),
                Some(event_callback_impl::<C>),
                mask.into(),
            )
            .to_result()
        }
    }
}
