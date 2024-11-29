// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::studio::{EventInstance, PlaybackState, StopMode};

impl EventInstance {
    /// Starts playback.
    ///
    ///If the instance was already playing then calling this function will restart the event.
    ///
    /// Generally it is a best practice to call [`EventInstance::release`] on event instances immediately after starting them,
    /// unless you want to play the event instance multiple times or explicitly stop it and start it again later.
    pub fn start(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_Start(self.inner.as_ptr()).to_result() }
    }

    /// Stops playback.
    pub fn stop(&self, mode: StopMode) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_Stop(self.inner.as_ptr(), mode.into()).to_result() }
    }

    /// Retrieves the playback state.
    ///
    /// You can poll this function to track the playback state of an event instance.
    ///
    /// If the instance is invalid, then the state will be set to [`PlaybackState::Stopped`].
    pub fn get_playback_state(&self) -> Result<PlaybackState> {
        let mut state = 0;
        unsafe {
            FMOD_Studio_EventInstance_GetPlaybackState(self.inner.as_ptr(), &mut state)
                .to_result()?;
        }
        let state = state.try_into()?;
        Ok(state)
    }

    /// Sets the pause state.
    ///
    /// This function allows pausing/unpausing of an event instance.
    pub fn set_paused(&self, paused: bool) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetPaused(self.inner.as_ptr(), paused.into()).to_result()
        }
    }

    /// Retrieves the paused state.
    pub fn get_paused(&self) -> Result<bool> {
        let mut paused = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_EventInstance_GetPaused(self.inner.as_ptr(), &mut paused).to_result()?;
        }
        Ok(paused.into())
    }

    /// Allow an event to continue past a sustain point.
    ///
    /// Multiple sustain points may be bypassed ahead of time and the key off count will be decremented each time the timeline cursor passes a sustain point.
    ///
    /// This function returns [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] if the event has no sustain points.
    pub fn key_off(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_KeyOff(self.inner.as_ptr()).to_result() }
    }
}
