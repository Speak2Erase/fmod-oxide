// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use num_enum::UnsafeFromPrimitive;
use std::ffi::{c_float, c_int, c_uint};

use super::{InstanceType, ParameterFlags, ParameterKind, UserPropertyKind};
use crate::{
    core::{Dsp, Sound},
    Guid, Mode,
};

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

// default impl is ok, all values are zero or none.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct AdvancedSettings {
    pub command_queue_size: c_uint,
    pub handle_initial_size: c_uint,
    pub studioupdateperiod: c_int,
    pub idle_sample_data_pool_size: c_int,
    pub streaming_schedule_delay: c_uint,
    pub encryption_key: Option<Utf8CString>,
}

impl AdvancedSettings {
    /// Create a safe [`AdvancedSettings`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// The encryption key from [`FMOD_STUDIO_ADVANCEDSETTINGS`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_ADVANCEDSETTINGS) -> Self {
        let encryption_key = if value.encryptionkey.is_null() {
            None
        } else {
            let cstring = unsafe { Utf8CStr::from_ptr_unchecked(value.encryptionkey) }.to_cstring();
            Some(cstring)
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

// It's safe to go from AdvancedSettings to FMOD_STUDIO_ADVANCEDSETTINGS because a &Utf8CStr meets all the safety FMOD expects. (aligned, null termienated, etc)
impl From<&AdvancedSettings> for FMOD_STUDIO_ADVANCEDSETTINGS {
    fn from(value: &AdvancedSettings) -> Self {
        let encryption_key = value
            .encryption_key
            .as_deref()
            .map_or(std::ptr::null(), Utf8CStr::as_ptr);

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

#[derive(Clone, PartialEq, Debug)]
pub struct ParameterDescription {
    pub name: Utf8CString,
    pub id: ParameterID,
    pub minimum: c_float,
    pub maximum: c_float,
    pub default_value: c_float,
    pub kind: ParameterKind,
    pub flags: ParameterFlags,
    pub guid: Guid,
}

// It's safe to go from ParameterDescription to FMOD_STUDIO_PARAMETER_DESCRIPTION because a &Utf8CString meets all the safety FMOD expects. (aligned, null terminated, etc)
impl From<&ParameterDescription> for FMOD_STUDIO_PARAMETER_DESCRIPTION {
    fn from(value: &ParameterDescription) -> Self {
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
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_PARAMETER_DESCRIPTION) -> ParameterDescription {
        unsafe {
            ParameterDescription {
                name: Utf8CStr::from_ptr_unchecked(value.name).to_cstring(),
                id: value.id.into(),
                minimum: value.minimum,
                maximum: value.maximum,
                default_value: value.defaultvalue,
                kind: ParameterKind::unchecked_transmute_from(value.type_),
                flags: value.flags.into(),
                guid: value.guid.into(),
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UserProperty {
    pub name: Utf8CString,
    pub kind: UserPropertyKind,
}

impl UserProperty {
    /// Create a safe [`UserPropertyKind`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// All string values from the FFI struct must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    /// The type field must match the type assigned to the union.
    ///
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_USER_PROPERTY) -> Self {
        unsafe {
            UserProperty {
                name: Utf8CStr::from_ptr_unchecked(value.name).to_cstring(),
                kind: match value.type_ {
                    FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER => {
                        UserPropertyKind::Int(value.__bindgen_anon_1.intvalue)
                    }
                    FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN => {
                        UserPropertyKind::Bool(value.__bindgen_anon_1.boolvalue.into())
                    }
                    FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT => {
                        UserPropertyKind::Float(value.__bindgen_anon_1.floatvalue)
                    }
                    FMOD_STUDIO_USER_PROPERTY_TYPE_STRING => {
                        let cstring =
                            Utf8CStr::from_ptr_unchecked(value.__bindgen_anon_1.stringvalue)
                                .to_cstring();
                        UserPropertyKind::String(cstring)
                    }
                    v => panic!("invalid user property type {v}"),
                },
            }
        }
    }
}

impl From<&UserProperty> for FMOD_STUDIO_USER_PROPERTY {
    fn from(value: &UserProperty) -> Self {
        let (kind, union) = match value.kind {
            UserPropertyKind::Int(v) => (
                FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER,
                FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 { intvalue: v },
            ),
            UserPropertyKind::Bool(v) => (
                FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN,
                FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 {
                    boolvalue: v.into(),
                },
            ),
            UserPropertyKind::Float(v) => (
                FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT,
                FMOD_STUDIO_USER_PROPERTY__bindgen_ty_1 { floatvalue: v },
            ),
            UserPropertyKind::String(ref v) => (
                FMOD_STUDIO_USER_PROPERTY_TYPE_STRING,
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

#[derive(Debug)]
pub struct SoundInfo {
    pub name_or_data: Utf8CString,
    pub mode: Mode, // FIXME ffi types
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
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_SOUND_INFO) -> Self {
        unsafe {
            SoundInfo {
                name_or_data: Utf8CStr::from_ptr_unchecked(value.name_or_data).to_cstring(),
                mode: value.mode.into(),
                ex_info: value.exinfo,
                subsound_index: value.subsoundindex,
            }
        }
    }
}

impl From<&SoundInfo> for FMOD_STUDIO_SOUND_INFO {
    fn from(value: &SoundInfo) -> Self {
        FMOD_STUDIO_SOUND_INFO {
            name_or_data: value.name_or_data.as_ptr(),
            mode: value.mode.into(),
            exinfo: value.ex_info,
            subsoundindex: value.subsound_index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub command_name: Utf8CString,
    pub parent_command_index: c_int,
    pub frame_number: c_int,
    pub frame_time: c_float,
    pub instance_type: InstanceType,
    pub output_type: InstanceType,
    pub instance_handle: c_uint,
    pub output_handle: c_uint,
}

impl From<&CommandInfo> for FMOD_STUDIO_COMMAND_INFO {
    fn from(value: &CommandInfo) -> Self {
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
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_COMMAND_INFO) -> Self {
        CommandInfo {
            command_name: unsafe { Utf8CStr::from_ptr_unchecked(value.commandname).to_cstring() },
            parent_command_index: value.parentcommandindex,
            frame_number: value.framenumber,
            frame_time: value.frametime,
            instance_type: unsafe { InstanceType::unchecked_transmute_from(value.instancetype) },
            output_type: unsafe { InstanceType::unchecked_transmute_from(value.instancetype) },
            instance_handle: value.instancehandle,
            output_handle: value.outputhandle,
        }
    }
}

pub struct ProgrammerSoundProperties<'prop> {
    // FIXME use option
    pub name: Utf8CString,
    pub sound: &'prop mut Sound,
    pub subsound_index: &'prop mut c_int,
}

pub struct PluginInstanceProperties {
    pub name: Utf8CString,
    pub dsp: Dsp,
}

impl From<&PluginInstanceProperties> for FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES {
    fn from(value: &PluginInstanceProperties) -> Self {
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
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES) -> Self {
        PluginInstanceProperties {
            name: unsafe { Utf8CStr::from_ptr_unchecked(value.name) }.to_cstring(),
            dsp: value.dsp.into(),
        }
    }
}

pub struct TimelineMarkerProperties {
    pub name: Utf8CString,
    pub position: c_int,
}

impl From<&TimelineMarkerProperties> for FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES {
    fn from(value: &TimelineMarkerProperties) -> Self {
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
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES) -> Self {
        TimelineMarkerProperties {
            name: unsafe { Utf8CStr::from_ptr_unchecked(value.name) }.to_cstring(),
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
