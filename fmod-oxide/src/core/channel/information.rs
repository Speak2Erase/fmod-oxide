// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_int;

use crate::{Channel, Sound};

impl Channel {
    /// Retrieves whether the Channel is being emulated by the virtual voice system.
    ///
    /// See the Virtual Voices guide for more information.
    pub fn is_virtual(&self) -> Result<bool> {
        let mut is_virtual = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Channel_IsVirtual(self.inner.as_ptr(), &mut is_virtual).to_result()?;
        }
        Ok(is_virtual.into())
    }

    /// Retrieves the currently playing [`Sound`].
    ///
    /// May return [`None`] if no [`Sound`] is playing.
    pub fn get_current_sound(&self) -> Result<Option<Sound>> {
        let mut sound = std::ptr::null_mut();
        unsafe {
            FMOD_Channel_GetCurrentSound(self.inner.as_ptr(), &mut sound).to_result()?;
        }
        Ok(if sound.is_null() {
            None
        } else {
            Some(sound.into())
        })
    }

    /// Retrieves the index of this object in the System Channel pool.
    pub fn get_index(&self) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_Channel_GetIndex(self.inner.as_ptr(), &mut index).to_result()?;
        }
        Ok(index)
    }
}
