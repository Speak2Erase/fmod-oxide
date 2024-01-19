#![warn(rust_2018_idioms, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    missing_docs, // todo: disable later
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub use fmod_sys as ffi;
pub use fmod_sys::{error_code_to_str, Error, Result};

pub mod studio {
    mod bank;
    pub use bank::*;

    mod system;
    pub use system::*;
}

pub mod common;
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
