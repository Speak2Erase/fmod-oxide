// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{InitFlags, OutputType, SpeakerMode, System};
use fmod_sys::*;
use std::ffi::{c_int, c_uint, c_void};

#[derive(Debug)]
pub struct SystemBuilder {
    pub(crate) system: *mut FMOD_SYSTEM,
    pub(crate) thread_unsafe: bool,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for SystemBuilder {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for SystemBuilder {}

#[cfg(doc)]
use crate::{debug, memory};

impl SystemBuilder {
    /// Creates a new [`SystemBuilder`].
    ///
    /// # Safety
    ///
    /// This must be called first to create an FMOD System object before any other API calls (except for [`memory::initialize`] and [`debug::initialize`]).
    /// Use this function to create 1 or multiple instances of FMOD System objects.
    ///
    /// Calls to [`SystemBuilder::new`] and [`System::release`] are not thread-safe.
    /// Do not call these functions simultaneously from multiple threads at once.
    pub unsafe fn new() -> Result<Self> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_System_Create(&mut system, FMOD_VERSION).to_result()? };

        Ok(SystemBuilder {
            system,
            thread_unsafe: false,
        })
    }

    /// # Safety
    ///
    /// This function intializes FMOD to be thread unsafe, which makes *EVERY* Struct in this crate `!Send` and `!Sync` *without* marking them as `!Send` and `!Sync`.
    /// This means that there are no handrails preventing you from using FMOD across multiple threads, and you *must* ensure this yourself!
    #[cfg(not(feature = "thread-unsafe"))]
    pub unsafe fn thread_unsafe(&mut self) {
        self.thread_unsafe = true;
    }

    #[cfg(feature = "thread-unsafe")]
    pub fn thread_unsafe(&mut self) {
        self.thread_unsafe = true;
    }

    pub fn software_format(
        &mut self,
        sample_rate: c_int,
        speaker_mode: SpeakerMode,
        raw_speakers: c_int,
    ) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetSoftwareFormat(
                self.system,
                sample_rate,
                speaker_mode.into(),
                raw_speakers,
            )
            .to_result()?;
        };
        Ok(self)
    }

    pub fn software_channels(&mut self, software_channels: c_int) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetSoftwareChannels(self.system, software_channels).to_result()?;
        };
        Ok(self)
    }

    pub fn dsp_buffer_size(
        &mut self,
        buffer_size: c_uint,
        buffer_count: c_int,
    ) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetDSPBufferSize(self.system, buffer_size, buffer_count).to_result()?;
        };
        Ok(self)
    }

    pub fn output(&mut self, kind: OutputType) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetOutput(self.system, kind.into()).to_result()?;
        };
        Ok(self)
    }

    pub fn output_by_plugin(&mut self, handle: c_uint) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetOutputByPlugin(self.system, handle).to_result()?;
        };
        Ok(self)
    }

    pub fn build(self, max_channels: c_int, flags: InitFlags) -> Result<System> {
        unsafe { self.build_with_extra_driver_data(max_channels, flags, std::ptr::null_mut()) }
    }

    /// # Safety
    ///
    /// See the FMOD docs explaining driver data for more safety information.
    pub unsafe fn build_with_extra_driver_data(
        self,
        max_channels: c_int,
        mut flags: InitFlags,
        driver_data: *mut c_void,
    ) -> Result<System> {
        if self.thread_unsafe {
            flags.insert(InitFlags::THREAD_UNSAFE);
        } else {
            #[cfg(not(feature = "thread-unsafe"))]
            flags.remove(InitFlags::THREAD_UNSAFE);
        }
        unsafe {
            FMOD_System_Init(self.system, max_channels, flags.bits(), driver_data).to_result()?;
        }
        Ok(self.system.into())
    }
}
