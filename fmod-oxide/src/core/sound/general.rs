// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_void;

use crate::{Sound, System};

impl Sound {
    /// Frees a sound object.
    ///
    /// This will stop any instances of this sound, and free the sound object and its children if it is a multi-sound object.
    /// If the sound was opened with `FMOD_NONBLOCKING` and hasn't finished opening yet, it will block.
    /// Additionally, if the sound is still playing or has recently been stopped, the release may stall, as the mixer may still be using the sound.
    /// Using `Sound::getOpenState` and checking the open state for `FMOD_OPENSTATE_READY` and `FMOD_OPENSTATE_ERROR` is a good way to avoid stalls.
    pub fn release(&self) -> Result<()> {
        unsafe { FMOD_Sound_Release(self.inner.as_ptr()).to_result() }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Sound_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_GetUserData(self.inner.as_ptr(), &mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    /// Retrieves the parent System object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_GetSystemObject(self.inner.as_ptr(), &mut system).to_result()?;
        }
        Ok(system.into())
    }
}
