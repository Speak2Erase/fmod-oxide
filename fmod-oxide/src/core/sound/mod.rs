// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

mod information;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Sound {
    pub(crate) inner: *mut FMOD_SOUND,
}

unsafe impl Send for Sound {}
unsafe impl Sync for Sound {}

impl From<*mut FMOD_SOUND> for Sound {
    fn from(value: *mut FMOD_SOUND) -> Self {
        Sound { inner: value }
    }
}

impl From<Sound> for *mut FMOD_SOUND {
    fn from(value: Sound) -> Self {
        value.inner
    }
}
