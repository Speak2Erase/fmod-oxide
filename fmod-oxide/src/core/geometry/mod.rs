// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

mod general;
mod polygons;
mod spatialization;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Geometry {
    pub(crate) inner: *mut FMOD_GEOMETRY,
}

unsafe impl Send for Geometry {}
unsafe impl Sync for Geometry {}

impl From<*mut FMOD_GEOMETRY> for Geometry {
    fn from(value: *mut FMOD_GEOMETRY) -> Self {
        Geometry { inner: value }
    }
}

impl From<Geometry> for *mut FMOD_GEOMETRY {
    fn from(value: Geometry) -> Self {
        value.inner
    }
}
