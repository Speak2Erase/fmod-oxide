// Copyright (c) 2024 Lily Lyons
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
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe {
            FMOD_Studio_EventDescription_SetUserData(self.inner.as_ptr(), userdata).to_result()
        }
    }

    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventDescription_GetUserData(self.inner.as_ptr(), &mut userdata)
                .to_result()?;
        }
        Ok(userdata)
    }

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
