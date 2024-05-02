// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod channel_group;
pub use channel_group::*;

mod system;
pub use system::*;

mod sound;
pub use sound::*;

mod dsp;
pub use dsp::*;

mod flags;
pub use flags::*;

mod enums;
pub use enums::*;

mod structs;
pub use structs::*;

pub mod thread {
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
        pub const STUDIO_LOAD_SAMPLE: FMOD_THREAD_PRIORITY =
            FMOD_THREAD_PRIORITY_STUDIO_LOAD_SAMPLE;
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
        pub const STUDIO_LOAD_BANK: FMOD_THREAD_STACK_SIZE =
            FMOD_THREAD_STACK_SIZE_STUDIO_LOAD_BANK;
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
            FMOD_Thread_SetAttributes(kind.into(), affinity.into(), priority, stack_size)
                .to_result()
        }
    }
}

pub mod file {
    use fmod_sys::*;

    /// Information function to retrieve the state of FMOD disk access.
    ///
    /// Do not use this function to synchronize your own reads with, as due to timing,
    /// you might call this function and it says false = it is not busy,
    /// but the split second after calling this function, internally FMOD might set it to busy.
    /// Use [`get_disk_busy`] for proper mutual exclusion as it uses semaphores.
    pub fn get_disk_busy() -> Result<bool> {
        let mut busy = 0;
        unsafe {
            FMOD_File_GetDiskBusy(&mut busy).to_result()?;
        }
        Ok(busy > 0)
    }

    /// Sets the busy state for disk access ensuring mutual exclusion of file operations.
    ///
    /// If file IO is currently being performed by FMOD this function will block until it has completed.
    ///
    /// This function should be called in pairs once to set the state, then again to clear it once complete.
    pub fn set_disk_busy(busy: bool) -> Result<()> {
        unsafe { FMOD_File_SetDiskBusy(std::ffi::c_int::from(busy)).to_result() }
    }
}

pub mod debug {
    use fmod_sys::*;
    use lanyard::Utf8CString;

    use std::ffi::{c_char, c_int};

    #[derive(PartialEq, Eq, Debug)]
    pub enum DebugMode {
        TTY,
        File(Utf8CString),
        Callback(
            unsafe extern "C" fn(
                FMOD_DEBUG_FLAGS,
                *const c_char,
                c_int,
                *const c_char,
                *const c_char,
            ) -> FMOD_RESULT,
        ),
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct DebugFlags: FMOD_DEBUG_FLAGS {
            const NONE = FMOD_DEBUG_LEVEL_NONE;
            const ERROR = FMOD_DEBUG_LEVEL_ERROR;
            const WARNING = FMOD_DEBUG_LEVEL_WARNING;
            const LOG = FMOD_DEBUG_LEVEL_LOG;
            const MEMORY = FMOD_DEBUG_TYPE_MEMORY;
            const FILE = FMOD_DEBUG_TYPE_FILE;
            const CODEC = FMOD_DEBUG_TYPE_CODEC;
            const TRACE = FMOD_DEBUG_TYPE_TRACE;
            const DISPLAY_TIMESTAMPS = FMOD_DEBUG_DISPLAY_TIMESTAMPS;
            const DISPLAY_LINENUMBERS = FMOD_DEBUG_DISPLAY_LINENUMBERS;
            const DISPLAY_THREAD = FMOD_DEBUG_DISPLAY_THREAD;
        }
    }

    impl From<FMOD_DEBUG_FLAGS> for DebugFlags {
        fn from(value: FMOD_DEBUG_FLAGS) -> Self {
            DebugFlags::from_bits_truncate(value)
        }
    }

    impl From<DebugFlags> for FMOD_DEBUG_FLAGS {
        fn from(value: DebugFlags) -> Self {
            value.bits()
        }
    }

    /// Specify the level and delivery method of log messages when using the logging version of FMOD.
    ///
    /// This function will return [`FMOD_RESULT::FMOD_ERR_UNSUPPORTED`] when using the non-logging (release) versions of FMOD.
    /// The logging version of FMOD can be recognized by the 'L' suffix in the library name, fmodL.dll or libfmodL.so for instance.
    ///
    /// By default this crate links against non-logging versions of FMOD in release builds.
    /// This behaviour can be changed with the "fmod-sys/force-debug" feature.
    ///
    /// Note that:
    ///     [`DebugFlags::LOG`] produces informational, warning and error messages.
    ///     [`DebugFlags::WARNING`] produces warnings and error messages.
    ///     [`DebugFlags::ERROR`] produces error messages only.
    pub fn initialize(flags: DebugFlags, mode: DebugMode) -> Result<()> {
        match mode {
            DebugMode::TTY => unsafe {
                FMOD_Debug_Initialize(
                    flags.into(),
                    FMOD_DEBUG_MODE_FMOD_DEBUG_MODE_TTY,
                    None,
                    std::ptr::null(),
                )
                .to_result()
            },
            DebugMode::File(file) => unsafe {
                FMOD_Debug_Initialize(
                    flags.into(),
                    FMOD_DEBUG_MODE_FMOD_DEBUG_MODE_FILE,
                    None,
                    file.as_ptr(),
                )
                .to_result()
            },
            DebugMode::Callback(c) => unsafe {
                FMOD_Debug_Initialize(
                    flags.into(),
                    FMOD_DEBUG_MODE_FMOD_DEBUG_MODE_CALLBACK,
                    Some(c),
                    std::ptr::null(),
                )
                .to_result()
            },
        }
    }
}

