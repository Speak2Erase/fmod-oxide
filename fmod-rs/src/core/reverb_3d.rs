// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Reverb3D {
    pub(crate) inner: *mut FMOD_REVERB3D,
}

unsafe impl Send for Reverb3D {}
unsafe impl Sync for Reverb3D {}

impl From<*mut FMOD_REVERB3D> for Reverb3D {
    fn from(value: *mut FMOD_REVERB3D) -> Self {
        Reverb3D { inner: value }
    }
}

impl From<Reverb3D> for *mut FMOD_REVERB3D {
    fn from(value: Reverb3D) -> Self {
        value.inner
    }
}
