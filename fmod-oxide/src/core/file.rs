// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

/// Information function to retrieve the state of FMOD disk access.
///
/// Do not use this function to synchronize your own reads with, as due to timing,
/// you might call this function and it says false = it is not busy,
/// but the split second after calling this function, internally FMOD might set it to busy.
/// Use [`get_disk_busy`] for proper mutual exclusion as it uses semaphores.
pub fn get_disk_busy() -> Result<bool> {
    let mut busy = 0;
    unsafe {
        FMOD_File_GetDiskBusy(&mut busy).to_result()?;
    }
    Ok(busy > 0)
}

/// Sets the busy state for disk access ensuring mutual exclusion of file operations.
///
/// If file IO is currently being performed by FMOD this function will block until it has completed.
///
/// This function should be called in pairs once to set the state, then again to clear it once complete.
pub fn set_disk_busy(busy: bool) -> Result<()> {
    unsafe { FMOD_File_SetDiskBusy(std::ffi::c_int::from(busy)).to_result() }
}
