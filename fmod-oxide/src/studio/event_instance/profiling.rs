// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::c_uint, mem::MaybeUninit};

use fmod_sys::*;

use crate::studio::{EventInstance, MemoryUsage};

impl EventInstance {
    /// Retrieves the event CPU usage data.
    ///
    /// [`crate::InitFlags::PROFILE_ENABLE`] with [`crate::SystemBuilder::build`] is required to call this function.
    // TODO fmod core docs
    pub fn get_cpu_usage(&self) -> Result<(c_uint, c_uint)> {
        let mut exclusive = 0;
        let mut inclusive = 0;
        unsafe {
            FMOD_Studio_EventInstance_GetCPUUsage(self.inner, &mut exclusive, &mut inclusive)
                .to_result()?;
        }
        Ok((exclusive, inclusive))
    }

    /// Retrieves memory usage statistics.
    ///
    /// Memory usage statistics are only available in logging builds, in release builds the return value will contain zero for all values this function.
    pub fn get_memory_usage(&self) -> Result<MemoryUsage> {
        let mut memory_usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventInstance_GetMemoryUsage(self.inner, memory_usage.as_mut_ptr())
                .to_result()?;

            let memory_usage = memory_usage.assume_init().into();
            Ok(memory_usage)
        }
    }
}
