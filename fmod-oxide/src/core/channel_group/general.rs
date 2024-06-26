// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::c_int;

use crate::{get_string, ChannelGroup};

impl ChannelGroup {
    /// Retrieves the name set when the group was created.
    pub fn get_name(&self) -> Result<Utf8CString> {
        unsafe {
            get_string(|name| {
                FMOD_ChannelGroup_GetName(self.inner, name.as_mut_ptr().cast(), name.len() as c_int)
            })
        }
    }

    /// Frees the memory for the group.
    ///
    /// Any [`Channel`]s or [`ChannelGroup`]s feeding into this group are moved to the master [`ChannelGroup`].
    pub fn release(&self) -> Result<()> {
        // release userdata
        #[cfg(feature = "userdata-abstraction")]
        let userdata = self.get_raw_userdata()?;

        unsafe {
            FMOD_ChannelGroup_Release(self.inner).to_result()?;
        }

        // release/remove userdata if it is not null
        #[cfg(feature = "userdata-abstraction")]
        if !userdata.is_null() {
            crate::userdata::remove_userdata(userdata.into());
            self.set_raw_userdata(std::ptr::null_mut())?;
        }

        Ok(())
    }
}
