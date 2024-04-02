// Copyright (C) 2024 Lily Lyons
//
// This file is part of fmod-rs.
//
// fmod-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fmod-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with fmod-rs.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    ffi::{c_char, c_float, c_int, CStr},
    os::raw::c_void,
    sync::Arc,
};

use fmod_sys::*;

use crate::studio::{Bank, CommandReplay, EventDescription, EventInstance, LoadBankFlags};
use crate::{Guid, Shareable, UserdataTypes};

pub(crate) struct InternalUserdata<U: UserdataTypes> {
    // we don't expose the callbacks at all so it's fine to just use a box
    pub create_instance_callback: Option<Box<dyn CreateInstanceCallback<U>>>,
    pub frame_callback: Option<Box<dyn FrameCallback<U>>>,
    pub load_bank_callback: Option<Box<dyn LoadBankCallback<U>>>,
    pub userdata: Option<Arc<U::CommandReplay>>,
}

pub struct CreateInstanceData<U: UserdataTypes> {
    pub replay: CommandReplay<U>,
    pub command_index: c_int,
    pub event_description: EventDescription<U>,
    pub userdata: Option<Arc<U::CommandReplay>>,
}
pub trait CreateInstanceCallback<U: UserdataTypes>:
    Fn(CreateInstanceData<U>) -> Result<Option<EventInstance<U>>> + Shareable
{
}
impl<T, U> CreateInstanceCallback<U> for T
where
    T: Fn(CreateInstanceData<U>) -> Result<Option<EventInstance<U>>> + Shareable,
    U: UserdataTypes,
{
}

pub struct FrameData<U: UserdataTypes> {
    pub replay: CommandReplay<U>,
    pub command_index: c_int,
    pub current_time: c_float,
    pub userdata: Option<Arc<U::CommandReplay>>,
}
pub trait FrameCallback<U: UserdataTypes>: Fn(FrameData<U>) -> Result<()> + Shareable {}
impl<T, U> FrameCallback<U> for T
where
    T: Fn(FrameData<U>) -> Result<()> + Shareable,
    U: UserdataTypes,
{
}

pub struct LoadBankData<U: UserdataTypes> {
    pub replay: CommandReplay<U>,
    pub command_index: c_int,
    pub bank_guid: Option<Guid>,
    pub bank_filename: Option<&'static CStr>, // FIXME 'static wrong
    pub load_flags: LoadBankFlags,
    pub userdata: Option<Arc<U::CommandReplay>>,
}
pub trait LoadBankCallback<U: UserdataTypes>:
    Fn(LoadBankData<U>) -> Result<Option<Bank<U>>> + Shareable
{
}
impl<T, U> LoadBankCallback<U> for T
where
    T: Fn(LoadBankData<U>) -> Result<Option<Bank<U>>> + Shareable,
    U: UserdataTypes,
{
}

pub(crate) unsafe extern "C" fn internal_create_instance_callback<U: UserdataTypes>(
    replay: *mut FMOD_STUDIO_COMMANDREPLAY,
    command_index: c_int,
    event_description: *mut FMOD_STUDIO_EVENTDESCRIPTION,
    instance: *mut *mut FMOD_STUDIO_EVENTINSTANCE,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    // userdata should always be set
    // when setting a callback by necessity userdata is set as we don't set any callbacks by default
    #[cfg(debug_assertions)]
    if userdata.is_null() {
        eprintln!("commandreplay userdata is null. aborting");
        std::process::abort();
    }

    // FIXME: handle unwinding panics
    unsafe {
        let internal_userdata = &mut *userdata.cast::<InternalUserdata<U>>();
        // the callback should ALWAYS be set if this callback is set
        let callback = internal_userdata
            .create_instance_callback
            .as_ref()
            .unwrap_unchecked();
        let data = CreateInstanceData {
            replay: CommandReplay::from_ffi(replay),
            command_index,
            event_description: EventDescription::from_ffi(event_description),
            userdata: internal_userdata.userdata.clone(),
        };

        let result = callback(data);

        match result {
            Ok(Some(i)) => {
                *instance = i.inner;
                FMOD_RESULT::FMOD_OK
            }
            Ok(None) => FMOD_RESULT::FMOD_OK,
            Err(e) => e.into(),
        }
    }
}

pub(crate) unsafe extern "C" fn internal_frame_callback<U: UserdataTypes>(
    replay: *mut FMOD_STUDIO_COMMANDREPLAY,
    command_index: c_int,
    current_time: c_float,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    // userdata should always be set
    // when setting a callback by necessity userdata is set as we don't set any callbacks by default
    #[cfg(debug_assertions)]
    if userdata.is_null() {
        eprintln!("commandreplay userdata is null. aborting");
        std::process::abort();
    }

    // FIXME: handle unwinding panics
    unsafe {
        let internal_userdata = &mut *userdata.cast::<InternalUserdata<U>>();
        // the callback should ALWAYS be set if this callback is set
        let callback = internal_userdata.frame_callback.as_ref().unwrap_unchecked();
        let data = FrameData {
            replay: CommandReplay::from_ffi(replay),
            command_index,
            current_time,
            userdata: internal_userdata.userdata.clone(),
        };

        callback(data).into()
    }
}

pub(crate) unsafe extern "C" fn internal_load_bank_callback<U: UserdataTypes>(
    replay: *mut FMOD_STUDIO_COMMANDREPLAY,
    command_index: c_int,
    bank_guid: *const FMOD_GUID,
    bank_filename: *const c_char,
    load_flags: FMOD_STUDIO_LOAD_BANK_FLAGS,
    bank: *mut *mut FMOD_STUDIO_BANK,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    // userdata should always be set
    // when setting a callback by necessity userdata is set as we don't set any callbacks by default
    #[cfg(debug_assertions)]
    if userdata.is_null() {
        eprintln!("commandreplay userdata is null. aborting");
        std::process::abort();
    }

    // FIXME: handle unwinding panics
    unsafe {
        let internal_userdata = &mut *userdata.cast::<InternalUserdata<U>>();
        // the callback should ALWAYS be set if this callback is set
        let callback = internal_userdata
            .load_bank_callback
            .as_ref()
            .unwrap_unchecked();

        let data = LoadBankData {
            replay: CommandReplay::from_ffi(replay),
            command_index,
            bank_guid: if bank_guid.is_null() {
                None
            } else {
                Some(Guid::from(*bank_guid))
            },
            bank_filename: if bank_filename.is_null() {
                None
            } else {
                Some(CStr::from_ptr(bank_filename))
            },
            load_flags: load_flags.into(),
            userdata: internal_userdata.userdata.clone(),
        };

        let result = callback(data);

        match result {
            Ok(Some(b)) => {
                *bank = b.inner;
                FMOD_RESULT::FMOD_OK
            }
            Ok(None) => FMOD_RESULT::FMOD_OK,
            Err(e) => e.into(),
        }
    }
}
