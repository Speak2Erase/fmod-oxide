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
    ffi::{c_float, c_int, CStr},
    marker::PhantomData,
    mem::MaybeUninit,
    sync::Arc,
};

use fmod_sys::*;

use super::{CommandInfo, PlaybackState, System};
use crate::UserdataTypes;

mod userdata;
pub use userdata::*;

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct CommandReplay<U: UserdataTypes = ()> {
    pub(crate) inner: *mut FMOD_STUDIO_COMMANDREPLAY,
    _phantom: PhantomData<U>,
}

unsafe impl<U: UserdataTypes> Send for CommandReplay<U> {}
unsafe impl<U: UserdataTypes> Sync for CommandReplay<U> {}
impl<U: UserdataTypes> Clone for CommandReplay<U> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<U: UserdataTypes> Copy for CommandReplay<U> {}

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
    pub fn set_user_data(&self, data: Option<U::CommandReplay>) -> Result<()> {
        unsafe {
            let userdata = &mut *self.get_or_insert_userdata()?;
            userdata.userdata = data.map(Arc::new);
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
    pub fn get_user_data(&self) -> Result<Option<Arc<U::CommandReplay>>> {
        unsafe {
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_CommandReplay_GetUserData(self.inner, &mut userdata).to_result()?;

            if userdata.is_null() {
                return Ok(None);
            }

            // userdata should ALWAYS be InternalUserdata
            let userdata = &mut *userdata.cast::<InternalUserdata<U>>();
            let userdata = userdata.userdata.clone();
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
