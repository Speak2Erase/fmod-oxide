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
    ffi::{c_char, CStr},
    ops::{Deref, Index},
    slice::SliceIndex,
};

use crate::cstr::Utf8CStr;

#[allow(unused_imports)]
use alloc::{
    borrow::Cow,
    boxed::Box,
    ffi::{CString, FromVecWithNulError, IntoStringError, NulError},
    rc::Rc,
    string::FromUtf8Error,
    string::String,
    sync::Arc,
    vec::Vec,
};

/// A type representing an owned, C-compatible, UTF-8, nul-terminated string with no nul bytes in the
/// middle.
/// This type is `#[repr(transparent)]` and can be transmuted to a <code>&[`CString`]</code> safely.
///
/// This type serves the purpose of being able to safely generate a
/// C-compatible string from a Rust string. An instance of this
/// type is a static guarantee that the underlying bytes contain no interior 0
/// bytes ("nul characters") and that the final byte is 0 ("nul terminator").
///
/// `Utf8CString` is to <code>&[`Utf8CStr`]</code> as [`String`] is to <code>&[str]</code>: the former
/// in each pair are owned strings; the latter are borrowed
/// references.
///
/// # Creating a `Utf8CString`
///
/// A `Utf8CString` is created from either a borrowed or owned string,
/// or anything that implements <code>[Into]<[String]></code> (for
/// example, you can build a `Utf8CString` straight out of a [`String`] or
/// a <code>&[str]</code>, since both implement that trait).
///
/// The [`Utf8CString::new`] method will actually check that the provided string
/// does not have 0 bytes in the middle, and return an error if it
/// finds one.
///
/// # Extracting a raw pointer to the whole C string
///
/// `Utf8CString` implements an [`as_ptr`][`Utf8CStr::as_ptr`] method through the [`Deref`]
/// trait. This method will give you a `*const c_char` which you can
/// feed directly to extern functions that expect a nul-terminated
/// string, like C's `strdup()`. Notice that [`as_ptr`][`Utf8CStr::as_ptr`] returns a
/// read-only pointer; if the C code writes to it, that causes
/// undefined behavior.
///
/// # Extracting a slice of the whole C string
///
/// Alternatively, you can obtain a <code>&[[u8]]</code> slice from a
/// `CString` with the [`Utf8CStr::as_bytes`] method. Slices produced in this
/// way do *not* contain the trailing nul terminator. This is useful
/// when you will be calling an extern function that takes a `*const
/// u8` argument which is not necessarily nul-terminated, plus another
/// argument with the length of the string — like C's `strndup()`.
/// You can of course get the slice's length with its
/// [`len`][slice::len] method.
///
/// If you need a <code>&[[u8]]</code> slice *with* the nul terminator, you
/// can use [`Utf8CStr::as_bytes_with_nul`] instead.
///
/// Once you have the kind of slice you need (with or without a nul
/// terminator), you can call the slice's own
/// [`as_ptr`][slice::as_ptr] method to get a read-only raw pointer to pass to
/// extern functions. See the documentation for that function for a
/// discussion on ensuring the lifetime of the raw pointer.
///
/// # Safety
///
/// `Utf8CString` is intended for working with traditional C-style strings
/// (a sequence of non-nul bytes terminated by a single nul byte); the
/// primary use case for these kinds of strings is interoperating with C-like
/// code. Often you will need to transfer ownership to/from that external
/// code. It is strongly recommended that you thoroughly read through the
/// documentation of `Utf8CString` before use, as improper ownership management
/// of `Utf8CString` instances can lead to invalid memory accesses, memory leaks,
/// and other memory errors.
///
/// # Caveats
///
/// **Most conversions ([`AsRef`], [`Deref`], etc) exclude the nul terminator.**
///
/// If you want to get a string *with* the nul terminator, you will need to use the `as_*_with_nul` methods.
#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Utf8CString(CString);

