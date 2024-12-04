// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod callback;
mod channel_format;
mod connections;
mod general;
mod metering;
mod parameters;
mod processing;

pub use parameters::{DataParameterType, ParameterType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Dsp {
    pub(crate) inner: NonNull<FMOD_DSP>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for Dsp {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for Dsp {}

impl From<*mut FMOD_DSP> for Dsp {
    fn from(value: *mut FMOD_DSP) -> Self {
        let inner = NonNull::new(value).unwrap();
        Dsp { inner }
    }
}

impl From<Dsp> for *mut FMOD_DSP {
    fn from(value: Dsp) -> Self {
        value.inner.as_ptr()
    }
}
