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
    borrow::{Borrow, Cow},
    ffi::{CStr, CString, FromBytesWithNulError},
    ops::{Deref, Index},
    str::Utf8Error,
};

use crate::utf8_cstring::Utf8CString;

#[repr(transparent)]
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Utf8CStr(CStr);

#[derive(Debug)]
pub enum Utf8CStrError {
    Utf8(Utf8Error),
    CStr(FromBytesWithNulError),
}

impl Utf8CStr {
    pub fn from_utf8_with_nul(slice: &[u8]) -> Result<&Self, Utf8CStrError> {
        let cstr = CStr::from_bytes_with_nul(slice)?;
        let _ = cstr.to_str()?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    pub unsafe fn from_utf8_with_nul_unchecked(slice: &[u8]) -> &Self {
        unsafe { &*(std::ptr::from_ref(slice) as *const Utf8CStr) }
    }

    pub unsafe fn from_cstr_unchecked(cstr: &CStr) -> &Self {
        unsafe { Self::from_utf8_with_nul_unchecked(cstr.to_bytes_with_nul()) }
    }

    pub fn from_cstr(cstr: &CStr) -> Result<&Self, Utf8Error> {
        let _ = cstr.to_str()?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    pub fn to_cstring(&self) -> Utf8CString {
        self.to_owned()
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.0.to_bytes()) }
    }

    pub fn as_str_with_nul(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.0.to_bytes_with_nul()) }
    }

    pub fn as_c_str(&self) -> &CStr {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.to_bytes()
    }

    pub fn as_bytes_with_nul(&self) -> &[u8] {
        self.0.to_bytes_with_nul()
    }
}

impl Deref for Utf8CStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<CStr> for Utf8CStr {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl AsRef<str> for Utf8CStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Utf8CStr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Borrow<str> for Utf8CStr {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<CStr> for Utf8CStr {
    fn borrow(&self) -> &CStr {
        self.as_c_str()
    }
}

impl PartialEq<CStr> for Utf8CStr {
    fn eq(&self, other: &CStr) -> bool {
        self.as_c_str().eq(other)
    }
}

impl PartialEq<str> for Utf8CStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<CString> for Utf8CStr {
    fn eq(&self, other: &CString) -> bool {
        self.as_c_str().eq(other)
    }
}

impl<I> Index<I> for Utf8CStr
where
    str: Index<I, Output = str>,
{
    type Output = str;

    fn index(&self, index: I) -> &Self::Output {
        self.as_str().index(index)
    }
}

impl std::fmt::Debug for Utf8CStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str_with_nul().fmt(f)
    }
}

impl std::fmt::Display for Utf8CStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> From<&'a Utf8CStr> for Cow<'a, CStr> {
    fn from(value: &'a Utf8CStr) -> Self {
        Cow::Borrowed(value.as_c_str())
    }
}

impl ToOwned for Utf8CStr {
    type Owned = Utf8CString;

    fn to_owned(&self) -> Self::Owned {
        unsafe { Utf8CString::from_cstring_unchecked(self.0.to_owned()) }
    }
}

impl std::fmt::Display for Utf8CStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Utf8CStrError::Utf8(e) => e.fmt(f),
            Utf8CStrError::CStr(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for Utf8CStrError {}

impl From<Utf8Error> for Utf8CStrError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<FromBytesWithNulError> for Utf8CStrError {
    fn from(value: FromBytesWithNulError) -> Self {
        Self::CStr(value)
    }
}
