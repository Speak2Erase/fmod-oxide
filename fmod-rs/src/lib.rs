//! fmod-rs
//!
//! Safe rust bindings to the FMOD sound engine.
//! This crate tries to be as rusty as possible, without comprimising on any APIs.
//! Certain APIs, such as loading banks from a pointer, are marked as unsafe, but are still available for use.
//!
//! Most documentation is copied directly from the FMOD docs, however some information (such as parameter values) are excluded.
//! Please refer to the FMOD documentation for more usage information.
//!
//! # Memory management & Copy types
//! TODO
//!
//! # Userdata
//! TODO

#![warn(rust_2018_idioms, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    missing_docs, // TODO: disable later
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
    clippy::missing_panics_doc // TODO: disable later
)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub use fmod_sys as ffi;
pub use fmod_sys::{error_code_to_str, Error, Result};

pub mod studio;

pub mod core;
pub use core::*;
use std::os::raw::c_int;

mod common;
pub use common::*;

mod userdata;
pub use userdata::UserdataTypes;

// relatively common bound
pub trait Shareable: Send + Sync + 'static {}
impl<T> Shareable for T where T: Send + Sync + 'static {}

use ffi::*;
use std::ffi::{c_char, c_uint, c_void, CString};

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
    pub struct MemoryFlags: c_uint {
        const NORMAL = FMOD_MEMORY_NORMAL;
        const FILE_FLUSH = FMOD_STUDIO_COMMANDCAPTURE_FILEFLUSH;
        const SKIP_INITIAL_STATE = FMOD_STUDIO_COMMANDCAPTURE_SKIP_INITIAL_STATE;
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

#[derive(PartialEq, Eq, Debug)]
pub enum DebugMode {
    TTY,
    File(String),
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
    pub struct DebugFlags: c_uint {
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
pub fn debug_initialize(flags: DebugFlags, mode: DebugMode) -> Result<()> {
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
        DebugMode::File(c) => unsafe {
            let file = CString::new(c)?;
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
