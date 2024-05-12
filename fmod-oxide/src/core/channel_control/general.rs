// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::{ChannelControl, System};

impl ChannelControl {
    // TODO callback
    // TODO userdata

    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_ChannelControl_GetSystemObject(self.inner, &mut system).to_result()? }
        Ok(system.into())
    }
}
