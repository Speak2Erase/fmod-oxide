// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_int;

use crate::studio::{EventDescription, EventInstance};

#[cfg(doc)]
use crate::studio::Bank;

impl EventDescription {
    /// Creates a playable instance.
    ///
    /// When an event instance is created, any required non-streaming sample data is loaded asynchronously.
    ///
    /// Use [`EventDescription::get_sample_loading_state`] to check the loading status.
    ///
    /// Sample data can be loaded ahead of time with [`EventDescription::load_sample_data`] or [`Bank::load_sample_data`]. See Sample Data Loading for more information.
    pub fn create_instance(&self) -> Result<EventInstance> {
        let mut instance = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventDescription_CreateInstance(self.inner.as_ptr(), &mut instance)
                .to_result()?;
            Ok(EventInstance::from(instance))
        }
    }

    /// Retrieves the number of instances.
    pub fn instance_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetInstanceCount(self.inner.as_ptr(), &mut count)
                .to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the instances.
    pub fn get_instance_list(&self) -> Result<Vec<EventInstance>> {
        let expected_count = self.instance_count()?;
        let mut count = 0;
        let mut list = vec![std::ptr::null_mut(); expected_count as usize];

        unsafe {
            FMOD_Studio_EventDescription_GetInstanceList(
                self.inner.as_ptr(),
                // eventinstance is repr transparent and has the same layout as *mut FMOD_STUDIO_EVENTINSTANCE, so this cast is ok
                list.as_mut_ptr(),
                list.capacity() as c_int,
                &mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            // *mut FMOD_STUDIO_EVENTINSTANCE is transmutable to EventInstance
            Ok(std::mem::transmute::<
                Vec<*mut fmod_sys::FMOD_STUDIO_EVENTINSTANCE>,
                Vec<EventInstance>,
            >(list))
        }
    }

    /// Releases all instances.
    ///
    /// This function immediately stops and releases all instances of the event.
    pub fn release_all_instances(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventDescription_ReleaseAllInstances(self.inner.as_ptr()).to_result() }
    }
}