/// An error indicating that a nul byte was not in the expected position, or that there was invalid UTF-8.
///
/// The vector used to create a [`Utf8CString`] must have one and only one nul byte, positioned at the end.
///
/// This error is created by the [`Utf8CString::from_utf8_with_nul`] method. See its documentation for more.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FromOwnedUtf8WithNul {
    /// The vec was not valid UTF-8.
    Utf8(FromUtf8Error),
    /// The vec was not a valid C string.
    CString(FromVecWithNulError),
}

impl Utf8CString {
    /// Creates a new C-compatible string from a string.
    ///
    /// This function will consume the provided data and use the underlying bytes to construct a new string, ensuring that there is a trailing 0 byte.
    /// This trailing 0 byte will be appended by this function; the provided data should not contain any 0 bytes in it.
    pub fn new(string: impl Into<String>) -> Result<Self, NulError> {
        let cstring = CString::new(string.into())?;
        Ok(unsafe { Self::from_cstring_unchecked(cstring) })
    }

    /// Creates a new C-compatible string from a container of bytes.
    ///
    /// Runtime checks are present to ensure there is only one nul byte in the String, its last element.
    pub fn from_string_with_nul(string: impl Into<String>) -> Result<Self, NulError> {
        let cstring = CString::new(string.into())?;
        Ok(unsafe { Self::from_cstring_unchecked(cstring) })
    }

    /// Attempts to converts a <code>[Vec]<[u8]></code> to a [`Utf8CString`].
    ///
    /// Runtime checks are present to ensure there is only one nul byte in the
    /// [`Vec`], its last element, and that it is valid UTF-8.
    pub fn from_utf8_with_nul(v: Vec<u8>) -> Result<Self, FromOwnedUtf8WithNul> {
        let string = String::from_utf8(v)?;
        let cstring = CString::from_vec_with_nul(string.into_bytes())?;
        Ok(unsafe { Utf8CString::from_cstring_unchecked(cstring) })
    }

    /// Creates a C-compatible string by consuming a byte vector,
    /// without checking for interior 0 bytes.
    ///
    /// Trailing 0 byte will be appended by this function.
    ///
    /// This method is equivalent to [`Utf8CString::new`] except that no runtime
    /// assertion is made that `v` contains no 0 bytes, and it requires an
    /// actual byte vector, not anything that can be converted to one with Into.
    ///
    /// # Safety
    ///
    /// The provided vector must be valid UTF-8 and contain no interior nul-bytes.
    #[must_use]
    pub unsafe fn from_utf8_unchecked(v: Vec<u8>) -> Self {
        unsafe {
            let cstring = CString::from_vec_unchecked(v); // this does append a nul byte
            Self::from_cstring_unchecked(cstring)
        }
    }

    /// Converts a <code>[Vec]<[u8]></code> to a [`CString`] without checking the
    /// invariants on the given [`Vec`].
    ///
    /// # Safety
    ///
    /// The given [`Vec`] **must** have one nul byte as its last element, and be valid UTF-8.
    /// This means it cannot be empty nor have any other nul byte anywhere else.
    #[must_use]
    pub unsafe fn from_utf8_with_nul_unchecked(v: Vec<u8>) -> Self {
        unsafe {
            let cstring = CString::from_vec_with_nul_unchecked(v); // this doesn't append a nul byte
            Self::from_cstring_unchecked(cstring)
        }
    }

    /// Unsafely creates a UTF-8 C string wrapper from [`CString`].
    ///
    /// # Safety
    ///
    /// The provided C string must be UTF-8.
    #[must_use]
    pub unsafe fn from_cstring_unchecked(cstring: CString) -> Self {
        Utf8CString(cstring)
    }

