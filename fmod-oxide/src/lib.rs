//! fmod-oxide
//!
//! Safe rust bindings to the FMOD sound engine.
//! This crate tries to be as rusty as possible, without comprimising on any APIs.
//! Certain APIs, such as loading banks from a pointer, are marked as unsafe, but are still available for use.
//!
//! Most documentation is copied directly from the FMOD docs, however some information (such as parameter values) are excluded.
//! Please refer to the FMOD documentation for more usage information.
//!
//! Please see the `fmod-sys` crate (or the [`ffi`] re-export) documentation for more information on the raw bindings,
//! and where to place the FMOD libraries. This crate does not distribute the FMOD libraries or headers!
//!
//! # Basic example
//! ```ignore
//! // System creation is unsafe and must be performed prior to any other FMOD operations.
//! let mut builder = unsafe { fmod::studio::SystemBuilder::new() }?;
//! let system = builder.build()?;
//!
//! // Load a bank
//! let bank = system.load_bank_file("path/to/bank.bank", fmod::studio::LoadBankFlags::NORMAL)?;
//! // Query all events in the bank
//! for event in bank.get_event_list().unwrap() {
//!     println!("Event: {}", event.get_path()?);
//! }
//!
//! // Releasing Systems is unsafe because it cannot be called concurrently, and all FMOD objects are rendered invalid.
//! unsafe { system.release() };
//! ```
//!
//! # Memory management & Copy types
//!
//! All FMOD objects are `Copy`, `Clone`, `Send` and `Sync` because it's possible to have multiple references to the same object. (e.g. loading a bank and then retrieving it by its path)
//! There are a lot of use-cases where you may want to fetch something (like a bank) and never use it again.
//! Implementing `Drop` to automatically release things would go against that particular use-case, so this crate opts to have manual `release()` methods instead.
//!
//! This crate does not currently guard against use-after-frees, although it's something I have planned.
//!
//! # String types
//! `fmod-oxide` aims to be as zero-cost as possible, and as such, it uses UTF-8 C strings from the `lanyard` crate as its string type.
//! This means that all FMOD functions take a `&Utf8CStr` instead of a `&str` or `&CStr`.
//! `&Utf8CStr` is pretty cheap to construct (and can even be done statically with the `c!` macro), so this should not be a problem.
//!
//! When FMOD returns a string, it will always return a `Utf8CString` (the owned version of `Utf8CStr`) because it's difficult to encode lifetime requirements of FMOD strings.
//!
//! This applies to structs like `fmod::studio::AdvancedSettings` which store C strings.
//! Converting structs like `AdvancedSettings` to their FFI equivalents is done by reference as to not pass ownership of the string to FMOD.
//!
//! # Userdata
//!
//! Right now this crate stores userdata in a global slotmap alongside its owner, and every so often will remove userdata with invalid owners.
//! This solution works best with a mark and sweep GC, which Rust does not have. We could somewhat solve this issue by doing this check in `System::update`.
//! That would make `System::update` expensive- it would have an additional `O(n)` complexity added to it, which goes against the purpose of this crate.
//!
//! It's difficult to associate userdata with an individual system in this system though- so we have to clear the slotmap whenever any system is released.
//! Releasing a system is performed at the end of execution generally so this probably won't be an issue.

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
#![doc(html_favicon_url = "https://www.fmod.com/assets/fmod-logo.svg")]
#![doc(html_logo_url = "https://www.fmod.com/assets/fmod-logo.svg")]

pub use lanyard::*;

#[doc(inline)]
pub use fmod_sys as ffi;
pub use fmod_sys::{error_code_to_str, Error, Result};

pub mod core;
#[doc(inline)]
pub use core::*;

pub mod studio;

#[doc(hidden)]
#[cfg(feature = "userdata-abstraction")]
pub mod userdata;
#[cfg(feature = "userdata-abstraction")]
pub use userdata::Userdata;

pub const VERSION: u32 = fmod_sys::FMOD_VERSION;
pub const MAX_CHANNEL_WIDTH: u32 = fmod_sys::FMOD_MAX_CHANNEL_WIDTH;
pub const MAX_LISTENERS: u32 = fmod_sys::FMOD_MAX_LISTENERS;
pub const MAX_SYSTEMS: u32 = fmod_sys::FMOD_MAX_SYSTEMS;

// relatively common bound
pub trait Shareable: Send + Sync + 'static {}
impl<T> Shareable for T where T: Send + Sync + 'static {}