pub mod memory {
    use fmod_sys::*;
    use std::ffi::{c_char, c_int, c_uint, c_void};

    #[derive(PartialEq, Eq, Debug)]
    pub enum MemoryType {
        Pool(&'static mut [u8]),
        Callback {
            alloc: unsafe extern "C" fn(
                size: c_uint,
                type_: FMOD_MEMORY_TYPE,
                sourcestr: *const c_char,
            ) -> *mut c_void,
            realloc: FMOD_MEMORY_REALLOC_CALLBACK,
            free: unsafe extern "C" fn(
                ptr: *mut c_void,
                type_: FMOD_MEMORY_TYPE,
                sourcestr: *const c_char,
            ),
        },
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct MemoryFlags: FMOD_MEMORY_TYPE {
          const NORMAL         = FMOD_MEMORY_NORMAL;
          const STREAM_FILE    = FMOD_MEMORY_STREAM_FILE;
          const STREAM_DECODE  = FMOD_MEMORY_STREAM_DECODE;
          const SAMPLEDATA     = FMOD_MEMORY_SAMPLEDATA;
          const DSP_BUFFER     = FMOD_MEMORY_DSP_BUFFER;
          const PLUGIN         = FMOD_MEMORY_PLUGIN;
          const PERSISTENT     = FMOD_MEMORY_PERSISTENT;
          const ALL            = FMOD_MEMORY_ALL;
        }
    }

    impl From<FMOD_MEMORY_TYPE> for MemoryFlags {
        fn from(value: FMOD_MEMORY_TYPE) -> Self {
            MemoryFlags::from_bits_truncate(value)
        }
    }

    impl From<MemoryFlags> for FMOD_MEMORY_TYPE {
        fn from(value: MemoryFlags) -> Self {
            value.bits()
        }
    }

    /// Specifies a method for FMOD to allocate and free memory, either through user supplied callbacks or through a user supplied memory buffer with a fixed size.
    ///
    /// # Safety
    ///
    /// This function must be called before any FMOD System object is created.
    ///
    /// If [`MemoryType::Callback::alloc`] and [`MemoryType::Callback::free`] are provided without [`MemoryType::Callback::realloc`]
    /// the reallocation is implemented via an allocation of the new size, copy from old address to new, then a free of the old address.
    ///
    /// Callback implementations must be thread safe.
    ///
    /// If you specify a fixed size pool that is too small, FMOD will return [`FMOD_RESULT::FMOD_ERR_MEMORY`] when the limit of the fixed size pool is exceeded.
    /// At this point, it's possible that FMOD may become unstable. To maintain stability, do not allow FMOD to run out of memory.
    /// To find out the required fixed size call [`memory_initialize`] with an overly large pool size (or no pool) and find out the maximum RAM usage at any one time with [`memory_get_stats`].
    /// The size of the pool is limited to [`c_int::MAX`].
    pub unsafe fn memory_initialize(memory_type: MemoryType, flags: MemoryFlags) -> Result<()> {
        match memory_type {
            MemoryType::Pool(pool) => unsafe {
                FMOD_Memory_Initialize(
                    pool.as_mut_ptr().cast(),
                    pool.len() as c_int,
                    None,
                    None,
                    None,
                    flags.into(),
                )
                .to_result()
            },
            MemoryType::Callback {
                alloc,
                realloc,
                free,
            } => unsafe {
                FMOD_Memory_Initialize(
                    std::ptr::null_mut(),
                    0,
                    Some(alloc),
                    realloc,
                    Some(free),
                    flags.into(),
                )
                .to_result()
            },
        }
    }

    /// Returns information on the memory usage of FMOD.
    ///
    /// This information is byte accurate and counts all allocs and frees internally.
    /// This is useful for determining a fixed memory size to make FMOD work within for fixed memory machines such as consoles.
    ///
    /// Note that if using [`memory_initialize`], the memory usage will be slightly higher than without it, as FMOD has to have a small amount of memory overhead to manage the available memory.
    pub fn memory_get_stats(blocking: bool) -> Result<(c_int, c_int)> {
        let mut current = 0;
        let mut max = 0;
        unsafe {
            FMOD_Memory_GetStats(&mut current, &mut max, blocking.into()).to_result()?;
        }
        Ok((current, max))
    }
}
