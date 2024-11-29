// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{ThreadAffinity, ThreadType};
use fmod_sys::*;

pub mod priority {
    use fmod_sys::*;

    pub const PLATFORM_MIN: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_PLATFORM_MIN;
    pub const PLATFORM_MAX: FMOD_THREAD_PRIORITY =
        FMOD_THREAD_PRIORITY_PLATFORM_MAX as FMOD_THREAD_PRIORITY; // no idea why this is u32 (32768 fits in an i32)
    pub const DEFAULT: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_DEFAULT;
    pub const LOW: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_LOW;
    pub const MEDIUM: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_MEDIUM;
    pub const HIGH: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_HIGH;
    pub const VERY_HIGH: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_VERY_HIGH;
    pub const EXTREME: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_EXTREME;
    pub const CRITICAL: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_CRITICAL;
    pub const MIXER: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_MIXER;
    pub const FEEDER: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_FEEDER;
    pub const STREAM: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_STREAM;
    pub const FILE: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_FILE;
    pub const NONBLOCKING: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_NONBLOCKING;
    pub const RECORD: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_RECORD;
    pub const GEOMETRY: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_GEOMETRY;
    pub const PROFILER: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_PROFILER;
    pub const STUDIO_UPDATE: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_STUDIO_UPDATE;
    pub const STUDIO_LOAD_BANK: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_STUDIO_LOAD_BANK;
    pub const STUDIO_LOAD_SAMPLE: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_STUDIO_LOAD_SAMPLE;
}

pub mod stack_size {
    use fmod_sys::*;
    pub const DEFAULT: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_DEFAULT;
    pub const MIXER: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_MIXER;
    pub const FEEDER: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_FEEDER;
    pub const STREAM: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_STREAM;
    pub const FILE: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_FILE;
    pub const NONBLOCKING: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_NONBLOCKING;
    pub const RECORD: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_RECORD;
    pub const GEOMETRY: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_GEOMETRY;
    pub const PROFILER: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_PROFILER;
    pub const STUDIO_UPDATE: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_STUDIO_UPDATE;
    pub const STUDIO_LOAD_BANK: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_STUDIO_LOAD_BANK;
    pub const STUDIO_LOAD_SAMPLE: FMOD_THREAD_STACK_SIZE =
        FMOD_THREAD_STACK_SIZE_STUDIO_LOAD_SAMPLE;
}

/// Specify the affinity, priority and stack size for all FMOD created threads.
///
/// You must call this function for the chosen thread before that thread is created for the settings to take effect.
///
/// Affinity can be specified using one (or more) of the [`ThreadAffinity`] constants or by providing the bits explicitly, i.e. (1<<3) for logical core three (core affinity is zero based).
/// See platform documentation for details on the available cores for a given device.
///
/// Priority can be specified using one of the [`FMOD_THREAD_PRIORITY`] constants or by providing the value explicitly, i.e. (-2) for the lowest thread priority on Windows.
/// See platform documentation for details on the available priority values for a given operating system.
///
/// Stack size can be specified explicitly, however for each thread you should provide a size equal to or larger than the expected default or risk causing a stack overflow at runtime.
pub fn set_attributes(
    kind: ThreadType,
    affinity: ThreadAffinity,
    priority: FMOD_THREAD_PRIORITY,
    stack_size: FMOD_THREAD_STACK_SIZE,
) -> Result<()> {
    unsafe {
        FMOD_Thread_SetAttributes(kind.into(), affinity.into(), priority, stack_size).to_result()
    }
}
