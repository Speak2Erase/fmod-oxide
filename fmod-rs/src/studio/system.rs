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
use std::{
    ffi::{c_float, c_int, c_uint, CStr},
    mem::MaybeUninit,
};

use crate::{Attributes3D, Guid, Vector};

use super::{AdvancedSettings, Bank, Bus, EventDescription, InitFlags, LoadBankFlags, ParameterID};

/// The main system object for FMOD Studio.
///
/// Initializing the FMOD Studio System object will also initialize the core System object.
///
/// Created with [`SystemBuilder`], which handles initialization for you.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // TODO: should this logically be copy?
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

    // TODO: move to a core system builder
    pub fn software_format(
        self,
        sample_rate: c_int,
        speaker_mode: FMOD_SPEAKERMODE, // todo convert to enum
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
        flags: FMOD_INITFLAGS, // todo core init flags
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

// TODO: could we solve this with an "owned" system and a shared system?
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
    // TODO: take &self or not?
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
    // TODO: load bank with callbacks
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

impl System {
    /// Sets the 3D attributes of the listener.
    pub fn set_listener_attributes(
        &self,
        listener: c_int,
        attributes: Attributes3D,
        attenuation_position: Option<Vector>,
    ) -> Result<()> {
        // we need to do this conversion seperately, for lifetime reasons
        let attenuation_position = attenuation_position.map(Into::into);
        unsafe {
            FMOD_Studio_System_SetListenerAttributes(
                self.inner,
                listener,
                &attributes.into(),
                attenuation_position
                    .as_ref()
                    .map_or(std::ptr::null(), |a| a as *const _),
            )
            .to_result()
        }
    }

    /// Retrieves listener 3D attributes.
    pub fn get_listener_attributes(&self, listener: c_int) -> Result<(Attributes3D, Vector)> {
        let mut attributes = MaybeUninit::uninit();
        let mut attenuation_position = MaybeUninit::uninit();

        unsafe {
            FMOD_Studio_System_GetListenerAttributes(
                self.inner,
                listener,
                attributes.as_mut_ptr(),
                attenuation_position.as_mut_ptr(),
            )
            .to_result()?;

            // TODO: check safety
            Ok((
                attributes.assume_init().into(),
                attenuation_position.assume_init().into(),
            ))
        }
    }

    /// Sets the listener weighting.
    ///
    /// Listener weighting is a factor which determines how much the listener influences the mix.
    /// It is taken into account for 3D panning, doppler, and the automatic distance event parameter. A listener with a weight of 0 has no effect on the mix.
    ///
    /// Listener weighting can be used to fade in and out multiple listeners.
    /// For example to do a crossfade, an additional listener can be created with a weighting of 0 that ramps up to 1 while the old listener weight is ramped down to 0.
    /// After the crossfade is finished the number of listeners can be reduced to 1 again.
    ///
    /// The sum of all the listener weights should add up to at least 1. It is a user error to set all listener weights to 0.
    pub fn set_listener_weight(&self, listener: c_int, weight: c_float) -> Result<()> {
        unsafe { FMOD_Studio_System_SetListenerWeight(self.inner, listener, weight).to_result() }
    }

    /// Retrieves listener weighting.
    pub fn get_listener_weight(&self, listener: c_int) -> Result<c_float> {
        let mut weight = 0.0;
        unsafe {
            FMOD_Studio_System_GetListenerWeight(self.inner, listener, &mut weight).to_result()?;
        }
        Ok(weight)
    }

    /// Sets the number of listeners in the 3D sound scene.
    ///
    /// If the number of listeners is set to more than 1 then FMOD uses a 'closest sound to the listener' method to determine what should be heard.
    pub fn set_listener_count(&self, amount: c_int) -> Result<()> {
        unsafe { FMOD_Studio_System_SetNumListeners(self.inner, amount).to_result() }
    }

    /// Sets the number of listeners in the 3D sound scene.
    ///
    /// If the number of listeners is set to more than 1 then FMOD uses a 'closest sound to the listener' method to determine what should be heard.
    pub fn get_listener_count(&self) -> Result<c_int> {
        let mut amount = 0;
        unsafe {
            FMOD_Studio_System_GetNumListeners(self.inner, &mut amount).to_result()?;
        }
        Ok(amount)
    }
}

impl System {
    /// Retrieves a loaded [`Bus`].
    ///
    /// This function allows you to retrieve a handle for any bus in the global mixer.
    ///
    /// `path_or_id` may be a path, such as `bus:/SFX/Ambience`, or an ID string, such as `{d9982c58-a056-4e6c-b8e3-883854b4bffb}`.
    ///
    /// Note that path lookups will only succeed if the strings bank has been loaded.
    pub fn get_bus(&self, path_or_id: &CStr) -> Result<Bus> {
        let mut bus = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetBus(self.inner, path_or_id.as_ptr(), &mut bus).to_result()?;
        }
        Ok(bus.into())
    }

    /// Retrieves a loaded [`Bus`].
    ///
    /// This function allows you to retrieve a handle for any bus in the global mixer.
    pub fn get_bus_by_id(&self, id: Guid) -> Result<Bus> {
        let mut bus = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetBusByID(self.inner, &id.into(), &mut bus).to_result()?;
        }
        Ok(bus.into())
    }
}

