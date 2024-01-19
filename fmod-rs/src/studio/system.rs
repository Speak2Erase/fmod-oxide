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

use crate::Guid;

use super::Bank;

/// The main system object for FMOD Studio.
///
/// Initializing the FMOD Studio System object will also initialize the core System object.
///
/// Created with [`SystemBuilder`], which handles initialization for you.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // todo: should this logically be copy?
#[repr(transparent)] // so we can transmute between types
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
        const NORMAL                = FMOD_STUDIO_INIT_NORMAL;
        const LIVEUPDATE            = FMOD_STUDIO_INIT_LIVEUPDATE;
        const ALLOW_MISSING_PLUGINS = FMOD_STUDIO_INIT_ALLOW_MISSING_PLUGINS;
        const SYNCHRONOUS_UPDATE    = FMOD_STUDIO_INIT_SYNCHRONOUS_UPDATE;
        const DEFERRED_CALLBACKS    = FMOD_STUDIO_INIT_DEFERRED_CALLBACKS;
        const LOAD_FROM_UPDATE      = FMOD_STUDIO_INIT_LOAD_FROM_UPDATE;
        const MEMORY_TRACKING       = FMOD_STUDIO_INIT_MEMORY_TRACKING;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LoadBankFlags: c_uint {
        const NORMAL             = FMOD_STUDIO_LOAD_BANK_NORMAL;
        const NONBLOCKING        = FMOD_STUDIO_LOAD_BANK_NONBLOCKING;
        const DECOMPRESS_SAMPLES = FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES;
        const UNENCRYPTED        = FMOD_STUDIO_LOAD_BANK_UNENCRYPTED;
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

// todo: could we solve this with an "owned" system and a shared system?
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

impl System {
    // todo: take &self or not?
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

impl System {
    // todo: load bank with callbacks
    pub fn load_bank_custom(&self) -> Result<Bank> {
        todo!()
    }

    /// Sample data must be loaded separately.
    ///
    /// By default this function will block until the file load finishes.
    ///
    /// Using the [`LoadBankFlags::NONBLOCKING`] flag will cause the bank to be loaded asynchronously.
    /// In that case this function will always return [`Ok`] and bank will contain a valid bank handle.
    /// Load errors for asynchronous banks can be detected by calling [`Bank::get_loading_state`].
    /// Failed asynchronous banks should be released by calling [`Bank::unload`].
    ///
    /// If a bank has been split, separating out assets and optionally streams from the metadata bank, all parts must be loaded before any APIs that use the data are called.
    /// It is recommended you load each part one after another (order is not important), then proceed with dependent API calls such as [`Bank::load_sample_data`] or [`System::get_event`].
    pub fn load_bank_file(&self, filename: &CStr, load_flags: LoadBankFlags) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_LoadBankFile(
                self.inner,
                filename.as_ptr(),
                load_flags.bits(),
                &mut bank,
            )
            .to_result()?;
        }
        Ok(bank.into())
    }

    /// Sample data must be loaded separately.
    ///
    /// This function is the safe counterpart of [`System::load_bank_pointer`].
    /// FMOD will allocate an internal buffer and copy the data from the passed in buffer before using it.
    /// The buffer passed to this function may be cleaned up at any time after this function returns.
    ///
    /// By default this function will block until the load finishes.
    ///
    /// Using the [`LoadBankFlags::NONBLOCKING`] flag will cause the bank to be loaded asynchronously.
    /// In that case this function will always return [`Ok`] and bank will contain a valid bank handle.
    /// Load errors for asynchronous banks can be detected by calling [`Bank::get_loading_state`].
    /// Failed asynchronous banks should be released by calling [`Bank::unload`].
    ///
    /// This function is not compatible with [`AdvancedSettings::encryption_key`], using them together will cause an error to be returned.
    ///
    /// If a bank has been split, separating out assets and optionally streams from the metadata bank, all parts must be loaded before any APIs that use the data are called.
    /// It is recommended you load each part one after another (order is not important), then proceed with dependent API calls such as [`Bank::load_sample_data`] or [`System::get_event`].
    pub fn load_bank_memory(&self, buffer: &[u8], flags: LoadBankFlags) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_LoadBankMemory(
                self.inner,
                buffer.as_ptr().cast::<i8>(),
                buffer.len() as c_int,
                FMOD_STUDIO_LOAD_MEMORY_MODE_FMOD_STUDIO_LOAD_MEMORY,
                flags.bits(),
                &mut bank,
            )
            .to_result()?;
        }
        Ok(bank.into())
    }

    /// Sample data must be loaded separately.
    ///
    /// This function is the unsafe counterpart of [`System::load_bank_memory`].
    /// FMOD will use the passed memory buffer directly.
    ///
    /// By default this function will block until the load finishes.
    ///
    /// Using the [`LoadBankFlags::NONBLOCKING`] flag will cause the bank to be loaded asynchronously.
    /// In that case this function will always return [`Ok`] and bank will contain a valid bank handle.
    /// Load errors for asynchronous banks can be detected by calling [`Bank::get_loading_state`].
    /// Failed asynchronous banks should be released by calling [`Bank::unload`].
    ///
    /// This function is not compatible with [`AdvancedSettings::encryption_key`], using them together will cause an error to be returned.
    ///
    /// If a bank has been split, separating out assets and optionally streams from the metadata bank, all parts must be loaded before any APIs that use the data are called.
    /// It is recommended you load each part one after another (order is not important), then proceed with dependent API calls such as [`Bank::load_sample_data`] or [`System::get_event`].
    ///
    /// # Safety
    /// When using this function the buffer must be aligned to [`FMOD_STUDIO_LOAD_MEMORY_ALIGNMENT`]
    /// and the memory must persist until the bank has been fully unloaded, which can be some time after calling [`Bank::unload`] to unload the bank.
    /// You can ensure the memory is not being freed prematurely by only freeing it after receiving the [`FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD`] callback.
    pub unsafe fn load_bank_pointer(
        &self,
        buffer: *const [u8],
        flags: LoadBankFlags,
    ) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_LoadBankMemory(
                self.inner,
                buffer.cast::<i8>(),
                (*buffer).len() as c_int,
                FMOD_STUDIO_LOAD_MEMORY_MODE_FMOD_STUDIO_LOAD_MEMORY_POINT,
                flags.bits(),
                &mut bank,
            )
            .to_result()?;
        }
        Ok(bank.into())
    }

    /// Unloads all currently loaded banks.
    pub fn unload_all_banks(&self) -> Result<()> {
        unsafe { FMOD_Studio_System_UnloadAll(self.inner).to_result() }
    }

    /// Retrieves a loaded bank
    ///
    /// `path_or_id` may be a path, such as `bank:/Weapons` or an ID string such as `{793cddb6-7fa1-4e06-b805-4c74c0fd625b}`.
    ///
    /// Note that path lookups will only succeed if the strings bank has been loaded.
    pub fn get_bank(&self, path_or_id: &CStr) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetBank(self.inner, path_or_id.as_ptr(), &mut bank).to_result()?;
        }
        Ok(bank.into())
    }

    /// Retrieves a loaded bank.
    pub fn get_bank_by_id(&self, id: Guid) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetBankByID(self.inner, &id.into(), &mut bank).to_result()?;
        }
        Ok(bank.into())
    }

    /// Retrieves the number of loaded banks.
    pub fn bank_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_System_GetBankCount(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    pub fn get_bank_list(&self) -> Result<Vec<Bank>> {
        let expected_count = self.bank_count()?;
        let mut count = 0;
        let mut list = vec![
            Bank {
                inner: std::ptr::null_mut()
            };
            expected_count as usize
        ];

        unsafe {
            FMOD_Studio_System_GetBankList(
                self.inner,
                // bank is repr transparent and has the same layout as *mut FMOD_STUDIO_BANK, so this cast is ok
                list.as_mut_ptr().cast::<*mut FMOD_STUDIO_BANK>(),
                list.capacity() as c_int,
                &mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(list)
        }
    }
}
