// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::studio::System;

impl System {
    /// Registers a plugin DSP.
    ///
    /// Plugin DSPs used by an event must be registered using this function before loading the bank containing the event.
    ///
    /// # Safety
    /// TODO
    pub unsafe fn register_plugin(&self) {
        todo!()
    }

    /// Unregisters a plugin DSP.
    ///
    /// # Safety
    /// TODO
    pub unsafe fn unregister_plugin(&self) {
        todo!()
    }
}
