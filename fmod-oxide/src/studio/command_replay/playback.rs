// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int};

use crate::studio::{CommandReplay, PlaybackState};

impl CommandReplay {
    /// Begins playback.
    ///
    /// If the replay is already running then calling this function will restart replay from the beginning.
    pub fn start(&self) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_Start(self.inner).to_result() }
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
            let state = state.try_into()?;
            Ok(state)
        }
    }

    /// Sets the paused state.
    pub fn set_paused(&self, paused: bool) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_SetPaused(self.inner, paused.into()).to_result() }
    }

    /// Retrieves the paused state.
    pub fn get_paused(&self) -> Result<bool> {
        let mut paused = FMOD_BOOL::FALSE;
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
}
