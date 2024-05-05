// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::Channel;

// FMOD's C API provides two versions of functions for channels: one that takes a `*mut FMOD_CHANNEL` and one that takes a `*mut FMOD_CHANNELGROUP`.
// The C++ API provides a base class `ChannelControl` that `Channel` and `ChannelGroup` inherits from.
// Seeing as we can cast from FMOD_CHANNELCONTROL to Channel* (in c++) we should be able to cast from FMOD_CHANNEL(GROUP) to FMOD_CHANNELCONTROL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct ChannelControl {
    pub(crate) inner: *mut FMOD_CHANNELCONTROL,
    // FIXME: if the above assumption is invalid, could we possibly add a bool to track if this is a Channel or ChannelGroup?
    // there's no real way to get a ChannelControl from FMOD's C API, this is a pure rust construct specific to this api,
    // so it would be feasible
}

unsafe impl Send for ChannelControl {}
unsafe impl Sync for ChannelControl {}

impl From<*mut FMOD_CHANNEL> for ChannelControl {
    fn from(value: *mut FMOD_CHANNEL) -> Self {
        ChannelControl {
            inner: value.cast(),
        }
    }
}

impl From<*mut FMOD_CHANNELCONTROL> for ChannelControl {
    fn from(value: *mut FMOD_CHANNELCONTROL) -> Self {
        ChannelControl {
            inner: value.cast(),
        }
    }
}

impl From<Channel> for ChannelControl {
    fn from(value: Channel) -> Self {
        ChannelControl {
            inner: value.inner.cast(),
        }
    }
}

impl From<ChannelControl> for *mut FMOD_CHANNELCONTROL {
    fn from(value: ChannelControl) -> Self {
        value.inner.cast()
    }
}
