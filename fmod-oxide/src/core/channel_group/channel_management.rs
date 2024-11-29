// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_int;

use crate::{Channel, ChannelGroup};

impl ChannelGroup {
    /// Retrieves the number of Channels that feed into to this group.
    pub fn get_channel_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_ChannelGroup_GetNumChannels(self.inner.as_ptr(), &mut count).to_result()? }
        Ok(count)
    }

    /// Retrieves the Channel at the specified index in the list of Channel inputs.
    pub fn get_channel(&self, index: c_int) -> Result<Channel> {
        let mut channel = std::ptr::null_mut();
        unsafe {
            FMOD_ChannelGroup_GetChannel(self.inner.as_ptr(), index, &mut channel).to_result()?;
        }
        Ok(channel.into())
    }
}
