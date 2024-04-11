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
    num::NonZeroUsize,
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
/// This type is `#[repr(transparent)]` and can be transmuted to a <code>&[`CStr`]</code> safely.
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
/// For the purposes of FFI you will likely want to refer to [`CStr`]'s documentation.
///
/// # Caveats
///
/// **Most conversions ([`AsRef`], [`Deref`], etc) exclude the nul terminator.**
///
/// If you want to get a string *with* the nul terminator, you will need to use the `as_*_with_nul` methods.
#[repr(transparent)]
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Utf8CStr(CStr);

/// An error indicating that a nul byte was not in the expected position, or that there was invalid UTF-8.
///
/// The slice used to create a [`Utf8CStr`] must have one and only one nul byte, positioned at the end.
///
/// This error is created by the [`Utf8CStr::from_utf8_with_nul`] method. See its documentation for more.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FromUtf8WithNul {
    /// The slice was not valid UTF-8.
    Utf8(Utf8Error),
    /// The slice was not a valid C string.
    CStr(FromBytesWithNulError),
}

/// An error indicating that no nul byte was present, or that there was invalid UTF-8.
///
/// A slice used to create a [`Utf8CStr`] must contain a nul byte somewhere within the slice, and must be valid UTF-8.
///
/// This error is created by the [`Utf8CStr::from_utf8_until_nul`] method.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FromUtf8UntilNul {
    /// The slice was not valid UTF-8.
    Utf8(Utf8Error),
    /// The slice was not a valid C string.
    CStr(FromBytesUntilNulError),
}

