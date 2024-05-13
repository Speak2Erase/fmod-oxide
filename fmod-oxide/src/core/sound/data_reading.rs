// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_uint;

use fmod_sys::*;

use crate::{OpenState, Sound};

impl Sound {
    /// Retrieves the state a sound is in after being opened with the non blocking flag, or the current state of the streaming buffer.
    ///
    /// When a sound is opened with FMOD_NONBLOCKING, it is opened and prepared in the background, or asynchronously.
    /// This allows the main application to execute without stalling on audio loads.
    /// This function will describe the state of the asynchronous load routine i.e. whether it has succeeded, failed or is still in progress.
    ///
    /// If 'starving' is true, then you will most likely hear a stuttering/repeating sound as the decode buffer loops on itself and replays old data.
    /// With the ability to detect stream starvation, muting the sound with ChannelControl::setMute will keep the stream quiet until it is not starving any more.
    ///
    /// #### Note: Always check [`OpenState`] to determine the state of the sound.
    /// Do not assume that if this function returns [`Ok`] then the sound has finished loading.
    pub fn get_open_state(&self) -> Result<(OpenState, c_uint, bool, bool)> {
        let mut open_state = 0;
        let mut percent_buffered = 0;
        let mut starving = FMOD_BOOL::FALSE;
        let mut disk_busy = FMOD_BOOL::FALSE;
        let error = unsafe {
            FMOD_Sound_GetOpenState(
                self.inner,
                &mut open_state,
                &mut percent_buffered,
                &mut starving,
                &mut disk_busy,
            )
            .to_error()
        };

        let open_state = OpenState::try_from_ffi(open_state, error)?;
        let starving = starving.into();
        let disk_busy = disk_busy.into();
        Ok((open_state, percent_buffered, starving, disk_busy))
    }

    // TODO read, seek, lock, unlock
}
