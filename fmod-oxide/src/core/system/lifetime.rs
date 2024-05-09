// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::{InitFlags, System, SystemBuilder};

impl System {
    /// A convenience function over [`SystemBuilder`] with sane defaults.
    ///
    /// # Safety
    ///
    /// See [`SystemBuilder::new`] for safety info.
    pub unsafe fn new() -> Result<Self> {
        unsafe { SystemBuilder::new() }?.build(0, InitFlags::NORMAL)
    }

    /// Close the connection to the output and return to an uninitialized state without releasing the object.
    ///
    /// Closing renders objects created with this System invalid.
    /// Make sure any Sound, [`crate::ChannelGroup`], Geometry and DSP objects are released before calling this.
    ///
    /// All pre-initialize configuration settings will remain and the System can be reinitialized as needed.
    pub fn close(&self) -> Result<SystemBuilder> {
        unsafe {
            FMOD_System_Close(self.inner).to_result()?;
            Ok(SystemBuilder { system: self.inner })
        }
    }

    /// Closes and frees this object and its resources.
    ///
    /// This will internally call [`System::close`], so calling [`System::close`] before this function is not necessary.
    ///
    /// # Safety
    ///
    /// [`System::release`] is not thread-safe. Do not call this function simultaneously from multiple threads at once.
    pub unsafe fn release(&self) -> Result<()> {
        unsafe { FMOD_System_Release(self.inner).to_result() }
    }

    /// Updates the FMOD system.
    ///
    /// Should be called once per 'game' tick, or once per frame in your application to perform actions such as:
    /// - Panning and reverb from 3D attributes changes.
    /// - Virtualization of Channels based on their audibility.
    /// - Mixing for non-realtime output types. See comment below.
    /// - Streaming if using [`InitFlags::STREAM_FROM_UPDATE`].
    /// - Mixing if using [`InitFlags::MIX_FROM_UPDATE`]
    /// - Firing callbacks that are deferred until Update.
    /// - DSP cleanup.
    ///
    /// If [`OutputType::NoSoundNRT`] or  [`OutputType::WavWriterNRT`] output modes are used,
    /// this function also drives the software / DSP engine, instead of it running asynchronously in a thread as is the default behavior.
    /// This can be used for faster than realtime updates to the decoding or DSP engine which might be useful if the output is the wav writer for example.
    ///
    /// If [`InitFlags::STREAM_FROM_UPDATE`]. is used, this function will update the stream engine.
    /// Combining this with the non realtime output will mean smoother captured output.
    pub fn update(&self) -> Result<()> {
        unsafe { FMOD_System_Update(self.inner).to_result() }
    }

    /// Suspend mixer thread and relinquish usage of audio hardware while maintaining internal state.
    ///
    /// Used on mobile platforms when entering a backgrounded state to reduce CPU to 0%.
    ///
    /// All internal state will be maintained, i.e. created [`Sound`] and [`Channel`]s will stay available in memory.
    pub fn suspend_mixer(&self) -> Result<()> {
        unsafe { FMOD_System_MixerSuspend(self.inner).to_result() }
    }

    /// Resume mixer thread and reacquire access to audio hardware.
    ///
    /// Used on mobile platforms when entering the foreground after being suspended.
    ///
    /// All internal state will resume, i.e. created [`Sound`] and [`Channel`]s are still valid and playback will continue.
    pub fn resume_mixer(&self) -> Result<()> {
        unsafe { FMOD_System_MixerResume(self.inner).to_result() }
    }
}
