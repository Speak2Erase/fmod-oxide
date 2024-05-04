// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Channel {
    pub(crate) inner: *mut FMOD_CHANNEL,
}

unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}

impl From<*mut FMOD_CHANNEL> for Channel {
    fn from(value: *mut FMOD_CHANNEL) -> Self {
        Channel { inner: value }
    }
}

impl From<Channel> for *mut FMOD_CHANNEL {
    fn from(value: Channel) -> Self {
        value.inner
    }
}
