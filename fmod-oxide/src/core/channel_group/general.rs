// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::c_int;

use crate::ChannelGroup;

impl ChannelGroup {
    /// Retrieves the name set when the group was created.
    pub fn get_name(&self) -> Result<Utf8CString> {
        let mut name = [0_i8; 512];
        unsafe {
            FMOD_ChannelGroup_GetName(self.inner, name.as_mut_ptr(), name.len() as c_int)
                .to_result()?;

            // FIXME is this right?
            let name = name
                .into_iter()
                .take_while(|&v| v != 0)
                .map(|v| v as u8)
                .collect();
            let name = Utf8CString::from_utf8_with_nul_unchecked(name);
            Ok(name)
        }
    }

    /// Frees the memory for the group.
    ///
    /// Any [`Channel`]s or [`ChannelGroup`]s feeding into this group are moved to the master [`ChannelGroup`].
    pub fn release(&self) -> Result<()> {
        unsafe { FMOD_ChannelGroup_Release(self.inner).to_result() }
    }
}
