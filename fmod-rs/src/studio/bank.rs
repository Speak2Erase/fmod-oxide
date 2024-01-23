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

use std::ffi::c_int;

use super::{Bus, LoadingState};
use fmod_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Bank {
    pub(crate) inner: *mut FMOD_STUDIO_BANK,
}

unsafe impl Send for Bank {}
unsafe impl Sync for Bank {}

impl From<*mut FMOD_STUDIO_BANK> for Bank {
    fn from(value: *mut FMOD_STUDIO_BANK) -> Self {
        Bank { inner: value }
    }
}

impl From<Bank> for *mut FMOD_STUDIO_BANK {
    fn from(value: Bank) -> Self {
        value.inner
    }
}

impl Bank {
    /// This function may be used to check the loading state of a bank which has been loaded asynchronously using the [`super::LoadBankFlags::NONBLOCKING`] flag,
    /// or is pending unload following a call to [`Bank::unload`].
    ///
    /// If an asynchronous load failed due to a file error state will contain [`LoadingState::Error`] and the return code from this function will be the error code of the bank load function.
    // TODO: make LoadingState contain the error?
    pub fn get_loading_state(&self) -> (LoadingState, Option<Error>) {
        let mut loading_state = 0;
        let error =
            unsafe { FMOD_Studio_Bank_GetLoadingState(self.inner, &mut loading_state).to_error() };

        (loading_state.into(), error)
    }

    /// Use this function to preload sample data ahead of time so that the events in the bank can play immediately when started.
    ///
    /// This function is equivalent to calling [`super::EventDescription::load_sample_data`] for all events in the bank, including referenced events.
    pub fn load_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_Bank_LoadSampleData(self.inner).to_result() }
    }

    /// Unloads non-streaming sample data for all events in the bank.
    ///
    /// Sample data loading is reference counted and the sample data will remain loaded until unload requests corresponding to all load requests are made, or until the bank is unloaded.
    pub fn unload_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_Bank_UnloadSampleData(self.inner).to_result() }
    }

    /// Retrieves the loading state of the samples in the bank.
    ///
    /// May be used for tracking the status of the [`Bank::load_sample_data`] operation.
    ///
    /// If [`Bank::load_sample_data`] has not been called for the bank then this function will return [`LoadingState::Unloaded`] even though sample data may have been loaded by other API calls.
    pub fn get_sample_loading_state(&self) -> Result<LoadingState> {
        let mut loading_state = 0;
        unsafe {
            FMOD_Studio_Bank_GetLoadingState(self.inner, &mut loading_state).to_result()?;
        }
        Ok(loading_state.into())
    }

    /// Unloads the bank.
    ///
    /// This will destroy all objects created from the bank, unload all sample data inside the bank, and invalidate all API handles referring to the bank.
    ///
    /// If the bank was loaded from user-managed memory, e.g. by [`super::System::load_bank_pointer`], then the memory must not be freed until the unload has completed.
    /// Poll the loading state using [`Bank::get_loading_state`] or use the [`FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD`] system callback to determine when it is safe to free the memory.
    pub fn unload(self) -> Result<()> {
        unsafe { FMOD_Studio_Bank_Unload(self.inner).to_result() }
    }
}

impl Bank {
    /// Retrieves the number of buses in the bank.
    pub fn bus_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetBusCount(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the buses in the bank.
    pub fn get_bus_list(&self) -> Result<Vec<Bus>> {
        let expected_count = self.bus_count()?;
        let mut count = 0;
        let mut list = vec![
            Bus {
                inner: std::ptr::null_mut()
            };
            expected_count as usize
        ];

        unsafe {
            FMOD_Studio_Bank_GetBusList(
                self.inner,
                // bus is repr transparent and has the same layout as *mut FMOD_STUDIO_BUS, so this cast is ok
                list.as_mut_ptr().cast::<*mut FMOD_STUDIO_BUS>(),
                list.capacity() as c_int,
                &mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(list)
        }
    }
}
