// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::studio::{Bank, LoadingState};

#[cfg(doc)]
use crate::studio::{EventDescription, LoadBankFlags, System};

impl Bank {
    /// This function may be used to check the loading state of a bank which has been loaded asynchronously using the [`LoadBankFlags::NONBLOCKING`] flag,
    /// or is pending unload following a call to [`Bank::unload`].
    ///
    /// If an asynchronous load failed due to a file error state will contain [`LoadingState::Error`] and the return code from this function will be the error code of the bank load function.
    pub fn get_loading_state(&self) -> Result<LoadingState> {
        let mut loading_state = 0;
        let error = unsafe {
            FMOD_Studio_Bank_GetLoadingState(self.inner.as_ptr(), &mut loading_state).to_error()
        };

        LoadingState::try_from_ffi(loading_state, error)
    }

    /// Use this function to preload sample data ahead of time so that the events in the bank can play immediately when started.
    ///
    /// This function is equivalent to calling [`EventDescription::load_sample_data`] for all events in the bank, including referenced events.
    pub fn load_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_Bank_LoadSampleData(self.inner.as_ptr()).to_result() }
    }

    /// Unloads non-streaming sample data for all events in the bank.
    ///
    /// Sample data loading is reference counted and the sample data will remain loaded until unload requests corresponding to all load requests are made, or until the bank is unloaded.
    pub fn unload_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_Bank_UnloadSampleData(self.inner.as_ptr()).to_result() }
    }

    /// Retrieves the loading state of the samples in the bank.
    ///
    /// May be used for tracking the status of the [`Bank::load_sample_data`] operation.
    ///
    /// If [`Bank::load_sample_data`] has not been called for the bank then this function will return [`LoadingState::Unloaded`] even though sample data may have been loaded by other API calls.
    pub fn get_sample_loading_state(&self) -> Result<LoadingState> {
        let mut loading_state = 0;
        let error = unsafe {
            FMOD_Studio_Bank_GetSampleLoadingState(self.inner.as_ptr(), &mut loading_state)
                .to_error()
        };
        LoadingState::try_from_ffi(loading_state, error)
    }

    /// Unloads the bank.
    ///
    /// This will destroy all objects created from the bank, unload all sample data inside the bank, and invalidate all API handles referring to the bank.
    ///
    /// If the bank was loaded from user-managed memory, e.g. by [`System::load_bank_pointer`], then the memory must not be freed until the unload has completed.
    /// Poll the loading state using [`Bank::get_loading_state`] or use the [`FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD`] system callback to determine when it is safe to free the memory.
    pub fn unload(self) -> Result<()> {
        // we don't deallocate userdata here because the system callback will take care of that for us
        unsafe { FMOD_Studio_Bank_Unload(self.inner.as_ptr()).to_result() }
    }
}
