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
// along with fmod-rs.  If not, see <http://www.gnu.org/licenses/>.

use std::{
    ffi::{c_char, c_float, c_int, CStr},
    marker::PhantomData,
    mem::MaybeUninit,
    os::raw::c_void,
};

use fmod_sys::*;

use crate::{Guid, Shareable, UserdataTypes};

use super::{
    Bank, CommandInfo, EventDescription, EventInstance, LoadBankFlags, PlaybackState, System,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct CommandReplay<U: UserdataTypes = ()> {
    pub(crate) inner: *mut FMOD_STUDIO_COMMANDREPLAY,
    _phantom: PhantomData<U>,
}

pub(crate) struct InternalUserdata<U: UserdataTypes> {
    // we don't expose the callbacks at all so it's fine to just use a box
    create_instance_callback: Option<Box<dyn CreateInstanceCallback<U>>>,
    frame_callback: Option<Box<dyn FrameCallback<U>>>,
    load_bank_callback: Option<Box<dyn LoadBankCallback<U>>>,
    userdata: Option<U::CommandReplay>,
}

pub struct CreateInstanceData<'u, U: UserdataTypes> {
    pub replay: CommandReplay<U>,
    pub command_index: c_int,
    pub event_description: EventDescription,
    pub userdata: Option<&'u U::CommandReplay>,
}
pub trait CreateInstanceCallback<U: UserdataTypes>:
    Fn(CreateInstanceData<'_, U>) -> Result<Option<EventInstance>> + Shareable
{
}
impl<T, U> CreateInstanceCallback<U> for T
where
    T: Fn(CreateInstanceData<'_, U>) -> Result<Option<EventInstance>> + Shareable,
    U: UserdataTypes,
{
}

pub struct FrameData<'u, U: UserdataTypes> {
    pub replay: CommandReplay<U>,
    pub command_index: c_int,
    pub current_time: c_float,
    pub userdata: Option<&'u U::CommandReplay>,
}
pub trait FrameCallback<U: UserdataTypes>: Fn(FrameData<'_, U>) -> Result<()> + Shareable {}
impl<T, U> FrameCallback<U> for T
where
    T: Fn(FrameData<'_, U>) -> Result<()> + Shareable,
    U: UserdataTypes,
{
}

pub struct LoadBankData<'u, U: UserdataTypes> {
    pub replay: CommandReplay<U>,
    pub command_index: c_int,
    pub bank_guid: Option<Guid>,
    pub bank_filename: Option<&'static CStr>, // FIXME 'static wrong
    pub load_flags: LoadBankFlags,
    pub userdata: Option<&'u U::CommandReplay>,
}
pub trait LoadBankCallback<U: UserdataTypes>:
    Fn(LoadBankData<'_, U>) -> Result<Option<Bank>> + Shareable
{
}
impl<T, U> LoadBankCallback<U> for T
where
    T: Fn(LoadBankData<'_, U>) -> Result<Option<Bank>> + Shareable,
    U: UserdataTypes,
{
}

unsafe extern "C" fn internal_create_instance_callback<U: UserdataTypes>(
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
            event_description: event_description.into(),
            userdata: internal_userdata.userdata.as_ref(),
        };

        let result = callback(data);

        match result {
            Ok(Some(i)) => {
                *instance = i.inner;
                FMOD_RESULT::FMOD_OK
            }
            Ok(None) => FMOD_RESULT::FMOD_OK,
            Err(e) => e.code,
        }
    }
}

unsafe extern "C" fn internal_frame_callback<U: UserdataTypes>(
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
            userdata: internal_userdata.userdata.as_ref(),
        };

        callback(data).into()
    }
}

unsafe extern "C" fn internal_load_bank_callback<U: UserdataTypes>(
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
            userdata: internal_userdata.userdata.as_ref(),
        };

        let result = callback(data);

        match result {
            Ok(Some(b)) => {
                *bank = b.inner;
                FMOD_RESULT::FMOD_OK
            }
            Ok(None) => FMOD_RESULT::FMOD_OK,
            Err(e) => e.code,
        }
    }
}

unsafe impl<U: UserdataTypes> Send for CommandReplay<U> {}
unsafe impl<U: UserdataTypes> Sync for CommandReplay<U> {}

impl<U: UserdataTypes> CommandReplay<U> {
    /// Create a System instance from its FFI equivalent.
    ///
    /// # Safety
    /// This operation is unsafe because it's possible that the [`FMOD_STUDIO_COMMANDREPLAY`] will not have the right userdata type.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_COMMANDREPLAY) -> Self {
        CommandReplay {
            inner: value,
            _phantom: PhantomData,
        }
    }
}

impl<U: UserdataTypes> From<CommandReplay<U>> for *mut FMOD_STUDIO_COMMANDREPLAY {
    fn from(value: CommandReplay<U>) -> Self {
        value.inner
    }
}

