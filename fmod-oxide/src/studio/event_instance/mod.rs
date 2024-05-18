// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

mod attributes_3d;
mod callback;
mod general;
mod parameters;
mod playback;
mod playback_properties;

pub(crate) use callback::event_callback_impl;
pub use callback::EventInstanceCallback;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct EventInstance {
    pub(crate) inner: *mut FMOD_STUDIO_EVENTINSTANCE,
}

unsafe impl Send for EventInstance {}
unsafe impl Sync for EventInstance {}

impl From<*mut FMOD_STUDIO_EVENTINSTANCE> for EventInstance {
    fn from(value: *mut FMOD_STUDIO_EVENTINSTANCE) -> Self {
        Self { inner: value }
    }
}

impl From<EventInstance> for *mut FMOD_STUDIO_EVENTINSTANCE {
    fn from(value: EventInstance) -> Self {
        value.inner
    }
}
