// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::mem::MaybeUninit;

use crate::studio::{BufferUsage, CpuUsage, MemoryUsage, System};

#[cfg(doc)]
use crate::studio::SystemBuilder;

impl System {
    /// Retrieves buffer usage information.
    ///
    /// Stall count and time values are cumulative. They can be reset by calling [`System::reset_buffer_usage`].
    ///
    /// Stalls due to the studio command queue overflowing can be avoided by setting a larger command queue size with [`SystemBuilder::settings`].
    pub fn get_buffer_usage(&self) -> Result<BufferUsage> {
        let mut usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetBufferUsage(self.inner, usage.as_mut_ptr()).to_result()?;

            let usage = usage.assume_init().into();
            Ok(usage)
        }
    }

    /// Resets memory buffer usage statistics.
    ///
    /// This function resets the buffer usage data tracked by the FMOD Studio System.
    pub fn reset_buffer_usage(&self) -> Result<()> {
        unsafe { FMOD_Studio_System_ResetBufferUsage(self.inner).to_result() }
    }

    /// Retrieves the amount of CPU used for different parts of the Studio engine.
    ///
    /// For readability, the percentage values are smoothed to provide a more stable output.
    pub fn get_cpu_usage(&self) -> Result<(CpuUsage, crate::CpuUsage)> {
        let mut usage = MaybeUninit::zeroed();
        let mut usage_core = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetCPUUsage(self.inner, usage.as_mut_ptr(), usage_core.as_mut_ptr())
                .to_result()?;

            let usage = usage.assume_init().into();
            let usage_core = usage_core.assume_init().into();
            Ok((usage, usage_core))
        }
    }

    /// Retrieves memory usage statistics.
    ///
    /// The memory usage `sample_data` field for the system is the total size of non-streaming sample data currently loaded.
    ///
    /// Memory usage statistics are only available in logging builds, in release builds memoryusage will contain zero for all values after calling this function.
    pub fn get_memory_usage(&self) -> Result<MemoryUsage> {
        let mut usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetMemoryUsage(self.inner, usage.as_mut_ptr()).to_result()?;

            let usage = usage.assume_init().into();
            Ok(usage)
        }
    }
}