impl<U: UserdataTypes> CommandReplay<U> {
    /// Sets a path substition that will be used when loading banks with this replay.
    ///
    /// [`System::load_bank_file`] commands in the replay are redirected to load banks from the specified directory, instead of using the directory recorded in the captured commands.
    pub fn set_bank_path(&self, path: &CStr) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_SetBankPath(self.inner, path.as_ptr()).to_result() }
    }

    /// The create instance callback is invoked each time a [`EventDescription::create_instance`] command is processed.
    ///
    /// The callback can either create a new event instance based on the callback parameters or skip creating the instance.
    /// If the instance is not created then subsequent commands for the event instance will be ignored in the replay.
    ///
    /// If this callback is not set then the system will always create an event instance.
    pub fn set_create_instance_callback<F>(&self, callback: F) -> Result<()>
    where
        F: CreateInstanceCallback<U>,
    {
        unsafe {
            let userdata = &mut *self.get_or_insert_userdata()?;
            userdata.create_instance_callback = Some(Box::new(callback));

            FMOD_Studio_CommandReplay_SetCreateInstanceCallback(
                self.inner,
                Some(internal_create_instance_callback::<U>),
            )
            .to_result()
        }
    }

    /// Sets a callback that is issued each time the replay reaches a new frame.
    pub fn set_frame_callback<F>(&self, callback: F) -> Result<()>
    where
        F: FrameCallback<U>,
    {
        unsafe {
            let userdata = &mut *self.get_or_insert_userdata()?;
            userdata.frame_callback = Some(Box::new(callback));

            FMOD_Studio_CommandReplay_SetFrameCallback(
                self.inner,
                Some(internal_frame_callback::<U>),
            )
            .to_result()
        }
    }

    /// The load bank callback is invoked whenever any of the Studio load bank functions are reached.
    ///
    /// This callback is required to be implemented to successfully replay [`System::load_bank_memory`] and [`System::load_bank_custom`] commands.
    ///
    /// The callback is responsible for loading the bank based on the callback parameters.
    /// If the bank is not loaded subsequent commands which reference objects in the bank will fail.
    ///
    /// If this callback is not set then the system will attempt to load banks from file according to recorded [`System::load_bank_file`] commands and skip other load commands.
    pub fn set_load_bank_callback<F>(&self, callback: F) -> Result<()>
    where
        F: LoadBankCallback<U>,
    {
        unsafe {
            let userdata = &mut *self.get_or_insert_userdata()?;
            userdata.load_bank_callback = Some(Box::new(callback));

            FMOD_Studio_CommandReplay_SetLoadBankCallback(
                self.inner,
                Some(internal_load_bank_callback::<U>),
            )
            .to_result()
        }
    }

    /// Sets user data.
    ///
    /// This function allows arbitrary user data to be attached to this object.
    /// The provided data may be shared/accessed from multiple threads, and so must implement Send + Sync 'static.
    pub fn set_user_data<T>(&self, data: Option<U::CommandReplay>) -> Result<()> {
        unsafe {
            let userdata = &mut *self.get_or_insert_userdata()?;
            userdata.userdata = data;
        }

        Ok(())
    }

    unsafe fn get_or_insert_userdata(&self) -> Result<*mut InternalUserdata<U>> {
        unsafe {
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_CommandReplay_GetUserData(self.inner, &mut userdata).to_result()?;

            // FIXME extract this common behavior into a macro or something
            // create and set the userdata if we haven't already
            if userdata.is_null() {
                let boxed_userdata = Box::new(InternalUserdata::<U> {
                    create_instance_callback: None,
                    frame_callback: None,
                    load_bank_callback: None,
                    userdata: None,
                });
                userdata = Box::into_raw(boxed_userdata).cast();

                FMOD_Studio_CommandReplay_SetUserData(self.inner, userdata).to_result()?;
            }

            Ok(userdata.cast::<InternalUserdata<U>>())
        }
    }

    /// Begins playback.
    ///
    /// If the replay is already running then calling this function will restart replay from the beginning.
    pub fn start(&self) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_Stop(self.inner).to_result() }
    }

    /// Stops playback.
    ///
    /// If the [`CommandReplayFlags::SKIP_CLEANUP`] flag has been used then the system state is left as it was at the end of the playback,
    /// otherwise all resources that were created as part of the replay will be cleaned up.
    pub fn stop(&self) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_Stop(self.inner).to_result() }
    }

    /// Retrieves the progress through the command replay.
    ///
    /// If this function is called before [`CommandReplay::start`] then both tuple fields will be 0.
    /// If this function is called after [`CommandReplay::stop`] then the index and time of the last command which was replayed will be returned.
    pub fn get_current_command(&self) -> Result<(c_int, c_float)> {
        let mut command_index = 0;
        let mut current_time = 0.0;
        unsafe {
            FMOD_Studio_CommandReplay_GetCurrentCommand(
                self.inner,
                &mut command_index,
                &mut current_time,
            )
            .to_result()?;
        }
        Ok((command_index, current_time))
    }

    /// Retrieves the playback state.
    pub fn get_playback_state(&self) -> Result<PlaybackState> {
        let mut state = 0;
        unsafe {
            FMOD_Studio_CommandReplay_GetPlaybackState(self.inner, &mut state).to_result()?;
        }
        Ok(state.into())
    }

    /// Sets the paused state.
    pub fn set_paused(&self, paused: bool) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_SetPaused(self.inner, paused.into()).to_result() }
    }

    /// Retrieves the paused state.
    pub fn get_paused(&self) -> Result<bool> {
        let mut paused = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_CommandReplay_GetPaused(self.inner, &mut paused).to_result()?;
        }
        Ok(paused.into())
    }

    /// Seeks the playback position to a command.
    pub fn seek_to_command(&self, index: c_int) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_SeekToCommand(self.inner, index).to_result() }
    }

    /// Seeks the playback position to a time.
    ///
    /// This function moves the playback position to the the first command at or after `time`.
    /// If no command exists at or after `time` then [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    pub fn seek_to_time(&self, time: c_float) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_SeekToTime(self.inner, time).to_result() }
    }

    /// Retrieves the command index corresponding to the given playback time.
    ///
    /// This function will return an index for the first command at or after `time`.
    /// If `time` is greater than the total playback time then [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    pub fn command_at_time(&self, time: c_float) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_Studio_CommandReplay_GetCommandAtTime(self.inner, time, &mut index).to_result()?;
        }
        Ok(index)
    }

    /// Retrieves the number of commands in the replay.
    pub fn get_command_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_CommandReplay_GetCommandCount(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves command information.
    pub fn get_command_info(&self, index: c_int) -> Result<CommandInfo> {
        let mut info = MaybeUninit::zeroed();

        unsafe {
            FMOD_Studio_CommandReplay_GetCommandInfo(self.inner, index, info.as_mut_ptr())
                .to_result()?;

            let info = CommandInfo::from_ffi(info.assume_init());
            Ok(info)
        }
    }

    /// Retrieves the string representation of a command.
    pub fn get_command_string(&self, index: c_int) -> Result<String> {
        unsafe {
            let mut buffer = vec![0; 32];

            // run this once (best case) before we fall into the loop
            let mut result = FMOD_Studio_CommandReplay_GetCommandString(
                self.inner,
                index,
                buffer.as_mut_ptr().cast::<i8>(),
                buffer.len() as c_int,
            );

            // this function behaves differently to every other fmod function? strange
            // we copy what the c# bindings do, loop until the string fits
            while let FMOD_RESULT::FMOD_ERR_TRUNCATED = result {
                buffer.resize(buffer.len() + 32, 0);
                result = FMOD_Studio_CommandReplay_GetCommandString(
                    self.inner,
                    index,
                    buffer.as_mut_ptr().cast::<i8>(),
                    buffer.len() as c_int,
                );
            }
            result.to_result()?;

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let mut string = String::from_utf8_unchecked(buffer);
            // shrink the string to avoid wasting memory
            string.shrink_to_fit();

            Ok(string)
        }
    }

    /// Retrieves the total playback time.
    pub fn get_length(&self) -> Result<c_float> {
        let mut length = 0.0;
        unsafe {
            FMOD_Studio_CommandReplay_GetLength(self.inner, &mut length).to_result()?;
        }
        Ok(length)
    }

    /// Retrieves the Studio System object associated with this replay object.
    pub fn get_system(&self) -> Result<System<U>> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_CommandReplay_GetSystem(self.inner, &mut system).to_result()?;
            Ok(System::from_ffi(system))
        }
    }

    ///Retrieves user data.
    ///
    /// This function allows arbitrary user data to be retrieved from this object.
    pub fn get_user_data<T>(&self) -> Result<Option<&U::CommandReplay>> {
        unsafe {
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_CommandReplay_GetUserData(self.inner, &mut userdata).to_result()?;

            if userdata.is_null() {
                return Ok(None);
            }

            // userdata should ALWAYS be InternalUserdata
            let userdata = &mut *userdata.cast::<InternalUserdata<U>>();
            let userdata = userdata.userdata.as_ref();
            Ok(userdata)
        }
    }

    /// Checks that the [`CommandReplay`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_CommandReplay_IsValid(self.inner).into() }
    }

    /// Releases the command replay.
    pub fn release(self) -> Result<()> {
        unsafe {
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_CommandReplay_GetUserData(self.inner, &mut userdata).to_result()?;

            // deallocate the userdata
            if !userdata.is_null() {
                let userdata = Box::from_raw(userdata.cast::<InternalUserdata<U>>());
                drop(userdata);
                FMOD_Studio_CommandReplay_SetUserData(self.inner, std::ptr::null_mut())
                    .to_result()?;
            }

            FMOD_Studio_CommandReplay_Release(self.inner).to_result()?;

            Ok(())
        }
    }
}
