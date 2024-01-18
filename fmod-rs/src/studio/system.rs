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

use fmod_sys::*;
use std::ffi::{c_int, c_uint, CStr};

/// The main system object for FMOD Studio.
///
/// Initializing the FMOD Studio System object will also initialize the core System object.
///
/// Created with [`SystemBuilder`], which handles initialization for you.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // todo: should this logically be copy?
pub struct System {
    pub(crate) inner: *mut FMOD_STUDIO_SYSTEM,
}

/// A builder for creating and initializing a [`System`].
///
/// Handles setting values that can only be set before initialization for you.
#[must_use]
pub struct SystemBuilder {
    system: *mut FMOD_STUDIO_SYSTEM,
}

// default impl is ok, all values are zero or none.
#[derive(Clone, Copy, Default)]
pub struct AdvancedSettings {
    pub command_queue_size: c_uint,
    pub handle_initial_size: c_uint,
    pub studioupdateperiod: c_int,
    pub idle_sample_data_pool_size: c_int,
    pub streaming_schedule_delay: c_uint,
    // todo: lifetime requirements for this struct?
    // fmod might copy this to a managed string, so we can relax the 'static
    pub encryption_key: Option<&'static CStr>,
}

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct InitFlags: c_uint {
    const NORMAL                = 0x0000_0000;
    const LIVEUPDATE            = 0x0000_0001;
    const ALLOW_MISSING_PLUGINS = 0x0000_0002;
    const SYNCHRONOUS_UPDATE    = 0x0000_0004;
    const DEFERRED_CALLBACKS    = 0x0000_0008;
    const LOAD_FROM_UPDATE      = 0x0000_0010;
    const MEMORY_TRACKING       = 0x0000_0020;
  }
}

impl SystemBuilder {
    /// Creates a new [`SystemBuilder`].
    ///
    /// # Safety
    ///
    /// Calling either of this function concurrently with any FMOD Studio API function (including this function) may cause undefined behavior.
    /// External synchronization must be used if calls to [`SystemBuilder::new`] or [`System::release`] could overlap other FMOD Studio API calls.
    /// All other FMOD Studio API functions are thread safe and may be called freely from any thread unless otherwise documented.
    pub unsafe fn new() -> Result<Self> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_Studio_System_Create(&mut system, FMOD_VERSION).to_result()? };

        Ok(SystemBuilder { system })
    }

    pub fn settings(self, settings: AdvancedSettings) -> Result<Self> {
        let mut settings = settings.into();
        // this function expects a pointer. maybe this is incorrect?
        unsafe { FMOD_Studio_System_SetAdvancedSettings(self.system, &mut settings).to_result() }?;
        Ok(self)
    }

    // todo: move to a core system builder
    pub fn software_format(
        self,
        sample_rate: c_int,
        speaker_mode: FMOD_SPEAKERMODE,
        raw_speakers: c_int,
    ) -> Result<Self> {
        let mut core = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetCoreSystem(self.system, &mut core).to_result()?;
            FMOD_System_SetSoftwareFormat(core, sample_rate, speaker_mode, raw_speakers)
                .to_result()?;
        };
        Ok(self)
    }

    pub fn software_channels(self, software_channels: c_int) -> Result<Self> {
        let mut core = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetCoreSystem(self.system, &mut core).to_result()?;
            FMOD_System_SetSoftwareChannels(core, software_channels).to_result()?;
        };
        Ok(self)
    }

    pub fn dsp_buffer_size(self, buffer_size: c_uint, buffer_count: c_int) -> Result<Self> {
        let mut core = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetCoreSystem(self.system, &mut core).to_result()?;
            FMOD_System_SetDSPBufferSize(core, buffer_size, buffer_count).to_result()?;
        };
        Ok(self)
    }

    pub fn build(
        self,
        max_channels: c_int,
        studio_flags: InitFlags,
        flags: FMOD_INITFLAGS,
    ) -> Result<System> {
        unsafe {
            FMOD_Studio_System_Initialize(
                self.system,
                max_channels,
                studio_flags.bits(),
                flags,
                std::ptr::null_mut(), // not sure how to handle this
            )
            .to_result()?;
        }
        Ok(System { inner: self.system })
    }
}

impl AdvancedSettings {
    /// Create a safe [`AdvancedSettings`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// The encryption key from [`FMOD_STUDIO_ADVANCEDSETTINGS`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`CStr::from_ptr`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_ADVANCEDSETTINGS) -> Self {
        let encryption_key = if value.encryptionkey.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(value.encryptionkey)) }
        };

        Self {
            command_queue_size: value.commandqueuesize,
            handle_initial_size: value.handleinitialsize,
            studioupdateperiod: value.studioupdateperiod,
            idle_sample_data_pool_size: value.idlesampledatapoolsize,
            streaming_schedule_delay: value.streamingscheduledelay,
            encryption_key,
        }
    }
}

// It's safe to go from AdvancedSettings to FMOD_STUDIO_ADVANCEDSETTINGS because a &'static CStr meets all the safety FMOD expects. (aligned, null termienated, etc)
impl From<AdvancedSettings> for FMOD_STUDIO_ADVANCEDSETTINGS {
    fn from(value: AdvancedSettings) -> Self {
        let encryption_key = value.encryption_key.map_or(std::ptr::null(), CStr::as_ptr);

        FMOD_STUDIO_ADVANCEDSETTINGS {
            cbsize: std::mem::size_of::<Self>() as c_int,
            commandqueuesize: value.command_queue_size,
            handleinitialsize: value.handle_initial_size,
            studioupdateperiod: value.studioupdateperiod,
            idlesampledatapoolsize: value.idle_sample_data_pool_size,
            streamingscheduledelay: value.streaming_schedule_delay,
            encryptionkey: encryption_key,
        }
    }
}

impl From<*mut FMOD_STUDIO_SYSTEM> for System {
    fn from(value: *mut FMOD_STUDIO_SYSTEM) -> Self {
        System { inner: value }
    }
}

impl From<System> for *mut FMOD_STUDIO_SYSTEM {
    fn from(value: System) -> Self {
        value.inner
    }
}

/// Most of FMOD is thread safe.
/// There are some select functions that are not thread safe to call, those are marked as unsafe.
unsafe impl Send for System {}
unsafe impl Sync for System {}

impl System {
    /// A convenience function over [`SystemBuilder`] with sane defaults.
    ///
    /// # Safety
    ///
    /// See [`SystemBuilder::new`] for safety info.
    pub unsafe fn new() -> Result<Self> {
        unsafe { SystemBuilder::new() }?.build(0, InitFlags::NORMAL, 0)
    }

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
        unsafe { FMOD_Studio_System_Release(self.inner) }.to_result()
    }
}
