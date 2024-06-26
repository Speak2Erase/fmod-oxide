// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

mod channel_format;
mod connections;
mod general;
mod metering;
mod parameters;
mod processing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Dsp {
    pub(crate) inner: *mut FMOD_DSP,
}

unsafe impl Send for Dsp {}
unsafe impl Sync for Dsp {}

impl From<*mut FMOD_DSP> for Dsp {
    fn from(value: *mut FMOD_DSP) -> Self {
        Dsp { inner: value }
    }
}

impl From<Dsp> for *mut FMOD_DSP {
    fn from(value: Dsp) -> Self {
        value.inner
    }
}
