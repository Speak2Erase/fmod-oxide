// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct ChannelGroup {
    pub(crate) inner: *mut FMOD_CHANNELGROUP,
}

unsafe impl Send for ChannelGroup {}
unsafe impl Sync for ChannelGroup {}

impl From<*mut FMOD_CHANNELGROUP> for ChannelGroup {
    fn from(value: *mut FMOD_CHANNELGROUP) -> Self {
        ChannelGroup { inner: value }
    }
}

impl From<ChannelGroup> for *mut FMOD_CHANNELGROUP {
    fn from(value: ChannelGroup) -> Self {
        value.inner
    }
}
