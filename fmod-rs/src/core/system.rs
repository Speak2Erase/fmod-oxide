// Copyright (C) 2024 Lily Lyons
//
// This file is part of fmod-rs.
//
// fmod-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fmod-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with fmod-rs.  If not, see <http://www.gnu.org/licenses/>.

use std::ffi::{c_int, c_uint, c_void};

use fmod_sys::*;

use super::InitFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct System {
    pub(crate) inner: *mut FMOD_SYSTEM,
}

pub struct SystemBuilder {
    system: *mut FMOD_SYSTEM,
}

unsafe impl Send for System {}
unsafe impl Sync for System {}

impl From<*mut FMOD_SYSTEM> for System {
    fn from(value: *mut FMOD_SYSTEM) -> Self {
        System { inner: value }
    }
}

impl From<System> for *mut FMOD_SYSTEM {
    fn from(value: System) -> Self {
        value.inner
    }
}

impl SystemBuilder {
    /// Creates a new [`SystemBuilder`].
    ///
    /// # Safety
    ///
    /// This must be called first to create an FMOD System object before any other API calls (except for Memory_Initialize and Debug_Initialize).
    /// Use this function to create 1 or multiple instances of FMOD System objects.
    ///
    /// Calls to [`SystemBuilder::new`] and [`System::release`] are not thread-safe.
    /// Do not call these functions simultaneously from multiple threads at once.
    pub unsafe fn new() -> Result<Self> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_System_Create(&mut system, FMOD_VERSION).to_result()? };

        Ok(SystemBuilder { system })
    }

    pub fn software_format(
        self,
        sample_rate: c_int,
        speaker_mode: FMOD_SPEAKERMODE, // todo convert to enum
        raw_speakers: c_int,
    ) -> Result<Self> {
        unsafe {
            FMOD_System_SetSoftwareFormat(self.system, sample_rate, speaker_mode, raw_speakers)
                .to_result()?;
        };
        Ok(self)
    }

    pub fn software_channels(self, software_channels: c_int) -> Result<Self> {
        unsafe {
            FMOD_System_SetSoftwareChannels(self.system, software_channels).to_result()?;
        };
        Ok(self)
    }

    pub fn dsp_buffer_size(self, buffer_size: c_uint, buffer_count: c_int) -> Result<Self> {
        unsafe {
            FMOD_System_SetDSPBufferSize(self.system, buffer_size, buffer_count).to_result()?;
        };
        Ok(self)
    }

    pub fn output(self, kind: FMOD_OUTPUTTYPE) -> Result<Self> {
        unsafe {
            FMOD_System_SetOutput(self.system, kind).to_result()?;
        };
        Ok(self)
    }

    pub fn output_by_plugin(self, handle: c_uint) -> Result<Self> {
        unsafe {
            FMOD_System_SetOutputByPlugin(self.system, handle).to_result()?;
        };
        Ok(self)
    }

    pub fn build(
        self,
        max_channels: c_int,
        flags: InitFlags, // todo core init flags
    ) -> Result<System> {
        unsafe { self.build_with_extra_driver_data(max_channels, flags, std::ptr::null_mut()) }
    }

    /// # Safety
    ///
    /// See the FMOD docs explaining driver data for more safety information.
    pub unsafe fn build_with_extra_driver_data(
        self,
        max_channels: c_int,
        flags: InitFlags,
        driver_data: *mut c_void,
    ) -> Result<System> {
        unsafe {
            FMOD_System_Init(self.system, max_channels, flags.bits(), driver_data).to_result()?;
        }
        Ok(System { inner: self.system })
    }
}

impl System {
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
    /// If FMOD_OUTPUTTYPE_NOSOUND_NRT or FMOD_OUTPUTTYPE_WAVWRITER_NRT output modes are used,
    /// this function also drives the software / DSP engine, instead of it running asynchronously in a thread as is the default behavior.
    /// This can be used for faster than realtime updates to the decoding or DSP engine which might be useful if the output is the wav writer for example.
    ///
    /// If [`InitFlags::STREAM_FROM_UPDATE`]. is used, this function will update the stream engine.
    /// Combining this with the non realtime output will mean smoother captured output.
    pub fn update(&self) -> Result<()> {
        unsafe { FMOD_System_Update(self.inner).to_result() }
    }
}
