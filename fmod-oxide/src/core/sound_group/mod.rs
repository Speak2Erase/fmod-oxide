// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod general;
mod group;
mod sound;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct SoundGroup {
    pub(crate) inner: NonNull<FMOD_SOUNDGROUP>,
}

unsafe impl Send for SoundGroup {}
unsafe impl Sync for SoundGroup {}

impl From<*mut FMOD_SOUNDGROUP> for SoundGroup {
    fn from(value: *mut FMOD_SOUNDGROUP) -> Self {
        let inner = NonNull::new(value).unwrap();
        SoundGroup { inner }
    }
}

impl From<SoundGroup> for *mut FMOD_SOUNDGROUP {
    fn from(value: SoundGroup) -> Self {
        value.inner.as_ptr()
    }
}
