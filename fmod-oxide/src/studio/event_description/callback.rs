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

#[cfg(feature = "userdata-abstraction")]
use crate::userdata::{get_userdata, insert_userdata, set_userdata, Userdata};

#[cfg(feature = "userdata-abstraction")]
impl EventDescription {
    pub fn set_userdata(&self, userdata: Userdata) -> Result<()> {
        let pointer = self.get_raw_userdata()?;
        if pointer.is_null() {
            let key = insert_userdata(userdata, *self);
            self.set_raw_userdata(key.into())?;
        } else {
            set_userdata(pointer.into(), userdata);
        }

        Ok(())
    }

    pub fn get_userdata(&self) -> Result<Option<Userdata>> {
        let pointer = self.get_raw_userdata()?;
        Ok(get_userdata(pointer.into()))
    }
}

impl EventDescription {
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_raw_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Studio_EventDescription_SetUserData(self.inner, userdata).to_result() }
    }

    pub fn get_raw_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventDescription_GetUserData(self.inner, &mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    pub fn set_callback<C: EventInstanceCallback>(&self, mask: EventCallbackMask) -> Result<()> {
        unsafe {
            FMOD_Studio_EventDescription_SetCallback(
                self.inner,
                Some(event_callback_impl::<C>),
                mask.into(),
            )
            .to_result()
        }
    }
}
