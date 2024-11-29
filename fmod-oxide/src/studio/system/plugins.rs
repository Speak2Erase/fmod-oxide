// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CStr;

use crate::studio::System;

impl System {
    /// Registers a plugin DSP.
    ///
    /// Plugin DSPs used by an event must be registered using this function before loading the bank containing the event.
    ///
    /// # Safety
    ///
    /// This function provides no gaurdrails or safe API for registering a plugin.
    /// It can call into non-rust external code.
    /// Dsp descriptions are intended to be retrieved from a plugin's C API, so it's not feasible to provide a safe API for this function.
    /// TODO
    pub unsafe fn register_plugin(
        &self,
        dsp_description: *const FMOD_DSP_DESCRIPTION,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_RegisterPlugin(self.inner.as_ptr(), dsp_description).to_result()
        }
    }

    /// Unregisters a plugin DSP.
    ///
    /// # Safety
    ///
    /// This function provides no gaurdrails or safe API for unregistering a plugin.
    /// It can call into non-rust external code.
    /// TODO
    pub unsafe fn unregister_plugin(&self, name: &Utf8CStr) -> Result<()> {
        unsafe {
            FMOD_Studio_System_UnregisterPlugin(self.inner.as_ptr(), name.as_ptr()).to_result()
        }
    }
}
