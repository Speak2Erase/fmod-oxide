#![warn(rust_2018_idioms, clippy::pedantic)]
#![cfg_attr(not(feature = "alloc"), no_std)]

//! UTF-8 equivalents of `std`'s C string types.

#[cfg(feature = "alloc")]
extern crate alloc;

mod cstr;
#[cfg(feature = "alloc")]
mod cstring;

pub use cstr::*;
#[cfg(feature = "alloc")]
pub use cstring::*;

#[macro_export]
macro_rules! c {
    ($s:literal) => {{
        const __CSTR: &'static $crate::Utf8CStr = {
            let nul_terminated = concat!($s, "\0").as_bytes();
            let Ok(cstr) = core::ffi::CStr::from_bytes_with_nul(nul_terminated) else {
                panic!("string contains nul byte")
            };
            unsafe { $crate::Utf8CStr::from_cstr_unchecked(cstr) } // SAFETY: nul_terminated came from a rust string literal which is utf-8.
        };
        __CSTR
    }};
}

macro_rules! cmp_impls {
    (impl $impl_for:ty {
      $(
        $( #[$attrs:meta] )*
        $cmp_type:ty: $convert_method:expr => $cmp_convert_method:expr
      ),+
    }) => {
      $(
        $( #[$attrs] )*
        impl PartialEq<$cmp_type> for $impl_for {
          fn eq(&self, other: &$cmp_type) -> bool {
            $convert_method(self).eq($cmp_convert_method(other))
          }
        }

        $( #[$attrs] )*
        impl PartialEq<$impl_for> for $cmp_type {
          fn eq(&self, other: &$impl_for) -> bool {
            other.eq(self)
          }
        }

        $( #[$attrs] )*
        impl PartialOrd<$cmp_type> for $impl_for {
          fn partial_cmp(&self, other: &$cmp_type) -> Option<core::cmp::Ordering> {
            $convert_method(self).partial_cmp($cmp_convert_method(other))
          }
        }

        $( #[$attrs] )*
        impl PartialOrd<$impl_for> for $cmp_type {
          fn partial_cmp(&self, other: &$impl_for) -> Option<core::cmp::Ordering> {
            other.partial_cmp(self)
          }
        }
      )+
    };
}
pub(crate) use cmp_impls;

#[cfg(test)]
mod tests {
    use crate::{Utf8CStr, Utf8CString};

    const TEST_STR: &str = "Hello, world!";
    const INTERIOR_NUL: &str = "Hello\0, world!";
    const TRAILING_NUL: &str = "Hello, world!\0";

    #[test]
    fn it_works() {
        let str = c!("Hello, world!");
        assert_eq!(str, TEST_STR);
    }

    #[test]
    fn from_interior() {
        Utf8CStr::from_str_with_nul(INTERIOR_NUL).expect_err("string had interior nul");
    }

    #[test]
    fn from_trailing() {
        let str = Utf8CStr::from_str_with_nul(TRAILING_NUL).unwrap();
        assert_eq!(str, TEST_STR);
    }

    #[test]
    fn truncating_interior() {
        let str = Utf8CStr::from_str_until_nul(INTERIOR_NUL).unwrap();
        assert_eq!(str, "Hello");
    }

    #[test]
    fn from_non_terminated() {
        let str = Utf8CString::new(TEST_STR).unwrap();
        assert_eq!(str, TEST_STR);
    }
}
