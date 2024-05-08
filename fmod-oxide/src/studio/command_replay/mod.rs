// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

mod callback;
mod general;
mod playback;
mod query;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)] // so we can transmute between types
pub struct CommandReplay {
    pub(crate) inner: *mut FMOD_STUDIO_COMMANDREPLAY,
}

unsafe impl Send for CommandReplay {}
unsafe impl Sync for CommandReplay {}

impl CommandReplay {
    /// Create a System instance from its FFI equivalent.
    ///
    /// # Safety
    /// This operation is unsafe because it's possible that the [`FMOD_STUDIO_COMMANDREPLAY`] will not have the right userdata type.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_COMMANDREPLAY) -> Self {
        CommandReplay { inner: value }
    }
}

impl From<CommandReplay> for *mut FMOD_STUDIO_COMMANDREPLAY {
    fn from(value: CommandReplay) -> Self {
        value.inner
    }
}
