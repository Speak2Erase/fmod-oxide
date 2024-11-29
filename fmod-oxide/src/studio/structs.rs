// Copyright (c) 2024 Melody Madeline Lyons
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
    Guid, SoundBuilder,
};

/// Memory usage statistics.
///
/// Memory usage `exclusive` and `inclusive` values do not include sample data loaded in memory because sample data is a shared resource.
/// Streaming sample data is not a shared resource and is included in the exclusive and `inclusive` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryUsage {
    /// Size of memory belonging to the bus or event instance.
    pub exclusive: c_int,
    /// Size of memory belonging exclusively to the bus or event plus the inclusive memory sizes of all buses and event instances which route into it.
    pub inclusive: c_int,
    /// Size of shared sample memory referenced by the bus or event instance,
    /// inclusive of all sample memory referenced by all buses and event instances which route into it.
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

/// Describes an event parameter identifier.
///
/// `ParameterID` can be retrieved from the `ParameterDescription`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// forces this type to have the exact same layout as FMOD_STUDIO_PARAMETER_ID so we can safely transmute between them.
#[repr(C)]
pub struct ParameterID {
    /// First half of the ID.
    pub data_1: c_uint,
    /// Second half of the ID.
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

/// Settings for advanced features like configuring memory and cpu usage.
// default impl is ok, all values are zero or none.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct AdvancedSettings {
    /// Command queue size for studio async processing.
    pub command_queue_size: c_uint,
    /// Initial size to allocate for handles. Memory for handles will grow as needed in pages.
    pub handle_initial_size: c_uint,
    /// Update period of Studio when in async mode, in milliseconds. Will be quantized to the nearest multiple of mixer duration.
    pub studio_update_period: c_int,
    /// Size in bytes of sample data to retain in memory when no longer used, to avoid repeated disk I/O. Use -1 to disable.
    pub idle_sample_data_pool_size: c_int,
    /// Specify the schedule delay for streams, in samples.
    /// Lower values can reduce latency when scheduling events containing streams but may cause scheduling issues if too small.
    pub streaming_schedule_delay: c_uint,
    /// Specify the key for loading sounds from encrypted banks.
    pub encryption_key: Option<Utf8CString>, // TODO investigate if FMOD copies this string or if it needs to be kept alive
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
            studio_update_period: value.studioupdateperiod,
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
            studioupdateperiod: value.studio_update_period,
            idlesampledatapoolsize: value.idle_sample_data_pool_size,
            streamingscheduledelay: value.streaming_schedule_delay,
            encryptionkey: encryption_key,
        }
    }
}

/// Describes an event parameter.
#[derive(Clone, PartialEq, Debug)]
pub struct ParameterDescription {
    /// The parameter's name.
    pub name: Utf8CString,
    /// The parameter's id.
    pub id: ParameterID,
    /// The parameter's minimum value.
    pub minimum: c_float,
    /// The parameter's maximum value.
    pub maximum: c_float,
    /// The parameter's default value.
    pub default_value: c_float,
    /// The parameter's type.
    pub kind: ParameterKind,
    /// The parameter's behavior flags.
    pub flags: ParameterFlags,
    /// The parameter's `Guid`.
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

/// Describes a user property.
#[derive(Clone, PartialEq, Debug)]
pub struct UserProperty {
    /// Property name.
    pub name: Utf8CString,
    /// Property type.
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
    ///
    /// # Panics
    ///
    /// This function will panic if `value` is not a valid user property type.
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

/// Information for a single buffer in FMOD Studio.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BufferInfo {
    /// Current buffer usage in bytes.
    pub current_usage: c_int,
    /// Peak buffer usage in bytes.
    pub peak_usage: c_int,
    /// Buffer capacity in bytes.
    pub capacity: c_int,
    /// Cumulative number of stalls due to buffer overflow.
    pub stall_count: c_int,
    /// Cumulative amount of time stalled due to buffer overflow, in seconds.
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

/// Information for FMOD Studio buffer usage.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BufferUsage {
    /// Information for the Studio Async Command buffer.
    pub studio_command_queue: BufferInfo,
    /// Information for the Studio handle table.
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

/// Performance information for Studio API functionality.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CpuUsage {
    /// `System::update` CPU usage.
    /// Percentage of main thread, or main thread if the System was created with `SYNCHRONOUS_UPDATE`.
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

/// Describes a sound in the audio table.
#[derive(Debug)]
pub struct SoundInfo<'a> {
    /// The Sound's sound builder.
    pub builder: SoundBuilder<'a>,
    /// Subsound index for loading the sound.
    pub subsound_index: c_int,
}

impl<'a> SoundInfo<'a> {
    /// Create a safe [`SoundInfo`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// See [`SoundBuilder::from_ffi`] for more information.
    pub unsafe fn from_ffi(value: FMOD_STUDIO_SOUND_INFO) -> Self {
        SoundInfo {
            builder: unsafe {
                SoundBuilder::from_ffi(value.name_or_data, value.mode, value.exinfo)
            },
            subsound_index: value.subsoundindex,
        }
    }
}

impl From<&SoundInfo<'_>> for FMOD_STUDIO_SOUND_INFO {
    fn from(value: &SoundInfo<'_>) -> Self {
        FMOD_STUDIO_SOUND_INFO {
            name_or_data: value.builder.name_or_data,
            mode: value.builder.mode,
            exinfo: value.builder.create_sound_ex_info,
            subsoundindex: value.subsound_index,
        }
    }
}

