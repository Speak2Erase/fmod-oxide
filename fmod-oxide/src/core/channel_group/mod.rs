// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ops::Deref;

use crate::ChannelControl;

mod channel_management;
mod general;
mod group_management;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Deref for ChannelGroup {
    type Target = ChannelControl;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        unsafe {
            // perform a debug check to ensure that the the cast results in the same pointer
            let control = FMOD_ChannelGroup_CastToControl(self.inner);
            assert_eq!(
                control as usize, self.inner as usize,
                "ChannelControl cast was not equivalent! THIS IS A MAJOR BUG. PLEASE REPORT THIS!"
            );
        }
        // channelcontrol has the same layout as channel, and if the assumption in channel_control.rs is correct,
        // this is cast is safe.
        unsafe { &*std::ptr::from_ref(self).cast::<ChannelControl>() }
    }
}
