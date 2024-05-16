// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_int, c_uint};

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};

use crate::{get_string, Sound, TimeUnit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct SyncPoint {
    pub(crate) inner: *mut FMOD_SYNCPOINT,
}

unsafe impl Send for SyncPoint {}
unsafe impl Sync for SyncPoint {}

impl From<*mut FMOD_SYNCPOINT> for SyncPoint {
    fn from(value: *mut FMOD_SYNCPOINT) -> Self {
        SyncPoint { inner: value }
    }
}

impl From<SyncPoint> for *mut FMOD_SYNCPOINT {
    fn from(value: SyncPoint) -> Self {
        value.inner
    }
}

impl Sound {
    /// Retrieve a sync point.
    ///
    /// For for more information on sync points see Sync Points.
    pub fn get_sync_point(&self, index: i32) -> Result<SyncPoint> {
        let mut sync_point = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_GetSyncPoint(self.inner, index, &mut sync_point).to_result()?;
        }
        Ok(sync_point.into())
    }

    /// Retrieves information on an embedded sync point.
    ///
    /// For for more information on sync points see Sync Points.
    pub fn get_sync_point_info(
        &self,
        point: SyncPoint,
        offset_type: TimeUnit,
    ) -> Result<(Utf8CString, c_uint)> {
        let mut offset = 0;
        let name = get_string(|name| unsafe {
            FMOD_Sound_GetSyncPointInfo(
                self.inner,
                point.into(),
                name.as_mut_ptr().cast(),
                name.len() as c_int,
                &mut offset,
                offset_type.into(),
            )
        })?;
        Ok((name, offset))
    }

    /// Retrieves the number of sync points stored within a sound.
    ///
    /// For for more information on sync points see Sync Points.
    pub fn get_sync_point_count(&self) -> Result<i32> {
        let mut count = 0;
        unsafe {
            FMOD_Sound_GetNumSyncPoints(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Adds a sync point at a specific time within the sound.
    ///
    /// For more information on sync points see Sync Points.
    pub fn add_sync_point(
        &self,
        offset: c_uint,
        offset_type: TimeUnit,
        name: &Utf8CStr,
    ) -> Result<SyncPoint> {
        let mut sync_point = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_AddSyncPoint(
                self.inner,
                offset,
                offset_type.into(),
                name.as_ptr(),
                &mut sync_point,
            )
            .to_result()?;
        }
        Ok(sync_point.into())
    }

    /// Deletes a sync point within the sound.
    ///
    /// For for more information on sync points see Sync Points.
    pub fn delete_sync_point(&self, point: SyncPoint) -> Result<()> {
        unsafe {
            FMOD_Sound_DeleteSyncPoint(self.inner, point.into()).to_result()?;
        }
        Ok(())
    }
}
