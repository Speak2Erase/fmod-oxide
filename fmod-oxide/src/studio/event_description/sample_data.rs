// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::studio::{EventDescription, LoadingState};

impl EventDescription {
    /// Loads non-streaming sample data used by the event.
    ///
    /// This function will load all non-streaming sample data required by the event and any referenced events.
    ///
    /// Sample data is loaded asynchronously, [`EventDescription::get_sample_loading_state`] may be used to poll the loading state.
    pub fn load_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventDescription_LoadSampleData(self.inner.as_ptr()).to_result() }
    }

    /// Unloads all non-streaming sample data.
    ///
    /// Sample data will not be unloaded until all instances of the event are released.
    pub fn unload_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventDescription_UnloadSampleData(self.inner.as_ptr()).to_result() }
    }

    /// Retrieves the sample data loading state.
    ///
    /// If the event is invalid, then the returned state is [`LoadingState::Unloaded`] and this function returns [`FMOD_RESULT::FMOD_ERR_INVALID_HANDLE`].
    pub fn get_sample_loading_state(&self) -> Result<LoadingState> {
        let mut loading_state = 0;

        let error = unsafe {
            FMOD_Studio_EventDescription_GetSampleLoadingState(
                self.inner.as_ptr(),
                &mut loading_state,
            )
            .to_error()
        };

        LoadingState::try_from_ffi(loading_state, error)
    }
}
