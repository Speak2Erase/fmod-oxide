// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_void;

use crate::studio::{Bank, System, SystemCallbackMask};

/// Trait for this particular FMOD callback.
///
/// No `self` parameter is passed to the callback!
///
/// Callbacks are called from the Studio Update Thread in default / async mode and the main (calling) thread in synchronous mode.
#[allow(unused_variables)]
pub trait SystemCallback {
    /// Called at the start of the main Studio update. For async mode this will be on its own thread.
    fn preupdate(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called at the end of the main Studio update. For async mode this will be on its own thread.
    fn postupdate(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called directly when a bank has just been unloaded, after all resources are freed.
    fn bank_unload(system: System, bank: Bank, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called after a live update connection has been established.
    fn liveupdate_connected(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called after live update session disconnects.
    fn liveupdate_disconnected(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }
}

unsafe extern "C" fn callback_impl<C: SystemCallback>(
    system: *mut FMOD_STUDIO_SYSTEM,
    kind: FMOD_SYSTEM_CALLBACK_TYPE,
    command_data: *mut c_void,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    // FIXME handle panics
    let system = System::from(system);

    let result = match kind {
        FMOD_STUDIO_SYSTEM_CALLBACK_PREUPDATE => C::preupdate(system, userdata),
        FMOD_STUDIO_SYSTEM_CALLBACK_POSTUPDATE => C::postupdate(system, userdata),
        FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD => {
            let bank = Bank::from(command_data.cast());
            C::bank_unload(system, bank, userdata)
        }
        FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_CONNECTED => {
            C::liveupdate_connected(system, userdata)
        }
        FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_DISCONNECTED => {
            C::liveupdate_disconnected(system, userdata)
        }
        _ => {
            eprintln!("warning: unknown event callback type {kind}");
            return FMOD_RESULT::FMOD_OK;
        }
    };
    result.into()
}

impl System {
    /// Sets the user data.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Studio_System_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    /// Retrieves the user data.
    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetUserData(self.inner.as_ptr(), &mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    /// Sets a callback for the Studio System.
    pub fn set_callback<C: SystemCallback>(&self, mask: SystemCallbackMask) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetCallback(
                self.inner.as_ptr(),
                Some(callback_impl::<C>),
                mask.into(),
            )
            .to_result()
        }
    }
}
