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
            FMOD_SoundGroup_GetName(self.inner, name.as_mut_ptr().cast(), name.len() as c_int)
        })
    }

    /// Releases a soundgroup object and returns all sounds back to the master sound group.
    ///
    /// You cannot release the master [`SoundGroup`].
    pub fn release(&self) -> Result<()> {
        // release userdata
        #[cfg(feature = "userdata-abstraction")]
        let userdata = self.get_raw_userdata()?;

        unsafe {
            FMOD_SoundGroup_Release(self.inner).to_result()?;
        }

        // release/remove userdata if it is not null
        #[cfg(feature = "userdata-abstraction")]
        if !userdata.is_null() {
            crate::userdata::remove_userdata(userdata.into());
            self.set_raw_userdata(std::ptr::null_mut())?;
        }

        Ok(())
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_raw_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_SoundGroup_SetUserData(self.inner, userdata).to_result() }
    }

    pub fn get_raw_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_SoundGroup_GetUserData(self.inner, &mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    /// Retrieves the parent System object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_SoundGroup_GetSystemObject(self.inner, &mut system).to_result()? };
        Ok(system.into())
    }
}

#[cfg(feature = "userdata-abstraction")]
impl SoundGroup {
    pub fn set_userdata(&self, userdata: crate::userdata::Userdata) -> Result<()> {
        use crate::userdata::{insert_userdata, set_userdata};

        let pointer = self.get_raw_userdata()?;
        if pointer.is_null() {
            let key = insert_userdata(userdata, *self);
            self.set_raw_userdata(key.into())?;
        } else {
            set_userdata(pointer.into(), userdata);
        }

        Ok(())
    }

    pub fn get_userdata(&self) -> Result<Option<crate::userdata::Userdata>> {
        use crate::userdata::get_userdata;

        let pointer = self.get_raw_userdata()?;
        Ok(get_userdata(pointer.into()))
    }
}
