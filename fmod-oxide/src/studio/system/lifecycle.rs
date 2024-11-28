// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::studio::{InitFlags, System, SystemBuilder};

impl System {
    /// A convenience function over [`SystemBuilder`] with sane defaults.
    ///
    /// # Safety
    ///
    /// See [`SystemBuilder::new`] for safety info.
    pub unsafe fn new() -> Result<Self> {
        unsafe { SystemBuilder::new() }?.build(0, InitFlags::NORMAL, crate::InitFlags::NORMAL)
    }

    // TODO: could we solve this with an "owned" system and a shared system?
    ///This function will free the memory used by the Studio System object and everything created under it.
    ///
    /// # Safety
    ///
    /// Calling either of this function concurrently with any FMOD Studio API function (including this function) may cause undefined behavior.
    /// External synchronization must be used if calls to [`SystemBuilder::new`] or [`System::release`] could overlap other FMOD Studio API calls.
    /// All other FMOD Studio API functions are thread safe and may be called freely from any thread unless otherwise documented.
    ///
    /// All handles or pointers to objects associated with a Studio System object become invalid when the Studio System object is released.
    /// The FMOD Studio API attempts to protect against stale handles and pointers being used with a different Studio System object but this protection cannot be guaranteed and attempting to use stale handles or pointers may cause undefined behavior.
    ///
    /// This function is not safe to be called at the same time across multiple threads.
    pub unsafe fn release(self) -> Result<()> {
        unsafe { FMOD_Studio_System_Release(self.inner).to_result() }
    }

    /// Update the FMOD Studio System.
    ///
    /// When Studio is initialized in the default asynchronous processing mode this function submits all buffered commands for execution on the Studio Update thread for asynchronous processing.
    /// This is a fast operation since the commands are not processed on the calling thread.
    /// If Studio is initialized with [`InitFlags::DEFERRED_CALLBACKS`] then any deferred callbacks fired during any asynchronous updates since the last call to this function will be called.
    /// If an error occurred during any asynchronous updates since the last call to this function then this function will return the error result.
    ///
    /// When Studio is initialized with [`InitFlags::SYNCHRONOUS_UPDATE`] queued commands will be processed immediately when calling this function, the scheduling and update logic for the Studio system are executed and all callbacks are fired.
    /// This may block the calling thread for a substantial amount of time.
    pub fn update(&self) -> Result<()> {
        unsafe { FMOD_Studio_System_Update(self.inner) }.to_result()
    }

    /// This function blocks the calling thread until all pending commands have been executed and all non-blocking bank loads have been completed.
    ///
    /// This is equivalent to calling [`System::update`] and then sleeping until the asynchronous thread has finished executing all pending commands.
    pub fn flush_commands(&self) -> Result<()> {
        unsafe { FMOD_Studio_System_FlushCommands(self.inner) }.to_result()
    }

    /// Block until all sample loading and unloading has completed.
    ///
    /// This function may stall for a long time if other threads are continuing to issue calls to load and unload sample data, e.g. by creating new event instances.
    pub fn flush_sample_loading(&self) -> Result<()> {
        unsafe { FMOD_Studio_System_FlushSampleLoading(self.inner) }.to_result()
    }
}
