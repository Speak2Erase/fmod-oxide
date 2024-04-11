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
// along with fmod-rs.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    borrow::Borrow,
    ffi::{CStr, CString, FromBytesWithNulError},
    ops::{Deref, Index},
    str::Utf8Error,
};

use crate::utf8_cstr::Utf8CStr;

#[repr(transparent)]
#[derive(Clone)]
pub struct Utf8CString(CString);

impl Utf8CString {
    pub unsafe fn from_cstring_unchecked(cstring: CString) -> Self {
        Utf8CString(cstring)
    }

    pub fn from_cstring(cstring: CString) -> Result<Self, Utf8Error> {
        let _ = cstring.as_c_str().to_str()?;
        Ok(unsafe { Self::from_cstring_unchecked(cstring) })
    }
}

impl Borrow<Utf8CStr> for Utf8CString {
    fn borrow(&self) -> &Utf8CStr {
        unsafe { Utf8CStr::from_cstr_unchecked(self.0.as_c_str()) }
    }
}
