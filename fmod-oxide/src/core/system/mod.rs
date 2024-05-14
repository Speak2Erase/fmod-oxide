// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

mod builder;
mod creation;
mod device_selection;
mod filesystem;
mod general;
mod geometry;
mod information;
mod lifetime;
mod network;
mod plugin;
mod recording;
mod runtime_control;
mod setup;
pub use builder::SystemBuilder;
pub use setup::RolloffCallback;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct System {
    pub(crate) inner: *mut FMOD_SYSTEM,
}

unsafe impl Send for System {}
unsafe impl Sync for System {}

impl From<*mut FMOD_SYSTEM> for System {
    fn from(value: *mut FMOD_SYSTEM) -> Self {
        System { inner: value }
    }
}

impl From<System> for *mut FMOD_SYSTEM {
    fn from(value: System) -> Self {
        value.inner
    }
}
