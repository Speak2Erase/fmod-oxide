// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

mod attributes;
mod callback;
mod general;
mod instance;
mod parameter;
mod sample_data;
mod user_property;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct EventDescription {
    pub(crate) inner: *mut FMOD_STUDIO_EVENTDESCRIPTION,
}

unsafe impl Send for EventDescription {}
unsafe impl Sync for EventDescription {}

impl From<*mut FMOD_STUDIO_EVENTDESCRIPTION> for EventDescription {
    fn from(inner: *mut FMOD_STUDIO_EVENTDESCRIPTION) -> Self {
        Self { inner }
    }
}

impl From<EventDescription> for *mut FMOD_STUDIO_EVENTDESCRIPTION {
    fn from(value: EventDescription) -> Self {
        value.inner
    }
}
