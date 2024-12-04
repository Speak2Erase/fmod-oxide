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
//! I've been thinking on this issue for a few months and I can't find a way to safely do it that doesn't involve significant overhead.
//! My previous approach involved setting userdata to a slotmap index, and removing things from the slotmap when they were released or stopped being valid.
//! This was quite jank to say the least and was not a solution I was happy with.
//!
//! After that, I tried fixing `release()`'s use-after-free issues by storing a `HashSet` of all the pointers FMOD returns from its API, and removing pointers from that `HashSet` when `release()` was called.
//! This would've had similar issues to the slotmap approach but would drop userdata early and not handle the `EventDescription` issue, so I scrapped it.
//!
//! Ultimately I've decided to make userdata not the concern of this crate. Setting and fetching it is perfectly safe, *using* the pointer is what's unsafe.
//! It's likely better this way- the semantics of userdata are just too flexible and its hard to cover every edge case.
//! Userdata isn't super commonly used anyway- it is mainly used to pass data into callbacks, but it's easy enough to use a `static` to do that.
//!
//! # Feature flags
#![doc = document_features::document_features!()]
// Used to document cfgs (copied from https://docs.rs/winit/latest/src/winit/lib.rs.html#1-207)
#![cfg_attr(docsrs, feature(doc_auto_cfg), doc(cfg_hide(doc, docsrs)))]
#![warn(
    rust_2018_idioms,
    clippy::pedantic,
    missing_debug_implementations,
    missing_copy_implementations,
    //missing_docs
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::must_use_candidate
)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![doc(html_favicon_url = "https://www.fmod.com/assets/fmod-logo.svg")]
#![doc(html_logo_url = "https://www.fmod.com/assets/fmod-logo.svg")]

// FIXME fetching things (like from a bank) could have an alternative that doesn't allocate and instead fills out a slice
// TODO no_std?

pub use lanyard::*;

#[doc(inline)]
pub use fmod_sys as ffi;
pub use fmod_sys::{error_code_to_str, Error, Result};

pub mod core;
#[doc(inline)]
pub use core::*;

/// The FMOD Studio API.
///
/// The Studio API is a more high-level library which is tightly integrated with *FMOD Studio*, FMOD's production tool.
pub mod studio;

pub const VERSION: u32 = fmod_sys::FMOD_VERSION;
pub const MAX_CHANNEL_WIDTH: u32 = fmod_sys::FMOD_MAX_CHANNEL_WIDTH;
pub const MAX_LISTENERS: u32 = fmod_sys::FMOD_MAX_LISTENERS;
pub const MAX_SYSTEMS: u32 = fmod_sys::FMOD_MAX_SYSTEMS;

// relatively common bound
pub trait Shareable: Send + Sync + 'static {}
impl<T> Shareable for T where T: Send + Sync + 'static {}
