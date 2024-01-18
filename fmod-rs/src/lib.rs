#![warn(rust_2018_idioms, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    missing_docs, // todo: disable later
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::must_use_candidate
)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub use fmod_sys as ffi;
pub use fmod_sys::{error_code_to_str, Error, Result};

pub mod studio {
    mod system;
    pub use system::*;
}

#[cfg(test)]
mod tests {
    mod studio {

        mod system {
            #[test]
            fn lifecycle() {
                let system = unsafe { crate::studio::System::new() }.unwrap();
                unsafe { system.release() }.unwrap();
            }
        }
    }
}
