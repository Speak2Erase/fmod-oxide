// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod data_reading;
mod defaults;
mod general;
mod information;
mod music;
mod relationship;
mod synchronization;
pub use synchronization::SyncPoint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Sound {
    pub(crate) inner: NonNull<FMOD_SOUND>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for Sound {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for Sound {}

impl From<*mut FMOD_SOUND> for Sound {
    fn from(value: *mut FMOD_SOUND) -> Self {
        let inner = NonNull::new(value).unwrap();
        Sound { inner }
    }
}

impl From<Sound> for *mut FMOD_SOUND {
    fn from(value: Sound) -> Self {
        value.inner.as_ptr()
    }
}