impl System {
    /// Retrieves an [`EventDescription`].
    ///
    /// This function allows you to retrieve a handle to any loaded event description.
    ///
    /// `path+or_id` may be a path, such as `event:/UI/Cancel` or `snapshot:/IngamePause`, or an ID string, such as `{2a3e48e6-94fc-4363-9468-33d2dd4d7b00}`.
    ///
    /// Note that path lookups will only succeed if the strings bank has been loaded.
    pub fn get_event(&self, path_or_id: &CStr) -> Result<EventDescription> {
        let mut event = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetEvent(self.inner, path_or_id.as_ptr(), &mut event).to_result()?;
        }
        Ok(event.into())
    }

    /// Retrieves an [`EventDescription`].
    ///
    /// This function allows you to retrieve a handle to any loaded event description.
    pub fn get_event_by_id(&self, id: Guid) -> Result<EventDescription> {
        let mut event = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetEventByID(self.inner, &id.into(), &mut event).to_result()?;
        }
        Ok(event.into())
    }
}

impl System {
    /// Retrieves a global parameter value by unique identifier.
    ///
    /// The second tuple field is the final value of the parameter after applying adjustments due to automation, modulation, seek speed, and parameter velocity to value.
    /// This is calculated asynchronously when the Studio system updates.
    pub fn get_parameter_by_id(&self, id: ParameterID) -> Result<(c_float, c_float)> {
        let mut value = 0.0;
        let mut final_value = 0.0;

        unsafe {
            FMOD_Studio_System_GetParameterByID(
                self.inner,
                id.into(),
                &mut value,
                &mut final_value,
            )
            .to_result()?;
        }

        Ok((value, final_value))
    }

    /// Sets a global parameter value by unique identifier.
    pub fn set_parameter_by_id(
        &self,
        id: ParameterID,
        value: c_float,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetParameterByID(
                self.inner,
                id.into(),
                value,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Sets a global parameter value by unique identifier, looking up the value label.
    ///
    /// If the specified label is not found, [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    /// This lookup is case sensitive.
    pub fn set_parameter_by_id_with_label(
        &self,
        id: ParameterID,
        label: &CStr,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetParameterByIDWithLabel(
                self.inner,
                id.into(),
                label.as_ptr(),
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Sets multiple global parameter values by unique identifier.
    ///
    /// If any ID is set to all zeroes then the corresponding value will be ignored.
    // TODO iterator version?
    pub fn set_parameters_by_ids(
        &self,
        ids: &[ParameterID],
        values: &mut [c_float], // TODO is this &mut correct? does fmod perform any writes?
        ignore_seek_speed: bool,
    ) -> Result<()> {
        // TODO don't panic, return result
        assert_eq!(ids.len(), values.len());

        unsafe {
            FMOD_Studio_System_SetParametersByIDs(
                self.inner,
                ids.as_ptr().cast(),
                values.as_mut_ptr(),
                ids.len() as c_int,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Retrieves a global parameter value by name.
    ///
    /// The second tuple field is the final value of the parameter after applying adjustments due to automation, modulation, seek speed, and parameter velocity to value.
    /// This is calculated asynchronously when the Studio system updates.
    pub fn get_parameter_by_name(&self, name: &CStr) -> Result<(c_float, c_float)> {
        let mut value = 0.0;
        let mut final_value = 0.0;

        unsafe {
            FMOD_Studio_System_GetParameterByName(
                self.inner,
                name.as_ptr(),
                &mut value,
                &mut final_value,
            )
            .to_result()?;
        }

        Ok((value, final_value))
    }
}
