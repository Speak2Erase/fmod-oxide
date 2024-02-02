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
    any::Any,
    ffi::{c_float, c_int, c_uint, CStr},
    mem::MaybeUninit,
    os::raw::c_void,
    sync::Arc,
};

use crate::{core, Attributes3D, Guid, Vector};

use super::{
    AdvancedSettings, Bank, BufferUsage, Bus, CommandCaptureFlags, CommandReplay,
    CommandReplayFlags, EventDescription, InitFlags, LoadBankFlags, MemoryUsage,
    ParameterDescription, ParameterID, SoundInfo, SystemCallbackKind, SystemCallbackMask, Vca,
};

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

pub(crate) struct InternalUserdata {
    // we don't expose the callback at all so it's fine to just use a box
    callback: Option<Box<CallbackFn>>,
    // this is an arc in case someone releases the system while holding onto a reference to the userdata
    userdata: Option<Userdata>,
    // used to ensure we don't misfire callbacks (we always subscribe to unloading banks to ensure userdata is freed)
    enabled_callbacks: SystemCallbackMask,
}

// hilariously long type signature because clippy
type CallbackFn = dyn Fn(System, SystemCallbackKind, Option<Userdata>) -> Result<()> + Send + Sync;
type Userdata = Arc<dyn Any + Send + Sync>;

unsafe extern "C" fn internal_callback(
    system: *mut FMOD_STUDIO_SYSTEM,
    kind: FMOD_STUDIO_SYSTEM_CALLBACK_TYPE,
    command_data: *mut c_void,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let mut result = FMOD_RESULT::FMOD_OK;

    // FIXME: handle unwinding panics

    // userdata should always be InternalUserdata, and if not null, it should be a valid reference to InternalUserdata
    // FIXME: this as_ref() might violate rust aliasing rules, should we use UnsafeCell?
    if let Some(internal_userdata) = unsafe { userdata.cast::<InternalUserdata>().as_ref() } {
        if let Some(callback) = &internal_userdata.callback {
            if internal_userdata.enabled_callbacks.contains(kind.into()) {
                let system = system.into();

                let kind = match kind {
                    FMOD_STUDIO_SYSTEM_CALLBACK_PREUPDATE => SystemCallbackKind::Preupdate,
                    FMOD_STUDIO_SYSTEM_CALLBACK_POSTUPDATE => SystemCallbackKind::Postupdate,
                    FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD => {
                        let bank = command_data.cast::<FMOD_STUDIO_BANK>().into();
                        SystemCallbackKind::BankUnload(bank)
                    }
                    FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_CONNECTED => {
                        SystemCallbackKind::LiveupdateConnected
                    }
                    FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_DISCONNECTED => {
                        SystemCallbackKind::LiveupdateDisconnected
                    }
                    _ => {
                        eprintln!("wrong system callback type {kind}, aborting");
                        std::process::abort()
                    }
                };

                let userdata = internal_userdata.userdata.clone();
                result = callback(system, kind, userdata).into();
            }
        }
    }

    if kind == FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD {
        let bank = Bank {
            inner: command_data.cast(),
        };
        if let Err(error) = deallocate_bank(bank) {
            eprintln!("error deallocating bank: {error}");
        }
    }

    result
}

