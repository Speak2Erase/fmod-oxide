// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod bank;
mod builder;
mod callback;
mod command_replay;
mod general;
mod lifecycle;
mod listener;
mod misc;
mod parameter;
mod plugins;
mod profiling; // things too small to really make their own module

pub use builder::SystemBuilder;
pub use callback::SystemCallback;

/// The main system object for FMOD Studio.
///
/// Initializing the FMOD Studio System object will also initialize the core System object.
///
/// Created with [`SystemBuilder`], which handles initialization for you.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)] // TODO: should this logically be copy?
#[repr(transparent)] // so we can transmute between types
pub struct System {
    pub(crate) inner: NonNull<FMOD_STUDIO_SYSTEM>,
}

// TODO tryfrom impls

impl From<*mut FMOD_STUDIO_SYSTEM> for System {
    fn from(inner: *mut FMOD_STUDIO_SYSTEM) -> Self {
        let inner = NonNull::new(inner).unwrap();
        Self { inner }
    }
}

/// Convert a System instance to its FFI equivalent.
///
/// This is safe, provided you don't use the pointer.
impl From<System> for *mut FMOD_STUDIO_SYSTEM {
    fn from(value: System) -> Self {
        value.inner.as_ptr()
    }
}

/// Most of FMOD is thread safe.
/// There are some select functions that are not thread safe to call, those are marked as unsafe.
unsafe impl Send for System {}
unsafe impl Sync for System {}
