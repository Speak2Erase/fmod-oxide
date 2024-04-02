// Copyright (C) 2024 Lily Lyons
//
// This file is part of fmod-rs.
//
// fmod-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fmod-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with fmod-rs.  If not, see <http://www.gnu.org/licenses/>.

use std::{ffi::c_float, mem::MaybeUninit};

use fmod_sys::*;

use crate::Guid;

/// Represents a global mixer VCA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Vca {
    pub(crate) inner: *mut FMOD_STUDIO_VCA,
}

unsafe impl Send for Vca {}
unsafe impl Sync for Vca {}

impl From<*mut FMOD_STUDIO_VCA> for Vca {
    fn from(value: *mut FMOD_STUDIO_VCA) -> Self {
        Vca { inner: value }
    }
}

impl From<Vca> for *mut FMOD_STUDIO_VCA {
    fn from(value: Vca) -> Self {
        value.inner
    }
}

impl Vca {
    /// Sets the volume level.
    ///
    /// The VCA volume level is used to linearly modulate the levels of the buses and VCAs which it controls.
    pub fn set_volume(&self, volume: c_float) -> Result<()> {
        unsafe { FMOD_Studio_VCA_SetVolume(self.inner, volume).to_result() }
    }

    /// Retrieves the volume level.
    ///
    /// The final combined volume returned in the second tuple field combines the user value set using [`Vca::set_volume`] with the result of any automation or modulation applied to the VCA.
    /// The final combined volume is calculated asynchronously when the Studio system updates.
    pub fn get_volume(&self) -> Result<(c_float, c_float)> {
        let mut volume = 0.0;
        let mut final_volume = 0.0;
        unsafe {
            FMOD_Studio_VCA_GetVolume(self.inner, &mut volume, &mut final_volume).to_result()?;
        }
        Ok((volume, final_volume))
    }
}

impl Vca {
    /// Retrieves the GUID.
    pub fn get_id(&self) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_VCA_GetID(self.inner, guid.as_mut_ptr()).to_result()?;

            let guid = guid.assume_init().into();

            Ok(guid)
        }
    }

    /// Retrieves the path.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    // TODO: convert into possible macro for the sake of reusing code
    pub fn get_path(&self) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error =
                FMOD_Studio_VCA_GetPath(self.inner, std::ptr::null_mut(), 0, &mut string_len)
                    .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_VCA_GetPath(
                self.inner,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }

    /// Checks that the VCA reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_VCA_IsValid(self.inner).into() }
    }
}