fn deallocate_bank(bank: Bank) -> Result<()> {
    let mut userdata = std::ptr::null_mut();
    unsafe { FMOD_Studio_Bank_GetUserData(bank.inner, &mut userdata).to_result()? };

    // deallocate the userdata if it is not null
    if !userdata.is_null() {
        unsafe {
            let userdata = userdata.cast::<super::bank::InternalUserdata>();

            drop(Box::from_raw(userdata));

            FMOD_Studio_Bank_SetUserData(bank.inner, std::ptr::null_mut()).to_result()?;
        }
    }

    bank.load_sample_data()?;

    let list = bank.get_event_list()?;
    for event in list {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventDescription_GetUserData(event.inner, &mut userdata).to_result()?;
        };

        // deallocate the userdata if it is not null
        if !userdata.is_null() {
            unsafe {
                let userdata = userdata.cast::<super::event_description::InternalUserdata>();

                drop(Box::from_raw(userdata));

                FMOD_Studio_EventDescription_SetUserData(event.inner, std::ptr::null_mut())
                    .to_result()?;
            }
        }
    }

    Ok(())
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
        flags: crate::InitFlags,
    ) -> Result<System> {
        unsafe {
            // we don't need
            self.build_with_extra_driver_data(
                max_channels,
                studio_flags,
                flags,
                std::ptr::null_mut(),
            )
        }
    }

    /// # Safety
    ///
    /// See the FMOD docs explaining driver data for more safety information.
    pub unsafe fn build_with_extra_driver_data(
        self,
        max_channels: c_int,
        studio_flags: InitFlags,
        flags: crate::InitFlags,
        driver_data: *mut c_void,
    ) -> Result<System> {
        unsafe {
            FMOD_Studio_System_Initialize(
                self.system,
                max_channels,
                studio_flags.bits(),
                flags.bits(),
                driver_data,
            )
            .to_result()?;

            // ensure the callback is always set so we can properly deallocate bank userdata
            FMOD_Studio_System_SetCallback(
                self.system,
                Some(internal_callback),
                FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD,
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
        unsafe { SystemBuilder::new() }?.build(0, InitFlags::NORMAL, crate::InitFlags::NORMAL)
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
        unsafe {
            // fetch userdata before release is called
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_System_GetUserData(self.inner, &mut userdata).to_result()?;

            FMOD_Studio_System_Release(self.inner).to_result()?;

            // deallocate the userdata after the system is released
            if !userdata.is_null() {
                let userdata = Box::from_raw(userdata.cast::<InternalUserdata>());
                drop(userdata);
            }
        }
        Ok(())
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
        ids: &[ParameterID], // TODO fmod says that the size of this must range from 1-32. do we need to enforce this?
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

    /// Sets a global parameter value by name.
    pub fn set_parameter_by_name(
        &self,
        name: &CStr,
        value: c_float,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetParameterByName(
                self.inner,
                name.as_ptr(),
                value,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Sets a global parameter value by name, looking up the value label.
    ///
    /// If the specified label is not found, [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned. This lookup is case sensitive.
    pub fn set_parameter_by_name_with_label(
        &self,
        name: &CStr,
        label: &CStr,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetParameterByNameWithLabel(
                self.inner,
                name.as_ptr(),
                label.as_ptr(),
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Retrieves a global parameter by name or path.
    ///
    /// `name` can be the short name (such as `Wind`) or the full path (such as `parameter:/Ambience/Wind`).
    /// Path lookups will only succeed if the strings bank has been loaded.
    pub fn get_parameter_description_by_name(&self, name: &CStr) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetParameterDescriptionByName(
                self.inner,
                name.as_ptr(),
                description.as_mut_ptr(),
            )
            .to_result()?;

            // FIXME lifetimes are incorrect and MUST be relaxed from 'static
            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves a global parameter by ID.
    pub fn get_parameter_description_by_id(&self, id: ParameterID) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetParameterDescriptionByID(
                self.inner,
                id.into(),
                description.as_mut_ptr(),
            )
            .to_result()?;

            // FIXME lifetimes are incorrect and MUST be relaxed from 'static
            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves the number of global parameters.
    pub fn parameter_description_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_System_GetParameterDescriptionCount(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of global parameters.
    pub fn get_parameter_description_list(&self) -> Result<Vec<ParameterDescription>> {
        let expected_count = self.parameter_description_count()?;
        let mut count = 0;
        // FIXME: is the use of MaybeUninit necessary?
        // it does imply intention though, which is ok.
        let mut list = vec![MaybeUninit::zeroed(); expected_count as usize];

        unsafe {
            FMOD_Studio_System_GetParameterDescriptionList(
                self.inner,
                // bank is repr transparent and has the same layout as *mut FMOD_STUDIO_BANK, so this cast is ok
                list.as_mut_ptr()
                    .cast::<FMOD_STUDIO_PARAMETER_DESCRIPTION>(),
                list.capacity() as c_int,
                &mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            // FIXME lifetimes are incorrect and MUST be relaxed from 'static
            let list = list
                .into_iter()
                .map(|uninit| {
                    let description = uninit.assume_init();
                    ParameterDescription::from_ffi(description)
                })
                .collect();

            Ok(list)
        }
    }

    /// Retrieves a global parameter label by name or path.
    ///
    /// `name` can be the short name (such as `Wind`) or the full path (such as `parameter:/Ambience/Wind`).
    /// Path lookups will only succeed if the strings bank has been loaded.
    pub fn get_parameter_label_by_name(&self, name: &CStr, label_index: c_int) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_System_GetParameterLabelByName(
                self.inner,
                name.as_ptr(),
                label_index,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error.code != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_System_GetParameterLabelByName(
                self.inner,
                name.as_ptr(),
                label_index,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }

    /// Retrieves a global parameter label by ID.
    pub fn get_parameter_label_by_id(&self, id: ParameterID, label_index: c_int) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_System_GetParameterLabelByID(
                self.inner,
                id.into(),
                label_index,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error.code != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_System_GetParameterLabelByID(
                self.inner,
                id.into(),
                label_index,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }
}

impl System {
    /// Retrieves a loaded VCA.
    ///
    /// This function allows you to retrieve a handle for any VCA in the global mixer.
    ///
    /// `path_or_id` may be a path, such as `vca:/MyVCA`, or an ID string, such as `{d9982c58-a056-4e6c-b8e3-883854b4bffb`}.
    ///
    /// Note that path lookups will only succeed if the strings bank has been loaded.
    pub fn get_vca(&self, path_or_id: &CStr) -> Result<Vca> {
        let mut vca = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetVCA(self.inner, path_or_id.as_ptr(), &mut vca).to_result()?;
        }
        Ok(vca.into())
    }

    /// Retrieves a loaded VCA.
    ///
    /// This function allows you to retrieve a handle for any VCA in the global mixer.
    pub fn get_vca_by_id(&self, id: Guid) -> Result<Vca> {
        let mut vca = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetVCAByID(self.inner, &id.into(), &mut vca).to_result()?;
        }
        Ok(vca.into())
    }
}

impl System {
    /// Retrieves advanced settings.
    pub fn get_advanced_settings(&self) -> Result<AdvancedSettings> {
        let mut advanced_settings = MaybeUninit::zeroed();

        unsafe {
            FMOD_Studio_System_GetAdvancedSettings(self.inner, advanced_settings.as_mut_ptr())
                .to_result()?;

            // FIXME advancedsettings here is a 'static. this is probably invalid!
            let advanced_settings = AdvancedSettings::from_ffi(advanced_settings.assume_init());

            Ok(advanced_settings)
        }
    }
}

impl System {
    /// Recording Studio commands to a file.
    ///
    /// The commands generated by the FMOD Studio API can be captured and later replayed for debug and profiling purposes.
    ///
    /// Unless the [`CommandCaptureFlags::SKIP_INITIAL_STATE`] flag is specified, the command capture will first record the set of all banks and event instances that currently exist.
    pub fn start_command_capture(&self, filename: &CStr, flags: CommandCaptureFlags) -> Result<()> {
        unsafe {
            FMOD_Studio_System_StartCommandCapture(self.inner, filename.as_ptr(), flags.into())
                .to_result()
        }
    }

    /// Stop recording Studio commands.
    pub fn stop_command_capture(&self) -> Result<()> {
        unsafe { FMOD_Studio_System_StopCommandCapture(self.inner).to_result() }
    }

    /// Load a command replay.
    pub fn load_command_replay(
        &self,
        filename: &CStr,
        flags: CommandReplayFlags,
    ) -> Result<CommandReplay> {
        let mut replay = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_LoadCommandReplay(
                self.inner,
                filename.as_ptr(),
                flags.into(),
                &mut replay,
            )
            .to_result()?;
        }
        Ok(replay.into())
    }
}

impl System {
    /// Retrieves buffer usage information.
    ///
    /// Stall count and time values are cumulative. They can be reset by calling [`System::reset_buffer_usage`].
    ///
    /// Stalls due to the studio command queue overflowing can be avoided by setting a larger command queue size with [`SystemBuilder::settings`].
    pub fn get_buffer_usage(&self) -> Result<BufferUsage> {
        let mut usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetBufferUsage(self.inner, usage.as_mut_ptr()).to_result()?;

            let usage = usage.assume_init().into();
            Ok(usage)
        }
    }

    /// Resets memory buffer usage statistics.
    ///
    /// This function resets the buffer usage data tracked by the FMOD Studio System.
    pub fn reset_buffer_usage(&self) -> Result<()> {
        unsafe { FMOD_Studio_System_ResetBufferUsage(self.inner).to_result() }
    }

    /// Retrieves the amount of CPU used for different parts of the Studio engine.
    ///
    /// For readability, the percentage values are smoothed to provide a more stable output.
    pub fn get_cpu_usage(&self) -> Result<(super::CpuUsage, crate::CpuUsage)> {
        let mut usage = MaybeUninit::zeroed();
        let mut usage_core = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetCPUUsage(self.inner, usage.as_mut_ptr(), usage_core.as_mut_ptr())
                .to_result()?;

            let usage = usage.assume_init().into();
            let usage_core = usage_core.assume_init().into();
            Ok((usage, usage_core))
        }
    }

    /// Retrieves memory usage statistics.
    ///
    /// The memory usage `sample_data` field for the system is the total size of non-streaming sample data currently loaded.
    ///
    /// Memory usage statistics are only available in logging builds, in release builds memoryusage will contain zero for all values after calling this function.
    pub fn get_memory_usage(&self) -> Result<MemoryUsage> {
        let mut usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetMemoryUsage(self.inner, usage.as_mut_ptr()).to_result()?;

            let usage = usage.assume_init().into();
            Ok(usage)
        }
    }
}

impl System {
    /// Registers a plugin DSP.
    ///
    /// Plugin DSPs used by an event must be registered using this function before loading the bank containing the event.
    ///
    /// # Safety
    /// TODO
    pub unsafe fn register_plugin(&self) {
        todo!()
    }

    /// Unregisters a plugin DSP.
    ///
    /// # Safety
    /// TODO
    pub unsafe fn unregister_plugin(&self) {
        todo!()
    }
}

impl System {
    /// Sets a callback for the FMOD Studio System.
    ///
    /// The system callback function is called for a variety of reasons, use the callbackmask to choose which callbacks you are interested in.
    ///
    /// Callbacks are called from the Studio Update Thread in default / async mode and the main (calling) thread in synchronous mode. See the [`FMOD_STUDIO_SYSTEM_CALLBACK_TYPE`] for details.
    pub fn set_callback<F>(&self, callback: F, mask: SystemCallbackMask) -> Result<()>
    where
        F: Fn(System, SystemCallbackKind, Option<Userdata>) -> Result<()> + Send + Sync + 'static,
    {
        // Always enable BankUnload to deallocate any userdata attached to banks
        let raw_mask = (mask | SystemCallbackMask::BANK_UNLOAD).into();

        unsafe {
            let mut userdata = std::ptr::null_mut();

            FMOD_Studio_System_GetUserData(self.inner, &mut userdata).to_result()?;

            // create & set the userdata if we haven't already
            if userdata.is_null() {
                let boxed_userdata = Box::new(InternalUserdata {
                    callback: None,
                    enabled_callbacks: SystemCallbackMask::empty(),
                    userdata: None,
                });
                userdata = Box::into_raw(boxed_userdata).cast();

                FMOD_Studio_System_SetUserData(self.inner, userdata).to_result()?;
            }

            // userdata should ALWAYS be InternalUserdata
            let userdata = &mut *userdata.cast::<InternalUserdata>();
            userdata.callback = Some(Box::new(callback));
            userdata.enabled_callbacks = mask;

            // is this allowed to be null?
            FMOD_Studio_System_SetCallback(self.inner, Some(internal_callback), raw_mask)
                .to_result()
        }
    }

    /// Sets the user data.
    ///
    /// This function allows arbitrary user data to be attached to this object, which wll be passed through the userdata parameter in any [`FMOD_STUDIO_SYSTEM_CALLBACK`]s.
    /// The provided data may be shared/accessed from multiple threads, and so must implement Send + Sync 'static.
    pub fn set_user_data<T>(&self, data: Option<T>) -> Result<()>
    where
        T: Any + Send + Sync + 'static,
    {
        unsafe {
            let mut userdata = std::ptr::null_mut();

            FMOD_Studio_System_GetUserData(self.inner, &mut userdata).to_result()?;

            // create & set the userdata if we haven't already
            if userdata.is_null() {
                let boxed_userdata = Box::new(InternalUserdata {
                    callback: None,
                    enabled_callbacks: SystemCallbackMask::empty(),
                    userdata: None,
                });
                userdata = Box::into_raw(boxed_userdata).cast();

                FMOD_Studio_System_SetUserData(self.inner, userdata).to_result()?;
            }

            // userdata should ALWAYS be InternalUserdata
            let userdata = &mut *userdata.cast::<InternalUserdata>();
            userdata.userdata = data.map(|d| Arc::new(d) as _); // closure is necessary to unsize type
        }

        Ok(())
    }

    /// Retrieves the user data.
    ///
    /// This function allows arbitrary user data to be retrieved from this object.
    // TODO should we just return the dyn Userdata directly?
    pub fn get_user_data<T>(&self) -> Result<Option<Arc<T>>>
    where
        T: Any + Send + Sync + 'static,
    {
        unsafe {
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_System_GetUserData(self.inner, &mut userdata).to_result()?;

            if userdata.is_null() {
                return Ok(None);
            }

            // userdata should ALWAYS be InternalUserdata
            let userdata = &mut *userdata.cast::<InternalUserdata>();
            let userdata = userdata
                .userdata
                .clone()
                .map(Arc::downcast::<T>)
                .and_then(std::result::Result::ok);
            Ok(userdata)
        }
    }
}

impl System {
    /// Retrieves information for loading a sound from the audio table.
    ///
    /// The [`SoundInfo`] structure contains information to be passed to [`crate::System::create_sound`] (which will create a parent sound),
    /// along with a subsound index to be passed to [`crate::Sound::get_sub_sound`] once the parent sound is loaded.
    ///
    /// The user is expected to call [`System::create_sound `]with the given information.
    /// It is up to the user to combine in any desired loading flags, such as [`FMOD_CREATESTREAM`], [`FMOD_CREATECOMPRESSEDSAMPLE`] or [`FMOD_NONBLOCKING`] with the flags in [`FMOD_STUDIO_SOUND_INFO::mode`].
    ///
    /// When the banks have been loaded via [`System::load_bank_memory`], the mode will be returned as [`FMOD_OPENMEMORY_POINT`].
    /// This won't work with the default [`FMOD_CREATESAMPLE`] mode.
    /// For memory banks, you should add in the [`FMOD_CREATECOMPRESSEDSAMPLE`] or [`FMOD_CREATESTREAM`] flag, or remove [`FMOD_OPENMEMORY_POINT`] and add [`FMOD_OPENMEMORY`] to decompress the sample into a new allocation.
    // TODO flags
    pub fn get_sound_info(&self, key: &CStr) -> Result<SoundInfo> {
        let mut sound_info = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetSoundInfo(self.inner, key.as_ptr(), sound_info.as_mut_ptr())
                .to_result()?;

            let sound_info = SoundInfo::from_ffi(sound_info.assume_init());
            Ok(sound_info)
        }
    }
}

impl System {
    /// Retrieves the Core System.
    pub fn get_core_system(&self) -> Result<core::System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetCoreSystem(self.inner, &mut system).to_result()?;
        }
        Ok(system.into())
    }
}

impl System {
    /// Retrieves the ID for a bank, event, snapshot, bus or VCA.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    ///
    /// The path can be copied to the system clipboard from FMOD Studio using the "Copy Path" context menu command.
    pub fn lookup_id(&self, path: &CStr) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_LookupID(self.inner, path.as_ptr(), guid.as_mut_ptr())
                .to_result()?;

            let guid = guid.assume_init().into();
            Ok(guid)
        }
    }

    /// Retrieves the path for a bank, event, snapshot, bus or VCA.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    pub fn lookup_path(&self, id: Guid) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_System_LookupPath(
                self.inner,
                &id.into(),
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error.code != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_System_LookupPath(
                self.inner,
                &id.into(),
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }

    /// Checks that the [`System`] reference is valid and has been initialized.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_System_IsValid(self.inner).into() }
    }
}
