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

mod command_replay;
pub use command_replay::*;

mod event;
pub use event::*;

mod vca;
pub use vca::*;

use crate::{
    core::{Dsp, Sound},
    Guid, UserdataTypes,
};

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
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct AdvancedSettings {
    pub command_queue_size: c_uint,
    pub handle_initial_size: c_uint,
    pub studioupdateperiod: c_int,
    pub idle_sample_data_pool_size: c_int,
    pub streaming_schedule_delay: c_uint,
    // TODO: lifetime requirements for this struct?
    // fmod might copy this to a managed string, so we can relax the 'static
    // TODO: set custom memory alloc callbacks to see if/how fmod copies strings
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ParameterDescription {
    // FIXME this 'static is WRONG, figure out lifetimes!!!
    // TODO change to regular str
    // (probably hard to do because of null termination. perhaps we should add our own utf-8 null terminated string type?)
    pub name: &'static CStr,
    pub id: ParameterID,
    pub minimum: c_float,
    pub maximum: c_float,
    pub default_value: c_float,
    pub kind: ParameterKind,
    pub flags: ParameterFlags,
    pub guid: Guid,
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct UserProperty {
    // FIXME this 'static is WRONG, figure out lifetimes!!!
    pub name: &'static CStr,
    pub kind: UserPropertyKind,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UserPropertyKind {
    Int(c_int),
    Bool(bool),
    Float(c_float),
    // FIXME this 'static is WRONG, figure out lifetimes!!!
    String(&'static CStr),
}

impl UserProperty {
    /// Create a safe [`UserPropertyKind`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// All string values from the FFI struct must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    /// The type field must match the type assigned to the union.
    ///
    /// See [`CStr::from_ptr`] for more information.
    unsafe fn from_ffi(value: FMOD_STUDIO_USER_PROPERTY) -> Self {
        unsafe {
            UserProperty {
                name: CStr::from_ptr(value.name),
                kind: match value.type_ {
                    FMOD_STUDIO_USER_PROPERTY_TYPE_FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER => {
                        UserPropertyKind::Int(value.__bindgen_anon_1.intvalue)
                    }
                    FMOD_STUDIO_USER_PROPERTY_TYPE_FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN => {
                        UserPropertyKind::Bool(value.__bindgen_anon_1.boolvalue.into())
                    }
                    FMOD_STUDIO_USER_PROPERTY_TYPE_FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT => {
                        UserPropertyKind::Float(value.__bindgen_anon_1.floatvalue)
                    }
                    FMOD_STUDIO_USER_PROPERTY_TYPE_FMOD_STUDIO_USER_PROPERTY_TYPE_STRING => {
                        UserPropertyKind::String(CStr::from_ptr(value.__bindgen_anon_1.stringvalue))
                    }
                    v => panic!("invalid user property type {v}"),
                },
            }
        }
    }
}

impl From<UserProperty> for FMOD_STUDIO_USER_PROPERTY {
    fn from(value: UserProperty) -> Self {
        let (kind, union) = match value.kind {
            UserPropertyKind::Int(v) => (
                FMOD_STUDIO_USER_PROPERTY_TYPE_FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER,
                FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 { intvalue: v },
            ),
            UserPropertyKind::Bool(v) => (
                FMOD_STUDIO_USER_PROPERTY_TYPE_FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN,
                FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 {
                    boolvalue: v.into(),
                },
            ),
            UserPropertyKind::Float(v) => (
                FMOD_STUDIO_USER_PROPERTY_TYPE_FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT,
                FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 { floatvalue: v },
            ),
            UserPropertyKind::String(v) => (
                FMOD_STUDIO_USER_PROPERTY_TYPE_FMOD_STUDIO_USER_PROPERTY_TYPE_STRING,
                FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 {
                    stringvalue: v.as_ptr(),
                },
            ),
        };
        FMOD_STUDIO_USER_PROPERTY {
            name: value.name.as_ptr(),
            type_: kind,
            __bindgen_anon_1: union,
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CommandCaptureFlags: c_uint {
        const NORMAL = FMOD_STUDIO_COMMANDCAPTURE_NORMAL;
        const FILE_FLUSH = FMOD_STUDIO_COMMANDCAPTURE_FILEFLUSH;
        const SKIP_INITIAL_STATE = FMOD_STUDIO_COMMANDCAPTURE_SKIP_INITIAL_STATE;
    }
}

impl From<FMOD_STUDIO_COMMANDCAPTURE_FLAGS> for CommandCaptureFlags {
    fn from(value: FMOD_STUDIO_COMMANDCAPTURE_FLAGS) -> Self {
        CommandCaptureFlags::from_bits_truncate(value)
    }
}

impl From<CommandCaptureFlags> for FMOD_STUDIO_COMMANDCAPTURE_FLAGS {
    fn from(value: CommandCaptureFlags) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CommandReplayFlags: c_uint {
        const NORMAL = FMOD_STUDIO_COMMANDCAPTURE_NORMAL;
        const SKIP_CLEANUP = FMOD_STUDIO_COMMANDREPLAY_SKIP_CLEANUP;
        const FAST_FORWARD = FMOD_STUDIO_COMMANDREPLAY_FAST_FORWARD;
        const SKIP_BANK_LOAD = FMOD_STUDIO_COMMANDREPLAY_SKIP_BANK_LOAD;
    }
}

impl From<FMOD_STUDIO_COMMANDREPLAY_FLAGS> for CommandReplayFlags {
    fn from(value: FMOD_STUDIO_COMMANDREPLAY_FLAGS) -> Self {
        CommandReplayFlags::from_bits_truncate(value)
    }
}

impl From<CommandReplayFlags> for FMOD_STUDIO_COMMANDREPLAY_FLAGS {
    fn from(value: CommandReplayFlags) -> Self {
        value.bits()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BufferInfo {
    pub current_usage: c_int,
    pub peak_usage: c_int,
    pub capacity: c_int,
    pub stall_count: c_int,
    pub stall_time: c_float,
}

impl From<FMOD_STUDIO_BUFFER_INFO> for BufferInfo {
    fn from(value: FMOD_STUDIO_BUFFER_INFO) -> Self {
        BufferInfo {
            current_usage: value.currentusage,
            peak_usage: value.peakusage,
            capacity: value.capacity,
            stall_count: value.stallcount,
            stall_time: value.stalltime,
        }
    }
}

impl From<BufferInfo> for FMOD_STUDIO_BUFFER_INFO {
    fn from(value: BufferInfo) -> Self {
        FMOD_STUDIO_BUFFER_INFO {
            currentusage: value.current_usage,
            peakusage: value.peak_usage,
            capacity: value.capacity,
            stallcount: value.stall_count,
            stalltime: value.stall_time,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BufferUsage {
    pub studio_command_queue: BufferInfo,
    pub studio_handle: BufferInfo,
}

impl From<FMOD_STUDIO_BUFFER_USAGE> for BufferUsage {
    fn from(value: FMOD_STUDIO_BUFFER_USAGE) -> Self {
        BufferUsage {
            studio_command_queue: value.studiocommandqueue.into(),
            studio_handle: value.studiohandle.into(),
        }
    }
}

impl From<BufferUsage> for FMOD_STUDIO_BUFFER_USAGE {
    fn from(value: BufferUsage) -> Self {
        FMOD_STUDIO_BUFFER_USAGE {
            studiocommandqueue: value.studio_command_queue.into(),
            studiohandle: value.studio_handle.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CpuUsage {
    pub update: c_float,
}

impl From<FMOD_STUDIO_CPU_USAGE> for CpuUsage {
    fn from(value: FMOD_STUDIO_CPU_USAGE) -> Self {
        CpuUsage {
            update: value.update,
        }
    }
}

impl From<CpuUsage> for FMOD_STUDIO_CPU_USAGE {
    fn from(value: CpuUsage) -> Self {
        FMOD_STUDIO_CPU_USAGE {
            update: value.update,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SoundInfo {
    pub name_or_data: &'static CStr,
    pub mode: FMOD_MODE, // FIXME ffi types
    pub ex_info: FMOD_CREATESOUNDEXINFO,
    pub subsound_index: c_int,
}

impl SoundInfo {
    /// Create a safe [`SoundInfo`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// All string values from the FFI struct must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`CStr::from_ptr`] for more information.
    unsafe fn from_ffi(value: FMOD_STUDIO_SOUND_INFO) -> Self {
        unsafe {
            SoundInfo {
                name_or_data: CStr::from_ptr(value.name_or_data),
                mode: value.mode,
                ex_info: value.exinfo,
                subsound_index: value.subsoundindex,
            }
        }
    }
}

impl From<SoundInfo> for FMOD_STUDIO_SOUND_INFO {
    fn from(value: SoundInfo) -> Self {
        FMOD_STUDIO_SOUND_INFO {
            name_or_data: value.name_or_data.as_ptr(),
            mode: value.mode,
            exinfo: value.ex_info,
            subsoundindex: value.subsound_index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PlaybackState {
    Playing = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_PLAYING,
    Sustaining = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_SUSTAINING,
    Stopped = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPED,
    Starting = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STARTING,
    Stopping = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPING,
}

impl From<FMOD_STUDIO_PLAYBACK_STATE> for PlaybackState {
    fn from(value: FMOD_STUDIO_PLAYBACK_STATE) -> Self {
        match value {
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_PLAYING => PlaybackState::Playing,
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_SUSTAINING => PlaybackState::Sustaining,
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPED => PlaybackState::Stopped,
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STARTING => PlaybackState::Starting,
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPING => PlaybackState::Stopping,
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<PlaybackState> for FMOD_STUDIO_PLAYBACK_STATE {
    fn from(value: PlaybackState) -> Self {
        value as FMOD_STUDIO_PLAYBACK_STATE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EventProperty {
    ChannelPriority = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY,
    ScheduleDelay = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY,
    ScheduleLookahead = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD,
    MinimumDistance = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE,
    MaximumDistance = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE,
    Cooldown = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN,
}

impl From<FMOD_STUDIO_EVENT_PROPERTY> for EventProperty {
    fn from(value: FMOD_STUDIO_EVENT_PROPERTY) -> Self {
        match value {
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY => {
                EventProperty::ChannelPriority
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY => {
                EventProperty::ScheduleDelay
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD => {
                EventProperty::ScheduleLookahead
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE => {
                EventProperty::MinimumDistance
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE => {
                EventProperty::MaximumDistance
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN => {
                EventProperty::Cooldown
            }
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<EventProperty> for FMOD_STUDIO_EVENT_PROPERTY {
    fn from(value: EventProperty) -> Self {
        value as FMOD_STUDIO_EVENT_PROPERTY
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SystemCallbackMask: c_uint {
        const PREUPDATE = FMOD_STUDIO_SYSTEM_CALLBACK_PREUPDATE;
        const POSTUPDATE = FMOD_STUDIO_SYSTEM_CALLBACK_POSTUPDATE;
        const BANK_UNLOAD = FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD;
        const LIVEUPDATE_CONNECTED = FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_CONNECTED;
        const LIVEUPDATE_DISCONNECTED = FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_DISCONNECTED;
    }
}

impl From<FMOD_STUDIO_SYSTEM_CALLBACK_TYPE> for SystemCallbackMask {
    fn from(value: FMOD_STUDIO_SYSTEM_CALLBACK_TYPE) -> Self {
        SystemCallbackMask::from_bits_truncate(value)
    }
}

impl From<SystemCallbackMask> for FMOD_STUDIO_SYSTEM_CALLBACK_TYPE {
    fn from(value: SystemCallbackMask) -> Self {
        value.bits()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemCallbackKind<U: UserdataTypes> {
    Preupdate,
    Postupdate,
    BankUnload(Bank<U>),
    LiveupdateConnected,
    LiveupdateDisconnected,
}

#[derive(Debug, Clone, Copy)]
pub struct CommandInfo {
    pub command_name: &'static CStr, // FIXME: WRONG
    pub parent_command_index: c_int,
    pub frame_number: c_int,
    pub frame_time: c_float,
    pub instance_type: InstanceType,
    pub output_type: InstanceType,
    pub instance_handle: c_uint,
    pub output_handle: c_uint,
}

impl From<CommandInfo> for FMOD_STUDIO_COMMAND_INFO {
    fn from(value: CommandInfo) -> Self {
        FMOD_STUDIO_COMMAND_INFO {
            commandname: value.command_name.as_ptr(),
            parentcommandindex: value.parent_command_index,
            framenumber: value.frame_number,
            frametime: value.frame_time,
            instancetype: value.instance_type.into(),
            outputtype: value.output_type.into(),
            instancehandle: value.instance_handle,
            outputhandle: value.output_handle,
        }
    }
}

impl CommandInfo {
    /// Create a safe [`CommandInfo`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// All string values from the FFI struct must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`CStr::from_ptr`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_COMMAND_INFO) -> Self {
        CommandInfo {
            command_name: unsafe { CStr::from_ptr(value.commandname) },
            parent_command_index: value.parentcommandindex,
            frame_number: value.framenumber,
            frame_time: value.frametime,
            instance_type: value.instancetype.into(),
            output_type: value.outputtype.into(),
            instance_handle: value.instancehandle,
            output_handle: value.outputhandle,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum InstanceType {
    None = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_NONE,
    System = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_SYSTEM,
    EventDescription = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION,
    EventInstance = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE,
    ParameterInstance = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE,
    Bus = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BUS,
    Vca = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_VCA,
    Bank = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BANK,
    CommandReplay = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY,
}

impl From<FMOD_STUDIO_INSTANCETYPE> for InstanceType {
    fn from(value: FMOD_STUDIO_INSTANCETYPE) -> Self {
        match value {
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_NONE => InstanceType::None,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_SYSTEM => InstanceType::System,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION => {
                InstanceType::EventDescription
            }
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE => {
                InstanceType::EventInstance
            }
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE => {
                InstanceType::ParameterInstance
            }
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BUS => InstanceType::Bus,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_VCA => InstanceType::Vca,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BANK => InstanceType::Bank,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY => {
                InstanceType::CommandReplay
            }
            _ => panic!("invalid instance type"),
        }
    }
}

impl From<InstanceType> for FMOD_STUDIO_INSTANCETYPE {
    fn from(value: InstanceType) -> Self {
        match value {
            InstanceType::None => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_NONE,
            InstanceType::System => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_SYSTEM,
            InstanceType::EventDescription => {
                FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION
            }
            InstanceType::EventInstance => {
                FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE
            }
            InstanceType::ParameterInstance => {
                FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE
            }
            InstanceType::Bus => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BUS,
            InstanceType::Vca => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_VCA,
            InstanceType::Bank => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BANK,
            InstanceType::CommandReplay => {
                FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY
            }
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EventCallbackMask: c_uint {
        const CREATED = FMOD_STUDIO_EVENT_CALLBACK_CREATED;
        const DESTROYED = FMOD_STUDIO_EVENT_CALLBACK_DESTROYED;
        const STARTING = FMOD_STUDIO_EVENT_CALLBACK_STARTING;
        const STARTED = FMOD_STUDIO_EVENT_CALLBACK_STARTED;
        const RESTARTED = FMOD_STUDIO_EVENT_CALLBACK_RESTARTED;
        const STOPPED = FMOD_STUDIO_EVENT_CALLBACK_STOPPED;
        const START_FAILED = FMOD_STUDIO_EVENT_CALLBACK_START_FAILED;
        const CREATE_PROGRAMMER_SOUND = FMOD_STUDIO_EVENT_CALLBACK_CREATE_PROGRAMMER_SOUND;
        const DESTROY_PROGRAMMER_SOUND = FMOD_STUDIO_EVENT_CALLBACK_DESTROY_PROGRAMMER_SOUND;
        const PLUGIN_CREATED = FMOD_STUDIO_EVENT_CALLBACK_PLUGIN_CREATED;
        const PLUGIN_DESTROYED = FMOD_STUDIO_EVENT_CALLBACK_PLUGIN_DESTROYED;
        const TIMELINE_MARKER = FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_MARKER;
        const TIMELINE_BEAT = FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_BEAT;
        const SOUND_PLAYED = FMOD_STUDIO_EVENT_CALLBACK_SOUND_PLAYED;
        const SOUND_STOPPED = FMOD_STUDIO_EVENT_CALLBACK_SOUND_STOPPED;
        const REAL_TO_VIRTUAL = FMOD_STUDIO_EVENT_CALLBACK_REAL_TO_VIRTUAL;
        const VIRTUAL_TO_REAL = FMOD_STUDIO_EVENT_CALLBACK_VIRTUAL_TO_REAL;
        const START_EVENT_COMMAND = FMOD_STUDIO_EVENT_CALLBACK_START_EVENT_COMMAND;
        const NESTED_TIMELINE_BEAT = FMOD_STUDIO_EVENT_CALLBACK_NESTED_TIMELINE_BEAT;
        const ALL = FMOD_STUDIO_EVENT_CALLBACK_ALL;
    }
}

impl From<FMOD_STUDIO_EVENT_CALLBACK_TYPE> for EventCallbackMask {
    fn from(value: FMOD_STUDIO_EVENT_CALLBACK_TYPE) -> Self {
        EventCallbackMask::from_bits_truncate(value)
    }
}

impl From<EventCallbackMask> for FMOD_STUDIO_EVENT_CALLBACK_TYPE {
    fn from(value: EventCallbackMask) -> Self {
        value.bits()
    }
}

pub enum EventCallbackKind<U: UserdataTypes> {
    Created,
    Destroyed,
    Starting,
    Started,
    Restarted,
    Stopped,
    StartFailed,
    CreateProgrammerSound(ProgrammerSoundProperties),
    DestroyProgrammerSound(ProgrammerSoundProperties),
    PluginCreated(PluginInstanceProperties),
    PluginDestroyed(PluginInstanceProperties),
    TimelineMarker(TimelineMarkerProperties),
    TimelineBeat(TimelineBeatProperties),
    SoundPlayed(Sound),
    SoundStopped(Sound),
    RealToVirtual,
    VirtualToReal,
    StartEventCommand(EventInstance<U>),
    NestedTimelineBeat(TimelineNestedBeatProperties),
}

pub struct ProgrammerSoundProperties {
    // FIXME lifetimes & mutability
    // FIXME enforce that writes MUST happen to this somehow also use option too
    pub name: &'static CStr,
    pub sound: &'static mut Sound,
    pub subsound_index: &'static mut c_int,
}

pub struct PluginInstanceProperties {
    // FIXME lifetimes
    pub name: &'static CStr,
    pub dsp: Dsp,
}

impl From<PluginInstanceProperties> for FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES {
    fn from(value: PluginInstanceProperties) -> Self {
        FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES {
            name: value.name.as_ptr(),
            dsp: value.dsp.inner,
        }
    }
}

impl PluginInstanceProperties {
    /// Create a safe [`PluginInstanceProperties`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// All string values from the FFI struct must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`CStr::from_ptr`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES) -> Self {
        PluginInstanceProperties {
            name: unsafe { CStr::from_ptr(value.name) },
            dsp: value.dsp.into(),
        }
    }
}

pub struct TimelineMarkerProperties {
    // FIXME lifetimes
    pub name: &'static CStr,
    pub position: c_int,
}

impl From<TimelineMarkerProperties> for FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES {
    fn from(value: TimelineMarkerProperties) -> Self {
        FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES {
            name: value.name.as_ptr(),
            position: value.position,
        }
    }
}

impl TimelineMarkerProperties {
    /// Create a safe [`TimelineMarkerProperties`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// All string values from the FFI struct must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`CStr::from_ptr`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES) -> Self {
        TimelineMarkerProperties {
            name: unsafe { CStr::from_ptr(value.name) },
            position: value.position,
        }
    }
}

pub struct TimelineBeatProperties {
    pub bar: c_int,
    pub beat: c_int,
    pub position: c_int,
    pub tempo: c_float,
    pub time_signature_upper: c_int,
    pub time_signature_lower: c_int,
}

impl From<TimelineBeatProperties> for FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES {
    fn from(value: TimelineBeatProperties) -> Self {
        FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES {
            bar: value.bar,
            beat: value.beat,
            position: value.position,
            tempo: value.tempo,
            timesignatureupper: value.time_signature_upper,
            timesignaturelower: value.time_signature_lower,
        }
    }
}

impl From<FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES> for TimelineBeatProperties {
    fn from(value: FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES) -> Self {
        TimelineBeatProperties {
            bar: value.bar,
            beat: value.beat,
            position: value.position,
            tempo: value.tempo,
            time_signature_upper: value.timesignatureupper,
            time_signature_lower: value.timesignaturelower,
        }
    }
}

pub struct TimelineNestedBeatProperties {
    pub event_guid: Guid,
    pub properties: TimelineBeatProperties,
}

impl From<TimelineNestedBeatProperties> for FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES {
    fn from(value: TimelineNestedBeatProperties) -> Self {
        FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES {
            eventid: value.event_guid.into(),
            properties: value.properties.into(),
        }
    }
}

impl From<FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES> for TimelineNestedBeatProperties {
    fn from(value: FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES) -> Self {
        TimelineNestedBeatProperties {
            event_guid: value.eventid.into(),
            properties: value.properties.into(),
        }
    }
}