impl Utf8CStr {
    /// Creates a C string wrapper from a byte slice with exactly one nul
    /// terminator.
    ///
    /// This function will cast the provided `slice` to a `Utf8CStr`
    /// wrapper after ensuring that the byte slice is nul-terminated,
    /// valid UTF-8, and does not contain any interior nul bytes.
    ///
    /// If the nul byte may not be at the end,
    /// [`Utf8CStr::from_utf8_until_nul`] can be used instead.
    pub fn from_utf8_with_nul(slice: &[u8]) -> Result<&Self, FromUtf8WithNul> {
        let cstr = CStr::from_bytes_with_nul(slice)?;
        let _ = cstr.to_str()?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    /// Creates a C string wrapper from a byte slice with any number of nuls.
    ///
    /// This method will create a `Utf8CStr` from any byte slice that contains at
    /// least one nul byte and is valid UTF-8.
    /// Unlike with [`Utf8CStr::from_utf8_with_nul`], the caller
    /// does not need to know where the nul byte is located.
    ///
    /// If the first byte is a nul character, this method will return an
    /// empty `Utf8CStr`. If multiple nul characters are present, the `Utf8CStr` will
    /// end at the first one.
    ///
    /// If the slice only has a single nul byte at the end, this method is
    /// equivalent to [`Utf8CStr::from_utf8_with_nul`].
    pub fn from_utf8_until_nul(slice: &[u8]) -> Result<&Self, FromUtf8UntilNul> {
        let cstr = CStr::from_bytes_until_nul(slice)?;
        let _ = cstr.to_str()?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    /// Creates a C string wrapper from a string slice with exactly one nul
    /// terminator.
    ///
    /// This function will cast the provided `str` to a `Utf8CStr`
    /// wrapper after ensuring that the byte slice is nul-terminated,
    /// and does not contain any interior nul bytes.
    ///
    /// If the nul byte may not be at the end,
    /// [`Utf8CStr::from_str_until_nul`] can be used instead.
    pub fn from_str_with_nul(str: &str) -> Result<&Self, FromBytesWithNulError> {
        let cstr = CStr::from_bytes_with_nul(str.as_bytes())?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    /// Creates a C string wrapper from a string slice with any number of nuls.
    ///
    /// This method will create a `Utf8CStr` from any string slice that contains at
    /// least one nul byte. Unlike with [`Utf8CStr::from_str_with_nul`], the caller
    /// does not need to know where the nul byte is located.
    ///
    /// If the first byte is a nul character, this method will return an
    /// empty `Utf8CStr`. If multiple nul characters are present, the `Utf8CStr` will
    /// end at the first one.
    ///
    /// If the slice only has a single nul byte at the end, this method is
    /// equivalent to [`Utf8CStr::from_str_with_nul`].
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
    #[must_use]
    pub const unsafe fn from_utf8_with_nul_unchecked(slice: &[u8]) -> &Self {
        unsafe { &*(core::ptr::from_ref(slice) as *const Utf8CStr) } // cast() does not have ?Sized, so we have to use an as cast.
    }

    /// Unsafely creates a UTF-8 C string wrapper from [`CStr`].
    ///
    /// # Safety
    ///
    /// The provided C string must be UTF-8.
    #[must_use]
    pub const unsafe fn from_cstr_unchecked(cstr: &CStr) -> &Self {
        unsafe { Self::from_utf8_with_nul_unchecked(cstr.to_bytes_with_nul()) }
    }

    /// This function is like [`Utf8CStr::from_ptr`], but does not perform UTF-8 validation.
    ///
    /// # Safety
    ///
    /// You must follow all safety requirements of [`CStr::from_ptr`]. The pointer **MUST** point to valid UTF-8.
    #[must_use]
    pub unsafe fn from_ptr_unchecked<'a>(ptr: *const c_char) -> &'a Self {
        unsafe { Self::from_cstr_unchecked(CStr::from_ptr(ptr)) }
    }

    /// Wraps a raw C string with a safe C string wrapper.
    ///
    /// This function will wrap the provided `ptr` with a `Utf8CStr` wrapper, which
    /// allows inspection and interoperation of non-owned C strings. The total
    /// size of the terminated buffer must be smaller than [`isize::MAX`] **bytes**
    /// in memory (a restriction from [`core::slice::from_raw_parts`]).
    ///
    /// # Safety
    ///
    /// You must follow all safety requirements of [`CStr::from_ptr`].
    ///
    /// # Caveat
    ///
    /// The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse,
    /// it's suggested to tie the lifetime to whichever source lifetime is safe in the context,
    /// such as by providing a helper function taking the lifetime of a host value for the slice,
    /// or by explicit annotation.
    pub unsafe fn from_ptr<'a>(ptr: *const c_char) -> Result<&'a Self, Utf8Error> {
        unsafe {
            let cstr = CStr::from_ptr(ptr);
            Self::from_cstr(cstr)
        }
    }

    /// Returns the inner pointer to this C string.
    ///
    /// The returned pointer will be valid for as long as `self` is, and points
    /// to a contiguous region of memory terminated with a 0 byte to represent
    /// the end of the string.
    ///
    /// The type of the returned pointer is
    /// [`*const c_char`][core::ffi::c_char], and whether it's
    /// an alias for `*const i8` or `*const u8` is platform-specific.
    ///
    /// **WARNING**
    ///
    /// The returned pointer is read-only; writing to it (including passing it
    /// to C code that writes to it) causes undefined behavior.
    ///
    /// It is your responsibility to make sure that the underlying memory is not
    /// freed too early!
    #[must_use]
    pub const fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }

    /// Yields a <code>&[`Utf8CStr`]</code> slice if the `CStr` contains valid UTF-8.
    ///
    /// If the contents of the `CStr` are valid UTF-8 data, this
    /// function will return the corresponding <code>&[`Utf8CStr`]</code> slice. Otherwise,
    /// it will return an error with details of where UTF-8 validation failed.
    pub fn from_cstr(cstr: &CStr) -> Result<&Self, Utf8Error> {
        let _ = cstr.to_str()?;
        Ok(unsafe { Self::from_cstr_unchecked(cstr) })
    }

    /// Converts a borrowed C string into an owned C string.
    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn to_cstring(&self) -> Utf8CString {
        self.to_owned()
    }

    /// Converts a `Utf8CStr` into a <code>&[`str`]</code>.
    ///
    /// The resulting string slice does not contain a nul byte.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.0.to_bytes()) }
    }

    /// Converts a `Utf8CStr` into a <code>&[`str`]</code>.
    ///
    /// The resulting string slice contains a nul byte.
    #[must_use]
    pub const fn as_str_with_nul(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.0.to_bytes_with_nul()) }
    }

    /// Converts a `Utf8CStr` into a <code>&[`CStr`]</code>.
    #[must_use]
    pub const fn as_c_str(&self) -> &CStr {
        &self.0
    }

    /// Converts a `Utf8CStr` into a <code>&[[u8]]</code>.
    ///
    /// The resulting byte slice does not contain a nul byte.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.to_bytes()
    }

    /// Converts a `Utf8CStr` into a <code>&[[u8]]</code>.
    ///
    /// The resulting byte slice contains a nul byte.
    #[must_use]
    pub const fn as_bytes_with_nul(&self) -> &[u8] {
        self.0.to_bytes_with_nul()
    }

    /// Returns the length of `self`. Like C's `strlen`, this does not include the nul terminator.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Returns the length of `self`. Unlike C's `strlen`, this **DOES** include the nul terminator.
    #[must_use]
    pub fn len_with_nul(&self) -> NonZeroUsize {
        unsafe {
            let len = self.as_str_with_nul().len();
            // SAFETY: len should always be >= 1 because of the nul terminator.
            NonZeroUsize::new_unchecked(len)
        }
    }

    /// Returns `true` if `self.len()` is 0.
    ///
    /// `self.len_with_nul` will still return 1, though.
    #[must_use]
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
