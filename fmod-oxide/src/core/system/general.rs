// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_void;

use fmod_sys::*;

use crate::System;

impl System {
    /// Mutual exclusion function to lock the FMOD DSP engine (which runs asynchronously in another thread), so that it will not execute.
    ///
    /// If the FMOD DSP engine is already executing, this function will block until it has completed.
    ///
    /// The function may be used to synchronize DSP network operations carried out by the user.
    ///
    /// An example of using this function may be for when the user wants to construct a DSP sub-network, without the DSP engine executing in the background while the sub-network is still under construction.
    ///
    /// Once the user no longer needs the DSP engine locked, it must be unlocked with [`System::unlock_dsp`].
    ///
    /// Note that the DSP engine should not be locked for a significant amount of time, otherwise inconsistency in the audio output may result. (audio skipping / stuttering).
    pub fn lock_dsp(&self) -> Result<()> {
        unsafe { FMOD_System_LockDSP(self.inner).to_result() }
    }

    // TODO add guard and investigate safety
    /// Mutual exclusion function to unlock the FMOD DSP engine (which runs asynchronously in another thread) and let it continue executing.
    ///
    /// The DSP engine must be locked with [`System::lock_dsp`] before this function is called.
    pub fn unlock_dsp(&self) -> Result<()> {
        unsafe { FMOD_System_UnlockDSP(self.inner).to_result() }
    }

    // TODO callbacks

    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_raw_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_System_SetUserData(self.inner, userdata).to_result() }
    }

    pub fn get_raw_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_System_GetUserData(self.inner, &mut userdata).to_result()?;
        }
        Ok(userdata)
    }
}

#[cfg(feature = "userdata-abstraction")]
impl System {
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
