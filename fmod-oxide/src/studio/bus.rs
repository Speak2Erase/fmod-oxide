// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_uint},
    mem::MaybeUninit,
    ptr::NonNull,
};

use fmod_sys::*;
use lanyard::Utf8CString;

use crate::{core::ChannelGroup, Guid};

use super::{MemoryUsage, StopMode};

/// Represents a global mixer bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Bus {
    pub(crate) inner: NonNull<FMOD_STUDIO_BUS>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for Bus {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for Bus {}

impl From<*mut FMOD_STUDIO_BUS> for Bus {
    fn from(value: *mut FMOD_STUDIO_BUS) -> Self {
        let inner = NonNull::new(value).unwrap();
        Bus { inner }
    }
}

impl From<Bus> for *mut FMOD_STUDIO_BUS {
    fn from(value: Bus) -> Self {
        value.inner.as_ptr()
    }
}

impl Bus {
    /// Sets the pause state.
    ///
    /// This function allows pausing/unpausing of all audio routed into the bus.
    ///
    /// An individual pause state is kept for each bus.
    /// Pausing a bus will override the pause state of its inputs (meaning they return true from [`Bus::get_paused`]), while unpausing a bus will cause its inputs to obey their individual pause state.
    /// The pause state is processed in the Studio system update, so [`Bus::get_paused`] will return the state as determined by the last update.
    pub fn set_paused(&self, paused: bool) -> Result<()> {
        unsafe { FMOD_Studio_Bus_SetPaused(self.inner.as_ptr(), paused.into()).to_result() }
    }

    /// Retrieves the pause state.
    pub fn get_paused(&self) -> Result<bool> {
        let mut paused = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_Bus_GetPaused(self.inner.as_ptr(), &mut paused).to_result()?;
        }
        Ok(paused.into())
    }

    /// Stops all event instances that are routed into the bus.
    pub fn stop_all_events(&self, stop_mode: StopMode) -> Result<()> {
        unsafe { FMOD_Studio_Bus_StopAllEvents(self.inner.as_ptr(), stop_mode.into()).to_result() }
    }
}

impl Bus {
    /// Sets the volume level.
    ///          
    /// This volume is applied as a scaling factor to the volume level set in FMOD Studio.
    pub fn set_volume(&self, volume: c_float) -> Result<()> {
        unsafe { FMOD_Studio_Bus_SetVolume(self.inner.as_ptr(), volume).to_result() }
    }

    /// Retrieves the volume level.
    ///
    /// The second tuple field is calculated by combining the volume set via [`Bus::set_volume`] with the bus's default volume and any snapshots or [`super::Vca`]s that affect the bus.
    /// Volume changes are processed in the Studio system update, so second field will be the value calculated by the last update.
    pub fn get_volume(&self) -> Result<(c_float, c_float)> {
        let mut volume = 0.0;
        let mut final_volume = 0.0;
        unsafe {
            FMOD_Studio_Bus_GetVolume(self.inner.as_ptr(), &mut volume, &mut final_volume)
                .to_result()?;
        }
        Ok((volume, final_volume))
    }

    /// Sets the mute state.
    ///
    /// Mute is an additional control for volume, the effect of which is equivalent to setting the volume to zero.
    ///
    /// An individual mute state is kept for each bus.
    /// Muting a bus will override the mute state of its inputs (meaning they return true from [`Bus::get_mute`]), while unmuting a bus will cause its inputs to obey their individual mute state.
    /// The mute state is processed in the Studio system update, so [`Bus::get_mute`] will return the state as determined by the last update.
    pub fn set_mute(&self, mute: bool) -> Result<()> {
        unsafe { FMOD_Studio_Bus_SetMute(self.inner.as_ptr(), mute.into()).to_result() }
    }

    /// Retrieves the mute state.
    pub fn get_mute(&self) -> Result<bool> {
        let mut mute = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_Bus_GetMute(self.inner.as_ptr(), &mut mute).to_result()?;
        }
        Ok(mute.into())
    }

    /// Sets the port index to use when attaching to an output port.
    ///
    /// When a bus which is an output port is instantiated it will be connected to an output port based on the port type set in Studio.
    /// For some port types a platform specific port index is required to connect to the correct output port.
    /// For example, if the output port type is a speaker in a controller then a platform specific port index may be required to specify which controller the bus is to attach to.
    /// In such a case call this function passing the platform specific port index.
    ///
    /// There is no need to call this function for port types which do not require an index.
    ///
    /// This function may be called at any time after a bank containing the bus has been loaded.
    pub fn set_port_index(&self, index: FMOD_PORT_INDEX) -> Result<()> {
        unsafe { FMOD_Studio_Bus_SetPortIndex(self.inner.as_ptr(), index).to_result() }
    }

    /// Retrieves the port index assigned to the bus.
    pub fn get_port_index(&self) -> Result<FMOD_PORT_INDEX> {
        let mut index = 0;
        unsafe {
            FMOD_Studio_Bus_GetPortIndex(self.inner.as_ptr(), &mut index).to_result()?;
        }
        Ok(index)
    }
}

