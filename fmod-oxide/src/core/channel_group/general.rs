// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::c_int;

use crate::{get_string, ChannelGroup};

#[cfg(doc)]
use crate::Channel;

impl ChannelGroup {
    /// Retrieves the name set when the group was created.
    pub fn get_name(&self) -> Result<Utf8CString> {
        unsafe {
            get_string(|name| {
                FMOD_ChannelGroup_GetName(
                    self.inner.as_ptr(),
                    name.as_mut_ptr().cast(),
                    name.len() as c_int,
                )
            })
        }
    }

    /// Frees the memory for the group.
    ///
    /// Any [`Channel`]s or [`ChannelGroup`]s feeding into this group are moved to the master [`ChannelGroup`].
    ///
    /// # Safety
    ///
    /// According to the FMOD documentation, [`ChannelGroup`]s are actual pointers, rather than a handle.
    /// After a [`ChannelGroup`] is released it is no longer safe to use!
    pub unsafe fn release(&self) -> Result<()> {
        unsafe { FMOD_ChannelGroup_Release(self.inner.as_ptr()).to_result() }
    }
}
