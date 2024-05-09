// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::c_int;

use crate::{SoundGroup, System};

impl SoundGroup {
    /// Retrieves the name of the sound group.
    pub fn get_name(&self) -> Result<Utf8CString> {
        let mut name = [0_i8; 512];
        unsafe {
            FMOD_SoundGroup_GetName(self.inner, name.as_mut_ptr(), name.len() as c_int)
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

    /// Releases a soundgroup object and returns all sounds back to the master sound group.
    ///
    /// You cannot release the master [`SoundGroup`].
    pub fn release(&self) -> Result<()> {
        unsafe { FMOD_SoundGroup_Release(self.inner).to_result() }
    }

    // TODO userdata

    /// Retrieves the parent System object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_SoundGroup_GetSystemObject(self.inner, &mut system).to_result()? };
        Ok(system.into())
    }
}