    /// Yields a <code>&[`Utf8CString`]</code> slice if the `CString` contains valid UTF-8.
    ///
    /// If the contents of the `CString` are valid UTF-8 data, this
    /// function will return the corresponding <code>&[`Utf8CString`]</code> slice. Otherwise,
    /// it will return an error with details of where UTF-8 validation failed.
    pub fn from_cstring(cstring: CString) -> Result<Self, IntoStringError> {
        // we end up doing a bunch of converting back and forth but the unsafe string -> cstring conversion is a cast, essentially
        // might be better to use as_str() instead
        let string = cstring.into_string()?; // into_string does NOT contain the nul byte, so we have to put it back in from_utf8_unchecked
        Ok(unsafe { Self::from_utf8_unchecked(string.into_bytes()) })
    }

    /// Converts a `Utf8CString` into a <code>&[`Utf8CStr`]</code>.
    #[must_use]
    pub fn as_utf8_cstr(&self) -> &Utf8CStr {
        unsafe { Utf8CStr::from_cstr_unchecked(self.0.as_c_str()) }
    }

    /// Retakes ownership of a `CString` that was transferred to C via
    /// [`Utf8CString::into_raw`].
    ///
    /// Additionally, the length of the string will be recalculated from the pointer.
    ///
    /// # Safety
    ///
    /// This should only ever be called with a pointer that was earlier
    /// obtained by calling [`Utf8CString::into_raw`]. Other usage (e.g., trying to take
    /// ownership of a string that was allocated by foreign code) is likely to lead
    /// to undefined behavior or allocator corruption.
    ///
    /// It should be noted that the length isn't just "recomputed," but that
    /// the recomputed length must match the original length from the
    /// [`Utf8CString::into_raw`] call. This means the [`Utf8CString::into_raw`]/`from_raw`
    /// methods should not be used when passing the string to C functions that can
    /// modify the string's length.
    ///
    /// > **Note:** If you need to borrow a string that was allocated by
    /// > foreign code, use [`Utf8CStr`]. If you need to take ownership of
    /// > a string that was allocated by foreign code, you will need to
    /// > make your own provisions for freeing it appropriately, likely
    /// > with the foreign code's API to do that.
    pub unsafe fn from_raw(ptr: *mut c_char) -> Self {
        unsafe {
            let cstring = CString::from_raw(ptr);
            Self::from_cstring_unchecked(cstring)
        }
    }

    /// Consumes the `CString` and transfers ownership of the string to a C caller.
    ///
    /// The pointer which this function returns must be returned to Rust and reconstituted using
    /// [`Utf8CString::from_raw`] to be properly deallocated. Specifically, one
    /// should *not* use the standard C `free()` function to deallocate
    /// this string.
    ///
    /// Failure to call [`Utf8CString::from_raw`] will lead to a memory leak.
    ///
    /// The C side must **not** modify the length of the string (by writing a
    /// nul byte somewhere inside the string or removing the final one) before
    /// it makes it back into Rust using [`Utf8CString::from_raw`]. See the safety section
    /// in [`Utf8CString::from_raw`].
    #[must_use]
    pub fn into_raw(self) -> *mut c_char {
        self.0.into_raw()
    }

    /// Consumes a `Utf8CString` and converts it into a [`String`].
    ///
    /// The resulting string does not contain a nul byte.
    #[must_use]
    pub fn into_string(self) -> String {
        unsafe { String::from_utf8_unchecked(self.0.into_bytes()) }
    }

    /// Consumes a `Utf8CString` and converts it into a [`String`].
    ///
    /// The resulting string contains a nul byte.
    #[must_use]
    pub fn into_string_with_nul(self) -> String {
        unsafe { String::from_utf8_unchecked(self.0.into_bytes_with_nul()) }
    }

    /// Consumes the `Utf8CString` and returns the underlying byte buffer.
    ///
    /// The returned buffer does **not** contain the trailing nul
    /// terminator, and it is guaranteed to not have any interior nul
    /// bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }

    /// Equivalent to [`Utf8CString::into_bytes()`] except that the
    /// returned vector includes the trailing nul terminator.
    #[must_use]
    pub fn into_bytes_with_nul(self) -> Vec<u8> {
        self.0.into_bytes_with_nul()
    }

