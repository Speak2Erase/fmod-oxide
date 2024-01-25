//! fmod-rs
//!
//! Safe rust bindings to the FMOD sound engine.
//! This crate tries to be as rusty as possible, without comprimising on any APIs.
//! Certain APIs, such as loading banks from a pointer, are marked as unsafe, but are still available for use.
//!
//! Most documentation is copied directly from the FMOD docs, however some information (such as parameter values) are excluded.
//! Please refer to the FMOD documentation for more usage information.
//!
//! # Memory management & Copy types
//! TODO
//!
//! # Userdata
//! TODO

#![warn(rust_2018_idioms, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    missing_docs, // TODO: disable later
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
    clippy::missing_panics_doc // TODO: disable later
)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub use fmod_sys as ffi;
pub use fmod_sys::{error_code_to_str, Error, Result};

pub mod studio;

pub mod core;

mod common;
pub use common::*;

#[cfg(test)]
mod tests {
    mod studio {

        mod system {}
    }

    mod common {
        use crate::Guid;

        #[test]
        fn guid_display() {
            let guid = Guid::default();

            let guid_cstr = std::ffi::CString::new(guid.to_string()).unwrap();
            let parsed_guid = Guid::parse(&guid_cstr).unwrap();

            assert_eq!(parsed_guid, guid);
        }
    }
}
