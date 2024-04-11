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

use core::{
    borrow::Borrow,
    ffi::{c_char, CStr, FromBytesUntilNulError, FromBytesWithNulError},
    ops::{Deref, Index},
    slice::SliceIndex,
    str::Utf8Error,
};

#[cfg(feature = "alloc")]
use crate::cstring::Utf8CString;
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
use alloc::{borrow::Cow, borrow::ToOwned, ffi::CString, rc::Rc, string::String, sync::Arc};

/// Representation of a borrowed UTF-8 C string.
/// This type is `#[repr(transparent)]` and can be transmuted to a <code>&[CStr]</code> safely.
///
/// This type represents a borrowed reference to a nul-terminated
/// array of bytes. It can be constructed safely from a <code>&[[u8]]</code> or a <code>&[str]</code>
/// slice, or unsafely from a raw `*const c_char`.
///
/// `&Utf8CStr` is to [`Utf8CString`] as <code>&[str]</code> is to [`String`]: the former
/// in each pair are borrowed references; the latter are owned
/// strings.
///
/// Like [`CStr`], this structure does **not** have a guaranteed layout and is not recommended to be placed in the signatures of FFI functions.
/// ### You cannot use this in place of a regular `*const c_char` pointer.
/// Instead, safe wrappers of FFI functions may leverage the unsafe [`Utf8CStr::from_ptr`] constructor to provide a safe interface to other consumers.
///
/// # Caveats
///
/// ### Most conversions ([`AsRef`], [`Deref`], etc) exclude the nul terminator.
///
/// If you want to get a string *with* the nul terminator, you will need to use the `as_*_with_nul` methods.
#[repr(transparent)]
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Utf8CStr(CStr);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FromUtf8WithNul {
    Utf8(Utf8Error),
    CStr(FromBytesWithNulError),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FromUtf8UntilNul {
    Utf8(Utf8Error),
    CStr(FromBytesUntilNulError),
}

impl Utf8CStr {
    pub fn from_utf8_with_nul(slice: &[u8]) -> Result<&Self, FromUtf8WithNul> {
        let cstr = CStr::from_bytes_with_nul(slice)?;
        let _ = cstr.to_str()?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    pub fn from_utf8_until_nul(slice: &[u8]) -> Result<&Self, FromUtf8UntilNul> {
        let cstr = CStr::from_bytes_until_nul(slice)?;
        let _ = cstr.to_str()?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    pub fn from_str_with_nul(str: &str) -> Result<&Self, FromBytesWithNulError> {
        let cstr = CStr::from_bytes_with_nul(str.as_bytes())?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    pub fn from_str_until_nul(str: &str) -> Result<&Self, FromBytesUntilNulError> {
        let cstr = CStr::from_bytes_until_nul(str.as_bytes())?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    /// Unsafely creates a UTF-8 C string wrapper from a byte slice.
    ///
    /// This function will cast the provided bytes to a [`Utf8CStr`] wrapper without performing any sanity checks.
    ///
    /// # Safety
    ///
    /// The provided slice must be UTF-8, nul-terminated and not contain any interior nul bytes.
    pub const unsafe fn from_utf8_with_nul_unchecked(slice: &[u8]) -> &Self {
        unsafe { &*(core::ptr::from_ref(slice) as *const Utf8CStr) } // cast() does not have ?Sized, so we have to use an as cast.
    }

    /// Unsafely creates a UTF-8 C string wrapper from [`CStr`].
    ///
    /// # Safety
    ///
    /// The provided C string must be UTF-8.
    pub const unsafe fn from_cstr_unchecked(cstr: &CStr) -> &Self {
        unsafe { Self::from_utf8_with_nul_unchecked(cstr.to_bytes_with_nul()) }
    }

    /// Wraps a raw C string with a safe C string wrapper.
    ///
    /// This function will wrap the provided `ptr` with a `Utf8CStr` wrapper, which
    /// allows inspection and interoperation of non-owned C strings. The total
    /// size of the terminated buffer must be smaller than [`isize::MAX`] **bytes**
    /// in memory (a restriction from [`slice::from_raw_parts`]).
    ///
    /// # Safety
    ///
    /// You must follow all safety requirements of [`CStr::from_ptr`], with the additional requirement that the pointer **MUST** be valid UTF-8.
    ///
    /// # Caveat
    ///
    /// The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse,
    /// it's suggested to tie the lifetime to whichever source lifetime is safe in the context,
    /// such as by providing a helper function taking the lifetime of a host value for the slice,
    /// or by explicit annotation.
    pub unsafe fn from_ptr<'a>(ptr: *const c_char) -> &'a Self {
        unsafe { Self::from_cstr_unchecked(CStr::from_ptr(ptr)) }
    }

    pub const fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }

    pub fn from_cstr(cstr: &CStr) -> Result<&Self, Utf8Error> {
        let _ = cstr.to_str()?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    #[cfg(feature = "alloc")]
    pub fn to_cstring(&self) -> Utf8CString {
        self.to_owned()
    }

    pub const fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.0.to_bytes()) }
    }

    pub const fn as_str_with_nul(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.0.to_bytes_with_nul()) }
    }

    pub const fn as_c_str(&self) -> &CStr {
        &self.0
    }

    pub const fn as_bytes(&self) -> &[u8] {
        self.0.to_bytes()
    }

    pub const fn as_bytes_with_nul(&self) -> &[u8] {
        self.0.to_bytes_with_nul()
    }

    pub const fn len(&self) -> usize {
        self.as_str().len()
    }

    pub fn len_with_nul(&self) -> usize {
        self.as_str_with_nul().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
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

super::cmp_impls! {
  impl Utf8CStr {
    CStr: Utf8CStr::as_c_str => core::convert::identity,
    str: Utf8CStr::as_str => core::convert::identity,
    &str: Utf8CStr::as_str => Deref::deref,
    #[cfg(feature = "alloc")]
    CString: Utf8CStr::as_c_str => CString::as_c_str,
    #[cfg(feature = "alloc")]
    String: Utf8CStr::as_str => String::as_str
  }
}

impl<I> Index<I> for Utf8CStr
where
    I: SliceIndex<str>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.as_str().index(index)
    }
}

