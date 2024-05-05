// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct DspConnection {
    pub(crate) inner: *mut FMOD_DSPCONNECTION,
}

unsafe impl Send for DspConnection {}
unsafe impl Sync for DspConnection {}

impl From<*mut FMOD_DSPCONNECTION> for DspConnection {
    fn from(value: *mut FMOD_DSPCONNECTION) -> Self {
        DspConnection { inner: value }
    }
}

impl From<DspConnection> for *mut FMOD_DSPCONNECTION {
    fn from(value: DspConnection) -> Self {
        value.inner
    }
}
