// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_int;

use crate::{ChannelGroup, DspConnection};

impl ChannelGroup {
    /// Adds a [`ChannelGroup`] as an input to this group.
    pub fn add_group(
        &self,
        group: ChannelGroup,
        propgate_dsp_clock: bool,
    ) -> Result<DspConnection> {
        let mut dsp_connection = std::ptr::null_mut();
        unsafe {
            FMOD_ChannelGroup_AddGroup(
                self.inner,
                group.inner,
                propgate_dsp_clock.into(),
                &mut dsp_connection,
            )
            .to_result()?;
        };
        Ok(dsp_connection.into())
    }

    /// Retrieves the number of [`ChannelGroup`]s that feed into to this group.
    pub fn get_group_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_ChannelGroup_GetNumGroups(self.inner, &mut count).to_result()? }
        Ok(count)
    }

    /// Retrieves the [`ChannelGroup`] at the specified index in the list of group inputs.
    pub fn get_group(&self, index: c_int) -> Result<ChannelGroup> {
        let mut group = std::ptr::null_mut();
        unsafe { FMOD_ChannelGroup_GetGroup(self.inner, index, &mut group).to_result()? }
        Ok(group.into())
    }

    /// Retrieves the [`ChannelGroup`] this object outputs to.
    pub fn get_parent_group(&self) -> Result<ChannelGroup> {
        let mut channel_group = std::ptr::null_mut();
        unsafe {
            FMOD_ChannelGroup_GetParentGroup(self.inner, &mut channel_group).to_result()?;
        }
        // FIXME: what if this is null? if it can be, what about other places we return pointers like this?
        // do we even need to worry about this issue? we aren't returning references...
        Ok(channel_group.into())
    }
}
