// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_void;

use crate::studio::{Bank, System, SystemCallbackMask};

#[allow(unused_variables)]
pub trait SystemCallback {
    fn preupdate(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn postupdate(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn bank_unload(system: System, bank: Bank, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn liveupdate_connected(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

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
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Studio_System_SetUserData(self.inner, userdata).to_result() }
    }

    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetUserData(self.inner, &mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    pub fn set_callback<C: SystemCallback>(&self, mask: SystemCallbackMask) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetCallback(self.inner, Some(callback_impl::<C>), mask.into())
                .to_result()
        }
    }
}
