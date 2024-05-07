// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CStr;
use std::{
    ffi::{c_float, c_int},
    mem::MaybeUninit,
};

use crate::studio::{CommandInfo, CommandReplay, System};

impl CommandReplay {
    /// Sets a path substition that will be used when loading banks with this replay.
    ///
    /// [`System::load_bank_file`] commands in the replay are redirected to load banks from the specified directory, instead of using the directory recorded in the captured commands.
    pub fn set_bank_path(&self, path: &Utf8CStr) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_SetBankPath(self.inner, path.as_ptr()).to_result() }
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
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_CommandReplay_GetSystem(self.inner, &mut system).to_result()?;
            Ok(System::from_ffi(system))
        }
    }

    /// Checks that the [`CommandReplay`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_CommandReplay_IsValid(self.inner).into() }
    }
}
