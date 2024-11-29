// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::{c_int, c_void};

use crate::{get_string, SoundGroup, System};

impl SoundGroup {
    /// Retrieves the name of the sound group.
    pub fn get_name(&self) -> Result<Utf8CString> {
        get_string(|name| unsafe {
            FMOD_SoundGroup_GetName(
                self.inner.as_ptr(),
                name.as_mut_ptr().cast(),
                name.len() as c_int,
            )
        })
    }

    /// Releases a soundgroup object and returns all sounds back to the master sound group.
    ///
    /// You cannot release the master [`SoundGroup`].
    pub fn release(&self) -> Result<()> {
        unsafe { FMOD_SoundGroup_Release(self.inner.as_ptr()).to_result() }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_SoundGroup_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_SoundGroup_GetUserData(self.inner.as_ptr(), &mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    /// Retrieves the parent System object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_SoundGroup_GetSystemObject(self.inner.as_ptr(), &mut system).to_result()? };
        Ok(system.into())
    }
}
