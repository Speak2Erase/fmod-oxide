// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod general;
mod mix_properties;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct DspConnection {
    pub(crate) inner: NonNull<FMOD_DSPCONNECTION>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for DspConnection {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for DspConnection {}

impl From<*mut FMOD_DSPCONNECTION> for DspConnection {
    fn from(value: *mut FMOD_DSPCONNECTION) -> Self {
        let inner = NonNull::new(value).unwrap();
        DspConnection { inner }
    }
}

impl From<DspConnection> for *mut FMOD_DSPCONNECTION {
    fn from(value: DspConnection) -> Self {
        value.inner.as_ptr()
    }
}
