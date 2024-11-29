// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::{ops::Deref, ptr::NonNull};

use crate::ChannelControl;

mod information;
mod playback_control;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Channel {
    pub(crate) inner: NonNull<FMOD_CHANNEL>,
}

unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}

impl From<*mut FMOD_CHANNEL> for Channel {
    fn from(value: *mut FMOD_CHANNEL) -> Self {
        let inner = NonNull::new(value).unwrap();
        Channel { inner }
    }
}

impl From<Channel> for *mut FMOD_CHANNEL {
    fn from(value: Channel) -> Self {
        value.inner.as_ptr()
    }
}

impl Deref for Channel {
    type Target = ChannelControl;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        unsafe {
            // perform a debug check to ensure that the the cast results in the same pointer
            let control = FMOD_Channel_CastToControl(self.inner.as_ptr());
            assert_eq!(
                control as usize,
                self.inner.as_ptr() as usize,
                "ChannelControl cast was not equivalent! THIS IS A MAJOR BUG. PLEASE REPORT THIS!"
            );
        }
        // channelcontrol has the same layout as channel, and if the assumption in channel_control.rs is correct,
        // this is cast is safe.
        unsafe { &*std::ptr::from_ref(self).cast::<ChannelControl>() }
    }
}