impl core::fmt::Debug for Utf8CStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str_with_nul().fmt(f)
    }
}

impl core::fmt::Display for Utf8CStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for &Utf8CStr {
    fn default() -> Self {
        unsafe { Utf8CStr::from_utf8_with_nul_unchecked(b"\0") }
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a Utf8CStr> for Cow<'a, CStr> {
    fn from(value: &'a Utf8CStr) -> Self {
        Cow::Borrowed(value.as_c_str())
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a Utf8CStr> for Cow<'a, str> {
    fn from(value: &'a Utf8CStr) -> Self {
        Cow::Borrowed(value.as_str())
    }
}

#[cfg(feature = "alloc")]
impl From<&Utf8CStr> for Rc<CStr> {
    fn from(value: &Utf8CStr) -> Self {
        Rc::from(value.as_c_str())
    }
}

#[cfg(feature = "alloc")]
impl From<&Utf8CStr> for Rc<Utf8CStr> {
    fn from(value: &Utf8CStr) -> Self {
        let rc = Rc::<CStr>::from(value);
        // SAFETY: This is how you spell a transmute of Rc's pointee type.
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Utf8CStr) }
    }
}

#[cfg(feature = "alloc")]
impl From<&Utf8CStr> for Rc<str> {
    fn from(value: &Utf8CStr) -> Self {
        Rc::from(value.as_str())
    }
}

#[cfg(feature = "alloc")]
impl From<&Utf8CStr> for Arc<Utf8CStr> {
    fn from(value: &Utf8CStr) -> Self {
        let arc = Arc::<[u8]>::from(value.as_bytes_with_nul());
        // SAFETY: This is how you spell a transmute of Arc's pointee type.
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Utf8CStr) }
    }
}

#[cfg(feature = "alloc")]
impl From<&Utf8CStr> for Arc<str> {
    fn from(value: &Utf8CStr) -> Self {
        Arc::from(value.as_str())
    }
}

#[cfg(feature = "alloc")]
impl From<&Utf8CStr> for String {
    fn from(value: &Utf8CStr) -> Self {
        value.as_str().to_owned()
    }
}

#[cfg(feature = "alloc")]
impl ToOwned for Utf8CStr {
    type Owned = Utf8CString;

    fn to_owned(&self) -> Self::Owned {
        unsafe { Utf8CString::from_cstring_unchecked(self.0.to_owned()) }
    }
}

impl core::fmt::Display for FromUtf8WithNul {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FromUtf8WithNul::Utf8(e) => e.fmt(f),
            FromUtf8WithNul::CStr(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromUtf8WithNul {}

impl From<Utf8Error> for FromUtf8WithNul {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<FromBytesWithNulError> for FromUtf8WithNul {
    fn from(value: FromBytesWithNulError) -> Self {
        Self::CStr(value)
    }
}

impl core::fmt::Display for FromUtf8UntilNul {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FromUtf8UntilNul::Utf8(e) => e.fmt(f),
            FromUtf8UntilNul::CStr(e) => e.fmt(f),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for FromUtf8UntilNul {}

impl From<Utf8Error> for FromUtf8UntilNul {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<FromBytesUntilNulError> for FromUtf8UntilNul {
    fn from(value: FromBytesUntilNulError) -> Self {
        Self::CStr(value)
    }
}