    /// Converts this `Utf8CString` into a boxed [`Utf8CStr`].
    #[must_use]
    pub fn into_boxed_utf8_cstr(self) -> Box<Utf8CStr> {
        unsafe { Box::from_raw(Box::into_raw(self.0.into_boxed_c_str()) as *mut Utf8CStr) }
    }
}

impl Deref for Utf8CString {
    type Target = Utf8CStr;

    fn deref(&self) -> &Self::Target {
        self.as_utf8_cstr()
    }
}

impl AsRef<Utf8CStr> for Utf8CString {
    fn as_ref(&self) -> &Utf8CStr {
        self.as_utf8_cstr()
    }
}

impl AsRef<str> for Utf8CString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<CStr> for Utf8CString {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl Borrow<Utf8CStr> for Utf8CString {
    fn borrow(&self) -> &Utf8CStr {
        self.as_utf8_cstr()
    }
}

impl Borrow<CStr> for Utf8CString {
    fn borrow(&self) -> &CStr {
        self.as_c_str()
    }
}

super::cmp_impls! {
  impl Utf8CString {
    CStr: Utf8CStr::as_c_str => core::convert::identity,
    str: Utf8CStr::as_str => core::convert::identity,
    &str: Utf8CStr::as_str => Deref::deref,
    #[cfg(feature = "alloc")]
    CString: Utf8CStr::as_c_str => CString::as_c_str,
    #[cfg(feature = "alloc")]
    String: Utf8CStr::as_str => String::as_str
  }
}

impl<'a> From<&'a Utf8CString> for Cow<'a, Utf8CStr> {
    fn from(value: &'a Utf8CString) -> Self {
        Self::Borrowed(value.as_utf8_cstr())
    }
}

impl<'a> From<Utf8CString> for Cow<'a, Utf8CStr> {
    fn from(value: Utf8CString) -> Self {
        Self::Owned(value)
    }
}

impl From<Box<Utf8CStr>> for Utf8CString {
    fn from(value: Box<Utf8CStr>) -> Self {
        unsafe {
            let cstr: Box<CStr> = Box::from_raw(Box::into_raw(value) as *mut CStr);
            Self::from_cstring_unchecked(CString::from(cstr))
        }
    }
}

impl From<Utf8CString> for Arc<Utf8CStr> {
    fn from(value: Utf8CString) -> Self {
        unsafe {
            let arc: Arc<CStr> = Arc::from(value.0);
            Arc::from_raw(Arc::into_raw(arc) as *const Utf8CStr)
        }
    }
}

impl From<Utf8CString> for Rc<Utf8CStr> {
    fn from(value: Utf8CString) -> Self {
        unsafe {
            let rc: Rc<CStr> = Rc::from(value.0);
            Rc::from_raw(Rc::into_raw(rc) as *const Utf8CStr)
        }
    }
}

impl From<Utf8CString> for Box<Utf8CStr> {
    fn from(value: Utf8CString) -> Self {
        unsafe {
            let b: Box<CStr> = Box::from(value.0);
            Box::from_raw(Box::into_raw(b) as *mut Utf8CStr)
        }
    }
}

impl<I> Index<I> for Utf8CString
where
    I: SliceIndex<str>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.as_str().index(index)
    }
}

impl core::fmt::Debug for Utf8CString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str_with_nul().fmt(f)
    }
}

impl core::fmt::Display for Utf8CString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for Utf8CString {
    fn default() -> Self {
        <&Utf8CStr>::default().to_cstring()
    }
}

impl core::fmt::Display for FromOwnedUtf8WithNul {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FromOwnedUtf8WithNul::Utf8(e) => e.fmt(f),
            FromOwnedUtf8WithNul::CString(e) => e.fmt(f),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for FromOwnedUtf8WithNul {}

impl From<FromUtf8Error> for FromOwnedUtf8WithNul {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<FromVecWithNulError> for FromOwnedUtf8WithNul {
    fn from(value: FromVecWithNulError) -> Self {
        Self::CString(value)
    }
}
