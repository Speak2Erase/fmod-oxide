// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CStr;
use std::ffi::{c_char, c_float, c_int, c_uint, c_void};

use crate::{
    studio::{Bank, CommandReplay, EventDescription, EventInstance, LoadBankFlags},
    Guid,
};

pub trait CreateInstanceCallback {
    fn create_instance_callback(
        replay: CommandReplay,
        command_index: c_int,
        description: EventDescription,
        userdata: *mut c_void,
    ) -> Result<Option<EventInstance>>;
}

unsafe extern "C" fn create_instance_impl<C: CreateInstanceCallback>(
    replay: *mut FMOD_STUDIO_COMMANDREPLAY,
    command_index: c_int,
    event_description: *mut FMOD_STUDIO_EVENTDESCRIPTION,
    event_instance: *mut *mut FMOD_STUDIO_EVENTINSTANCE,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    unsafe {
        let replay = CommandReplay::from(replay);
        let description = EventDescription::from(event_description);
        let result = C::create_instance_callback(replay, command_index, description, userdata);
        match result {
            Ok(Some(instance)) => {
                std::ptr::write(event_instance, instance.into());
                FMOD_RESULT::FMOD_OK
            }
            Ok(None) => FMOD_RESULT::FMOD_OK,
            Err(e) => e.into(),
        }
    }
}

pub trait FrameCallback {
    fn frame_callback(
        replay: CommandReplay,
        command_index: c_int,
        current_time: c_float,
        userdata: *mut c_void,
    ) -> Result<()>;
}

unsafe extern "C" fn frame_impl<C: FrameCallback>(
    replay: *mut FMOD_STUDIO_COMMANDREPLAY,
    command_index: c_int,
    current_time: c_float,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let replay = CommandReplay::from(replay);
    C::frame_callback(replay, command_index, current_time, userdata).into()
}

pub trait LoadBankCallback {
    fn load_bank_callback(
        replay: CommandReplay,
        command_index: c_int,
        guid: Option<Guid>,
        filename: Option<&Utf8CStr>,
        flags: LoadBankFlags,
        userdata: *mut c_void,
    ) -> Result<Option<Bank>>;
}

unsafe extern "C" fn load_bank_impl<C: LoadBankCallback>(
    replay: *mut FMOD_STUDIO_COMMANDREPLAY,
    command_index: c_int,
    guid: *const FMOD_GUID,
    filename: *const c_char,
    flags: c_uint,
    bank_ptr: *mut *mut FMOD_STUDIO_BANK,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let replay = CommandReplay::from(replay);
    let flags = LoadBankFlags::from(flags);
    let guid = if guid.is_null() {
        None
    } else {
        Some(unsafe { std::ptr::read(guid.cast()) })
    };
    let filename = if filename.is_null() {
        None
    } else {
        Some(unsafe { Utf8CStr::from_ptr_unchecked(filename) })
    };
    let result = C::load_bank_callback(replay, command_index, guid, filename, flags, userdata);
    match result {
        Ok(Some(bank)) => {
            unsafe {
                std::ptr::write(bank_ptr, bank.into());
            }
            FMOD_RESULT::FMOD_OK
        }
        Ok(None) => FMOD_RESULT::FMOD_OK,
        Err(e) => e.into(),
    }
}

impl CommandReplay {
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_CommandReplay_GetUserData(self.inner.as_ptr(), &mut userdata)
                .to_result()?;
        }
        Ok(userdata)
    }

    pub fn set_create_instance_callback<C: CreateInstanceCallback>(&self) -> Result<()> {
        unsafe {
            FMOD_Studio_CommandReplay_SetCreateInstanceCallback(
                self.inner.as_ptr(),
                Some(create_instance_impl::<C>),
            )
            .to_result()
        }
    }

    pub fn set_frame_callback<C: FrameCallback>(&self) -> Result<()> {
        unsafe {
            FMOD_Studio_CommandReplay_SetFrameCallback(self.inner.as_ptr(), Some(frame_impl::<C>))
                .to_result()
        }
    }

    pub fn set_load_bank_callback<C: LoadBankCallback>(&self) -> Result<()> {
        unsafe {
            FMOD_Studio_CommandReplay_SetLoadBankCallback(
                self.inner.as_ptr(),
                Some(load_bank_impl::<C>),
            )
            .to_result()
        }
    }
}