impl Bus {
    /// Retrieves the core [`ChannelGroup`].
    ///
    /// By default the [`ChannelGroup`] will only exist when it is needed; see Signal Paths in the FMOD documentation for details.
    /// If the [`ChannelGroup`] does not exist, this function will return [`FMOD_RESULT::FMOD_ERR_STUDIO_NOT_LOADED`].
    pub fn get_channel_group(&self) -> Result<ChannelGroup> {
        let mut channel_group = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_Bus_GetChannelGroup(self.inner.as_ptr(), &mut channel_group).to_result()?;
        }
        Ok(channel_group.into())
    }

    /// Locks the core [`ChannelGroup`].
    ///
    /// This function forces the system to create the [`ChannelGroup`] and keep it available until [`Bus::unlock_channel_group`] is called.
    /// See Signal Paths in the FMOD documentation for details.
    ///
    /// The [`ChannelGroup`] may not be available immediately after calling this function.
    /// When Studio has been initialized in asynchronous mode, the [`ChannelGroup`] will not be created until the command has been executed in the async thread.
    /// When Studio has been initialized with [`super::InitFlags::SYNCHRONOUS_UPDATE`], the [`ChannelGroup`] will be created in the next [`super::System::update`] call.
    ///
    /// You can call [`super::System::flush_commands`] to ensure the [`ChannelGroup`] has been created.
    /// Alternatively you can keep trying to obtain the [`ChannelGroup`] via [`Bus::get_channel_group`] until it is ready.
    pub fn lock_channel_group(&self) -> Result<()> {
        unsafe { FMOD_Studio_Bus_LockChannelGroup(self.inner.as_ptr()).to_result() }
    }

    /// Unlocks the core [`ChannelGroup`].
    ///
    /// This function allows the system to destroy the [`ChannelGroup`] when it is not needed.
    /// See Signal Paths in the FMOD documentation for details.
    pub fn unlock_channel_group(&self) -> Result<()> {
        unsafe { FMOD_Studio_Bus_UnlockChannelGroup(self.inner.as_ptr()).to_result() }
    }
}

impl Bus {
    /// Retrieves the bus CPU usage data.
    ///
    /// The first tuple field is the CPU time spent processing the events of this bus, in microseconds.
    ///
    /// The second tuple field is the CPU time spent processing the events and all input buses of this bus, in microseconds.
    ///
    /// [`crate::InitFlags::PROFILE_ENABLE`] with [`crate::SystemBuilder::build`] is required to call this function.
    pub fn get_cpu_usage(&self) -> Result<(c_uint, c_uint)> {
        let mut exclusive = 0;
        let mut inclusive = 0;
        unsafe {
            FMOD_Studio_Bus_GetCPUUsage(self.inner.as_ptr(), &mut exclusive, &mut inclusive)
                .to_result()?;
        }
        Ok((exclusive, inclusive))
    }

    /// Retrieves memory usage statistics.
    ///
    /// Memory usage statistics are only available in logging builds, in release builds the return value will contain zero for all values after calling this function.
    pub fn get_memory_usage(&self) -> Result<MemoryUsage> {
        let mut memory_usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_Bus_GetMemoryUsage(self.inner.as_ptr(), memory_usage.as_mut_ptr())
                .to_result()?;

            let memory_usage = memory_usage.assume_init().into();
            Ok(memory_usage)
        }
    }
}

impl Bus {
    /// Retrieves the GUID.
    pub fn get_id(&self) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_Bus_GetID(self.inner.as_ptr(), guid.as_mut_ptr()).to_result()?;

            let guid = guid.assume_init().into();

            Ok(guid)
        }
    }

    /// Retrieves the path.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    // TODO: convert into possible macro for the sake of reusing code
    pub fn get_path(&self) -> Result<Utf8CString> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_Bus_GetPath(
                self.inner.as_ptr(),
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_Bus_GetPath(
                self.inner.as_ptr(),
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = Utf8CString::from_utf8_with_nul_unchecked(path);

            Ok(path)
        }
    }

    /// Checks that the [`Bus`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_Bus_IsValid(self.inner.as_ptr()).into() }
    }
}
