use std::ffi::{c_float, c_int, c_uint, CStr};

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

mod bank;
pub use bank::*;

mod bus;
pub use bus::*;

mod system;
pub use system::*;

mod event_description;
pub use event_description::*;

mod vca;
pub use vca::*;

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LoadingState {
    Unloading = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADING,
    Unloaded = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADED,
    Loading = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADING,
    Loaded = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADED,
    Error = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_ERROR,
}

impl From<FMOD_STUDIO_LOADING_STATE> for LoadingState {
    fn from(value: FMOD_STUDIO_LOADING_STATE) -> Self {
        match value {
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADING => {
                LoadingState::Unloading
            }
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADED => LoadingState::Unloaded,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADING => LoadingState::Loading,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADED => LoadingState::Loaded,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_ERROR => LoadingState::Error,
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<LoadingState> for FMOD_STUDIO_LOADING_STATE {
    fn from(value: LoadingState) -> Self {
        value as FMOD_STUDIO_LOADING_STATE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StopMode {
    AllowFadeout = FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_ALLOWFADEOUT,
    Immediate = FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_IMMEDIATE,
}

impl From<FMOD_STUDIO_STOP_MODE> for StopMode {
    fn from(value: FMOD_STUDIO_STOP_MODE) -> Self {
        match value {
            FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_ALLOWFADEOUT => StopMode::AllowFadeout,
            FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_IMMEDIATE => StopMode::Immediate,
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<StopMode> for FMOD_STUDIO_STOP_MODE {
    fn from(value: StopMode) -> Self {
        value as FMOD_STUDIO_STOP_MODE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryUsage {
    pub exclusive: c_int,
    pub inclusive: c_int,
    pub sample_data: c_int,
}

impl From<FMOD_STUDIO_MEMORY_USAGE> for MemoryUsage {
    fn from(value: FMOD_STUDIO_MEMORY_USAGE) -> Self {
        MemoryUsage {
            exclusive: value.exclusive,
            inclusive: value.inclusive,
            sample_data: value.sampledata,
        }
    }
}

impl From<MemoryUsage> for FMOD_STUDIO_MEMORY_USAGE {
    fn from(value: MemoryUsage) -> Self {
        FMOD_STUDIO_MEMORY_USAGE {
            exclusive: value.exclusive,
            inclusive: value.inclusive,
            sampledata: value.sample_data,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// force this type to have the exact same layout as FMOD_STUDIO_PARAMETER_ID so we can safely transmute between them.
#[repr(C)]
pub struct ParameterID {
    pub data_1: c_uint,
    pub data_2: c_uint,
}

impl From<FMOD_STUDIO_PARAMETER_ID> for ParameterID {
    fn from(value: FMOD_STUDIO_PARAMETER_ID) -> Self {
        ParameterID {
            data_1: value.data1,
            data_2: value.data2,
        }
    }
}

impl From<ParameterID> for FMOD_STUDIO_PARAMETER_ID {
    fn from(value: ParameterID) -> Self {
        FMOD_STUDIO_PARAMETER_ID {
            data1: value.data_1,
            data2: value.data_2,
        }
    }
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
}

impl From<FMOD_STUDIO_INITFLAGS> for InitFlags {
    fn from(value: FMOD_STUDIO_INITFLAGS) -> Self {
        InitFlags::from_bits_truncate(value)
    }
}

impl From<InitFlags> for FMOD_STUDIO_INITFLAGS {
    fn from(value: InitFlags) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LoadBankFlags: c_uint {
        const NORMAL             = FMOD_STUDIO_LOAD_BANK_NORMAL;
        const NONBLOCKING        = FMOD_STUDIO_LOAD_BANK_NONBLOCKING;
        const DECOMPRESS_SAMPLES = FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES;
        const UNENCRYPTED        = FMOD_STUDIO_LOAD_BANK_UNENCRYPTED;
    }
}

impl From<FMOD_STUDIO_LOAD_BANK_FLAGS> for LoadBankFlags {
    fn from(value: FMOD_STUDIO_LOAD_BANK_FLAGS) -> Self {
        LoadBankFlags::from_bits_truncate(value)
    }
}

impl From<LoadBankFlags> for FMOD_STUDIO_LOAD_BANK_FLAGS {
    fn from(value: LoadBankFlags) -> Self {
        value.bits()
    }
}

// default impl is ok, all values are zero or none.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct AdvancedSettings {
    pub command_queue_size: c_uint,
    pub handle_initial_size: c_uint,
    pub studioupdateperiod: c_int,
    pub idle_sample_data_pool_size: c_int,
    pub streaming_schedule_delay: c_uint,
    // TODO: lifetime requirements for this struct?
    // fmod might copy this to a managed string, so we can relax the 'static
    pub encryption_key: Option<&'static CStr>,
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

#[derive(Clone, Copy, PartialEq)]
pub struct ParameterDescription {
    // FIXME this 'static is WRONG, figure out lifetimes!!!
    // TODO change to regular str
    // (probably hard to do because of null termination. perhaps we should add our own utf-8 null terminated string type?)
    name: &'static CStr,
    id: ParameterID,
    minimum: c_float,
    maximum: c_float,
    default_value: c_float,
    kind: ParameterKind,
    flags: ParameterFlags,
    guid: Guid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ParameterKind {
    GameControlled = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_GAME_CONTROLLED,
    AutomaticDistance = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE,
    AutomaticEventConeAngle =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_CONE_ANGLE,
    AutomaticEventOrientation =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_ORIENTATION,
    AutomaticDirection = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DIRECTION,
    AutomaticElevation = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_ELEVATION,
    AutomaticListenerOrientation =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_LISTENER_ORIENTATION,
    AutomaticSpeed = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED,
    AutomaticSpeedAbsolute =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED_ABSOLUTE,
    AutomaticDistanceNormalized =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE_NORMALIZED,
}

impl From<FMOD_STUDIO_PARAMETER_TYPE> for ParameterKind {
    fn from(value: FMOD_STUDIO_PARAMETER_TYPE) -> Self {
        match value {
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_GAME_CONTROLLED => {
                ParameterKind::GameControlled
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE => {
                ParameterKind::AutomaticDistance
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_CONE_ANGLE => {
                ParameterKind::AutomaticEventConeAngle
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_ORIENTATION => {
                ParameterKind::AutomaticEventOrientation
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DIRECTION => {
                ParameterKind::AutomaticDirection
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_ELEVATION => {
                ParameterKind::AutomaticElevation
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_LISTENER_ORIENTATION => {
                ParameterKind::AutomaticListenerOrientation
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED => {
                ParameterKind::AutomaticSpeed
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED_ABSOLUTE => {
                ParameterKind::AutomaticSpeedAbsolute
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE_NORMALIZED => {
                ParameterKind::AutomaticDistanceNormalized
            }
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<ParameterKind> for FMOD_STUDIO_PARAMETER_TYPE {
    fn from(value: ParameterKind) -> Self {
        value as FMOD_STUDIO_PARAMETER_TYPE
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ParameterFlags: c_uint {
        const READONLY = FMOD_STUDIO_PARAMETER_READONLY;
        const AUTOMATIC = FMOD_STUDIO_PARAMETER_AUTOMATIC;
        const GLOBAL = FMOD_STUDIO_PARAMETER_GLOBAL;
        const DISCRETE = FMOD_STUDIO_PARAMETER_DISCRETE;
        const LABELED = FMOD_STUDIO_PARAMETER_LABELED;
    }
}

impl From<FMOD_STUDIO_PARAMETER_FLAGS> for ParameterFlags {
    fn from(value: FMOD_STUDIO_PARAMETER_FLAGS) -> Self {
        ParameterFlags::from_bits_truncate(value)
    }
}

impl From<ParameterFlags> for FMOD_STUDIO_PARAMETER_FLAGS {
    fn from(value: ParameterFlags) -> Self {
        value.bits()
    }
}

// It's safe to go from ParameterDescription to FMOD_STUDIO_PARAMETER_DESCRIPTION because a &'static CStr meets all the safety FMOD expects. (aligned, null terminated, etc)
impl From<ParameterDescription> for FMOD_STUDIO_PARAMETER_DESCRIPTION {
    fn from(value: ParameterDescription) -> Self {
        FMOD_STUDIO_PARAMETER_DESCRIPTION {
            name: value.name.as_ptr(),
            id: value.id.into(),
            minimum: value.minimum,
            maximum: value.maximum,
            defaultvalue: value.default_value,
            type_: value.kind.into(),
            flags: value.flags.into(),
            guid: value.guid.into(),
        }
    }
}

impl ParameterDescription {
    /// Create a safe [`ParameterDescription`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// The name from [`FMOD_STUDIO_PARAMETER_DESCRIPTION`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`CStr::from_ptr`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_PARAMETER_DESCRIPTION) -> ParameterDescription {
        unsafe {
            ParameterDescription {
                name: CStr::from_ptr(value.name),
                id: value.id.into(),
                minimum: value.minimum,
                maximum: value.maximum,
                default_value: value.defaultvalue,
                kind: value.type_.into(),
                flags: value.flags.into(),
                guid: value.guid.into(),
            }
        }
    }
}