/// Describes a command replay command.
#[derive(Debug, Clone)]
pub struct CommandInfo {
    /// Fully qualified C++ name of the API function for this command.
    pub command_name: Utf8CString,
    /// Index of the command that created the instance this command operates on, or -1 if the command does not operate on any instance.
    pub parent_command_index: c_int,
    /// Frame the command belongs to.
    pub frame_number: c_int,
    /// Playback time at which this command will be executed.
    pub frame_time: c_float,
    /// Type of object that this command uses as an instance.
    pub instance_type: InstanceType,
    /// Type of object that this command outputs.
    pub output_type: InstanceType,
    /// Original handle value of the instance.
    pub instance_handle: c_uint,
    /// Original handle value of the command output.
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

/// Describes a programmer sound.
#[derive(Debug)]
pub struct ProgrammerSoundProperties<'prop> {
    /// Name of the programmer instrument (set in FMOD Studio).
    pub name: Utf8CString,
    /// Programmer-created sound.
    // FIXME use option for both of these
    pub sound: &'prop mut Sound,
    /// Subsound index.
    pub subsound_index: &'prop mut c_int,
}

/// Describes a DSP plug-in instance.
#[derive(Debug)]
pub struct PluginInstanceProperties {
    /// Name of the plug-in effect or sound (set in FMOD Studio).
    pub name: Utf8CString,
    /// DSP plug-in instance. (DSP)
    pub dsp: Dsp,
}

impl From<&PluginInstanceProperties> for FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES {
    fn from(value: &PluginInstanceProperties) -> Self {
        FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES {
            name: value.name.as_ptr(),
            dsp: value.dsp.into(),
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

/// Describes a marker on the timeline.
#[derive(Debug, Clone)]
pub struct TimelineMarkerProperties {
    /// Marker name.
    pub name: Utf8CString,
    /// Position of the marker on the timeline in milliseconds.
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

/// Describes a beat on the timeline.
#[derive(Clone, Copy, Debug)]
pub struct TimelineBeatProperties {
    /// Bar number (starting from 1).
    pub bar: c_int,
    /// Beat number within bar (starting from 1).
    pub beat: c_int,
    /// Position of the beat on the timeline in milliseconds.
    pub position: c_int,
    /// Current tempo in beats per minute.
    pub tempo: c_float,
    /// Current time signature upper number (beats per bar).
    pub time_signature_upper: c_int,
    /// Current time signature lower number (beat unit).
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

/// Describes a beat on the timeline from a nested event.
#[derive(Debug, Clone, Copy)]
pub struct TimelineNestedBeatProperties {
    /// Event description GUID.
    pub event_guid: Guid,
    /// Beat properties.
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
