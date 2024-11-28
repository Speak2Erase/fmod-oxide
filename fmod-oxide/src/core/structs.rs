// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::{
    ffi::{c_char, c_float, c_int, c_short, c_uchar, c_uint, c_ushort},
    marker::PhantomData,
    mem::MaybeUninit,
};

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};

use crate::{
    string_from_utf16_be, string_from_utf16_le, ChannelOrder, DspParameterDataType, Mode,
    SoundFormat, SoundGroup, SoundType, TagType, TimeUnit,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
// force this type to have the exact same layout as FMOD_STUDIO_PARAMETER_ID so we can safely transmute between them.
#[repr(C)]
pub struct Guid {
    pub data_1: c_uint,
    pub data_2: c_ushort,
    pub data_3: c_ushort,
    pub data_4: [c_uchar; 8],
}

impl Guid {
    pub fn parse(string: &Utf8CStr) -> Result<Self> {
        let mut guid = MaybeUninit::uninit();
        unsafe {
            FMOD_Studio_ParseID(string.as_ptr(), guid.as_mut_ptr()).to_result()?;
            Ok(guid.assume_init().into())
        }
    }
}

impl From<FMOD_GUID> for Guid {
    fn from(value: FMOD_GUID) -> Self {
        Guid {
            data_1: value.Data1,
            data_2: value.Data2,
            data_3: value.Data3,
            data_4: value.Data4,
        }
    }
}

impl From<Guid> for FMOD_GUID {
    fn from(value: Guid) -> Self {
        FMOD_GUID {
            Data1: value.data_1,
            Data2: value.data_2,
            Data3: value.data_3,
            Data4: value.data_4,
        }
    }
}

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Guid {
            data_1,
            data_2,
            data_3,
            data_4,
        } = self;

        f.write_str("{")?;
        f.write_fmt(format_args!("{data_1:0>8x}-{data_2:0>4x}-{data_3:0>4x}-"))?;
        f.write_fmt(format_args!("{:0>2x}{:0>2x}-", data_4[0], data_4[1]))?;
        for b in &data_4[2..] {
            f.write_fmt(format_args!("{b:0>2x}"))?;
        }
        f.write_str("}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vector {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

impl From<Vector> for FMOD_VECTOR {
    fn from(value: Vector) -> Self {
        FMOD_VECTOR {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<FMOD_VECTOR> for Vector {
    fn from(value: FMOD_VECTOR) -> Self {
        Vector {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Attributes3D {
    pub position: Vector,
    pub velocity: Vector,
    pub forward: Vector,
    pub up: Vector,
}

impl From<FMOD_3D_ATTRIBUTES> for Attributes3D {
    fn from(value: FMOD_3D_ATTRIBUTES) -> Self {
        Attributes3D {
            position: value.position.into(),
            velocity: value.velocity.into(),
            forward: value.forward.into(),
            up: value.up.into(),
        }
    }
}

impl From<Attributes3D> for FMOD_3D_ATTRIBUTES {
    fn from(value: Attributes3D) -> Self {
        FMOD_3D_ATTRIBUTES {
            position: value.position.into(),
            velocity: value.velocity.into(),
            forward: value.forward.into(),
            up: value.up.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct CpuUsage {
    pub dsp: c_float,
    pub stream: c_float,
    pub geometry: c_float,
    pub update: c_float,
    pub convolution_1: c_float,
    pub convolution_2: c_float,
}

impl From<FMOD_CPU_USAGE> for CpuUsage {
    fn from(value: FMOD_CPU_USAGE) -> Self {
        CpuUsage {
            dsp: value.dsp,
            stream: value.stream,
            geometry: value.geometry,
            update: value.update,
            convolution_1: value.convolution1,
            convolution_2: value.convolution2,
        }
    }
}

impl From<CpuUsage> for FMOD_CPU_USAGE {
    fn from(value: CpuUsage) -> Self {
        FMOD_CPU_USAGE {
            dsp: value.dsp,
            stream: value.stream,
            geometry: value.geometry,
            update: value.update,
            convolution1: value.convolution_1,
            convolution2: value.convolution_2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct ReverbProperties {
    pub decay_time: c_float,
    pub early_delay: c_float,
    pub late_delay: c_float,
    pub hf_reference: c_float,
    pub hf_decay_ratio: c_float,
    pub diffusion: c_float,
    pub density: c_float,
    pub low_shelf_frequency: c_float,
    pub low_shelf_gain: c_float,
    pub high_cut: c_float,
    pub early_late_mix: c_float,
    pub wet_level: c_float,
}

impl From<FMOD_REVERB_PROPERTIES> for ReverbProperties {
    fn from(value: FMOD_REVERB_PROPERTIES) -> Self {
        ReverbProperties {
            decay_time: value.DecayTime,
            early_delay: value.EarlyDelay,
            late_delay: value.LateDelay,
            hf_reference: value.HFReference,
            hf_decay_ratio: value.HFDecayRatio,
            diffusion: value.Diffusion,
            density: value.Density,
            low_shelf_frequency: value.LowShelfFrequency,
            low_shelf_gain: value.LowShelfGain,
            high_cut: value.HighCut,
            early_late_mix: value.EarlyLateMix,
            wet_level: value.WetLevel,
        }
    }
}

impl From<ReverbProperties> for FMOD_REVERB_PROPERTIES {
    fn from(value: ReverbProperties) -> Self {
        FMOD_REVERB_PROPERTIES {
            DecayTime: value.decay_time,
            EarlyDelay: value.early_delay,
            LateDelay: value.late_delay,
            HFReference: value.hf_reference,
            HFDecayRatio: value.hf_decay_ratio,
            Diffusion: value.diffusion,
            Density: value.density,
            LowShelfFrequency: value.low_shelf_frequency,
            LowShelfGain: value.low_shelf_gain,
            HighCut: value.high_cut,
            EarlyLateMix: value.early_late_mix,
            WetLevel: value.wet_level,
        }
    }
}

pub struct DspParameterDescription {
    pub kind: DspParameterType,
    pub name: Utf8CString,
    pub label: Utf8CString,
    pub description: Utf8CString,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DspParameterType {
    Float {
        min: f32,
        max: f32,
        default: f32,
        mapping: FloatMapping,
    },
    Int {
        min: i32,
        max: i32,
        default: i32,
        goes_to_infinity: bool,
        // TODO names
    },
    Bool {
        default: bool,
        // TODO names
    },
    Data {
        data_type: DspParameterDataType,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FloatMapping {
    // TODO
}

impl DspParameterDescription {
    /// Create a safe [`DspParameterDescription`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// [`FMOD_DSP_PARAMETER_DESC::type_`] must match the union value.
    ///
    /// The strings [`FMOD_DSP_PARAMETER_DESC`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_DSP_PARAMETER_DESC) -> Self {
        // FIXME these array accesses are safe and could be done in a safer way
        let name = unsafe { Utf8CStr::from_ptr_unchecked(value.name.as_ptr()).to_cstring() };
        let label = unsafe { Utf8CStr::from_ptr_unchecked(value.label.as_ptr()).to_cstring() };
        let description = unsafe { Utf8CStr::from_ptr_unchecked(value.description).to_cstring() };
        let kind = match value.type_ {
            FMOD_DSP_PARAMETER_TYPE_FLOAT => {
                let floatdesc = unsafe { value.__bindgen_anon_1.floatdesc };
                DspParameterType::Float {
                    min: floatdesc.min,
                    max: floatdesc.max,
                    default: floatdesc.defaultval,
                    mapping: FloatMapping {},
                }
            }
            FMOD_DSP_PARAMETER_TYPE_INT => {
                let intdesc = unsafe { value.__bindgen_anon_1.intdesc };
                DspParameterType::Int {
                    min: intdesc.min,
                    max: intdesc.max,
                    default: intdesc.defaultval,
                    goes_to_infinity: intdesc.goestoinf.into(),
                }
            }
            FMOD_DSP_PARAMETER_TYPE_BOOL => {
                let booldesc = unsafe { value.__bindgen_anon_1.booldesc };
                DspParameterType::Bool {
                    default: booldesc.defaultval.into(),
                }
            }
            FMOD_DSP_PARAMETER_TYPE_DATA => {
                let datadesc = unsafe { value.__bindgen_anon_1.datadesc };
                DspParameterType::Data {
                    data_type: datadesc.datatype.try_into().unwrap(),
                }
            }
            _ => panic!("invalid parameter description type"), // FIXME panic
        };
        Self {
            kind,
            name,
            label,
            description,
        }
    }

    // TODO ffi conversion (altho is it even needed?)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DspMeteringInfo {
    pub sample_count: c_int,
    pub peak_level: [c_float; 32],
    pub rms_level: [c_float; 32],
    pub channel_count: c_short,
}

impl From<FMOD_DSP_METERING_INFO> for DspMeteringInfo {
    fn from(value: FMOD_DSP_METERING_INFO) -> Self {
        Self {
            sample_count: value.numsamples,
            peak_level: value.peaklevel,
            rms_level: value.rmslevel,
            channel_count: value.numchannels,
        }
    }
}

impl From<DspMeteringInfo> for FMOD_DSP_METERING_INFO {
    fn from(value: DspMeteringInfo) -> Self {
        FMOD_DSP_METERING_INFO {
            numsamples: value.sample_count,
            peaklevel: value.peak_level,
            rmslevel: value.rms_level,
            numchannels: value.channel_count,
        }
    }
}

pub struct Tag {
    pub kind: TagType,
    pub name: Utf8CString,
    pub data: TagData,
    pub updated: bool,
}

// FIXME: these strings are most likely null-terminated
pub enum TagData {
    Binary(Vec<u8>),
    Integer(i64),
    Float(f64),
    String(String),
    Utf8String(String),
    Utf16StringBE(String),
    Utf16String(String),
}

impl Tag {
    /// Create a safe [`Tag`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// The string [`FMOD_TAG::name`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// This function will read into arbitrary memory! Because of this the tag data type must match the data type of the data pointer.
    #[allow(clippy::cast_lossless)]
    pub unsafe fn from_ffi(value: FMOD_TAG) -> Self {
        let kind = value.type_.try_into().unwrap();
        let name = unsafe { Utf8CStr::from_ptr_unchecked(value.name).to_cstring() };
        let updated = value.updated.into();
        let data = unsafe {
            // awful union-esquqe code
            match value.datatype {
                FMOD_TAGDATATYPE_BINARY => {
                    let slice =
                        std::slice::from_raw_parts(value.data as *const u8, value.datalen as usize);
                    TagData::Binary(slice.to_vec())
                }
                FMOD_TAGDATATYPE_INT => match value.datalen {
                    1 => TagData::Integer(*value.data.cast::<i8>() as i64),
                    2 => TagData::Integer(*value.data.cast::<i16>() as i64),
                    4 => TagData::Integer(*value.data.cast::<i32>() as i64),
                    8 => TagData::Integer(*value.data.cast::<i64>()),
                    _ => panic!("unrecognized integer data len"),
                },
                FMOD_TAGDATATYPE_FLOAT => match value.datalen {
                    4 => TagData::Float(*value.data.cast::<f32>() as f64),
                    8 => TagData::Float(*value.data.cast::<f64>()),
                    _ => panic!("unrecognized float data len"),
                },
                FMOD_TAGDATATYPE_STRING => {
                    let ascii =
                        std::slice::from_raw_parts(value.data.cast(), value.datalen as usize);
                    let string = String::from_utf8_lossy(ascii).into_owned();
                    TagData::String(string)
                }
                FMOD_TAGDATATYPE_STRING_UTF8 => {
                    let utf8 =
                        std::slice::from_raw_parts(value.data.cast(), value.datalen as usize);
                    let string = String::from_utf8_lossy(utf8).into_owned();
                    TagData::Utf8String(string)
                }
                // depending on the architecture rust will optimize this to a no-op
                // we still need to do this to ensure the correct endianness
                // ideally we could use String::from_utf16_be_lossy but that is nightly only and the tracking issue has basically no activity
                FMOD_TAGDATATYPE_STRING_UTF16 => {
                    let slice =
                        std::slice::from_raw_parts(value.data.cast(), value.datalen as usize);
                    let string = string_from_utf16_le(slice);
                    TagData::Utf16String(string)
                }
                FMOD_TAGDATATYPE_STRING_UTF16BE => {
                    let slice =
                        std::slice::from_raw_parts(value.data.cast(), value.datalen as usize);
                    let string = string_from_utf16_be(slice);
                    TagData::Utf16StringBE(string)
                }
                _ => panic!("unrecognized tag data type"), // FIXME panic
            }
        };
        Tag {
            kind,
            name,
            data,
            updated,
        }
    }
}

#[derive(Debug)]
pub struct SoundBuilder<'a> {
    pub(crate) mode: FMOD_MODE,
    pub(crate) create_sound_ex_info: FMOD_CREATESOUNDEXINFO,
    pub(crate) name_or_data: *const c_char,
    pub(crate) _phantom: PhantomData<&'a ()>,
}

const EMPTY_EXINFO: FMOD_CREATESOUNDEXINFO = unsafe {
    FMOD_CREATESOUNDEXINFO {
        cbsize: std::mem::size_of::<FMOD_CREATESOUNDEXINFO>() as c_int,
        ..std::mem::MaybeUninit::zeroed().assume_init()
    }
};

// setters
impl<'a> SoundBuilder<'a> {
    pub const fn open(filename: &'a Utf8CStr) -> Self {
        Self {
            mode: 0,
            create_sound_ex_info: EMPTY_EXINFO,
            name_or_data: filename.as_ptr(),
            _phantom: PhantomData,
        }
    }

    // TODO open_user

    /// # Safety
    ///
    /// The slice must remain valid until the sound has been loaded.
    /// See the [`Mode`] docs for more information.
    pub const unsafe fn open_memory(data: &'a [u8]) -> Self {
        Self {
            mode: FMOD_OPENMEMORY,
            create_sound_ex_info: FMOD_CREATESOUNDEXINFO {
                length: data.len() as c_uint,
                ..EMPTY_EXINFO
            },
            name_or_data: data.as_ptr().cast(),
            _phantom: PhantomData,
        }
    }

    /// # Safety
    ///
    /// The slice must remain valid until the sound has been released.
    /// Unlike [`Self::open_memory`] this function does not copy the data, so it is even more unsafe!
    pub const unsafe fn open_memory_point(data: &'a [u8]) -> Self {
        Self {
            mode: FMOD_OPENMEMORY_POINT,
            create_sound_ex_info: FMOD_CREATESOUNDEXINFO {
                length: data.len() as c_uint,
                ..EMPTY_EXINFO
            },
            name_or_data: data.as_ptr().cast(),
            _phantom: PhantomData,
        }
    }

    /// # Safety
    ///
    /// The [`FMOD_CREATESOUNDEXINFO`] must be valid.
    #[must_use]
    pub const unsafe fn with_raw_ex_info(mut self, ex_info: FMOD_CREATESOUNDEXINFO) -> Self {
        self.create_sound_ex_info = ex_info;
        self
    }

    #[must_use]
    pub const fn with_file_offset(mut self, file_offset: c_uint) -> Self {
        self.create_sound_ex_info.fileoffset = file_offset;
        self
    }

    #[must_use]
    pub const fn with_open_raw(
        mut self,
        channel_count: c_int,
        default_frequency: c_int,
        format: SoundFormat,
    ) -> Self {
        self.mode |= FMOD_OPENRAW;
        self.create_sound_ex_info.numchannels = channel_count;
        self.create_sound_ex_info.defaultfrequency = default_frequency;
        self.create_sound_ex_info.format = format as _;
        self
    }

    #[must_use]
    pub const fn with_mode(mut self, mode: Mode) -> Self {
        const DISABLE_MODES: Mode = Mode::OPEN_MEMORY
            .union(Mode::OPEN_MEMORY_POINT)
            .union(Mode::OPEN_USER)
            .union(Mode::OPEN_RAW);

        let mode = mode.difference(DISABLE_MODES); // these modes are not allowed to be set by the user, so we unset them
        let mode: FMOD_MODE = mode.bits();
        self.mode |= mode;
        self
    }

    #[must_use]
    pub const fn with_decode_buffer_size(mut self, size: c_uint) -> Self {
        self.create_sound_ex_info.decodebuffersize = size;
        self
    }

    #[must_use]
    pub const fn with_initial_subsound(mut self, initial_subsound: c_int) -> Self {
        self.create_sound_ex_info.initialsubsound = initial_subsound;
        self
    }

    #[must_use]
    pub const fn with_subsound_count(mut self, count: c_int) -> Self {
        self.create_sound_ex_info.numsubsounds = count;
        self
    }

    // TODO: check if this is safe
    #[must_use]
    pub const fn with_inclusion_list(mut self, list: &'a [c_int]) -> Self {
        self.create_sound_ex_info.inclusionlist = list.as_ptr().cast_mut().cast();
        self.create_sound_ex_info.inclusionlistnum = list.len() as c_int;
        self
    }

    // TODO check safety
    #[must_use]
    pub const fn with_dls_name(mut self, dls_name: &'a Utf8CStr) -> Self {
        self.create_sound_ex_info.dlsname = dls_name.as_ptr();
        self
    }

    // TODO check safety
    #[must_use]
    pub const fn with_encryption_key(mut self, key: &'a Utf8CStr) -> Self {
        self.create_sound_ex_info.encryptionkey = key.as_ptr();
        self
    }

    #[must_use]
    pub fn with_max_polyphony(mut self, max_polyphony: c_int) -> Self {
        self.create_sound_ex_info.maxpolyphony = max_polyphony;
        self
    }

    #[must_use]
    pub const fn with_suggested_sound_type(mut self, sound_type: SoundType) -> Self {
        self.create_sound_ex_info.suggestedsoundtype = sound_type as _;
        self
    }

    #[must_use]
    pub const fn with_file_buffer_size(mut self, size: c_int) -> Self {
        self.create_sound_ex_info.filebuffersize = size;
        self
    }

    #[must_use]
    pub const fn with_channel_order(mut self, order: ChannelOrder) -> Self {
        self.create_sound_ex_info.channelorder = order as _;
        self
    }

    #[must_use]
    pub fn with_initial_sound_group(mut self, group: SoundGroup) -> Self {
        self.create_sound_ex_info.initialsoundgroup = group.into();
        self
    }

    #[must_use]
    pub const fn with_initial_seek_position(mut self, position: c_uint, unit: TimeUnit) -> Self {
        self.create_sound_ex_info.initialseekposition = position;
        self.create_sound_ex_info.initialseekpostype = unit as _;
        self
    }

    #[must_use]
    pub fn with_ignore_set_filesystem(mut self, ignore: bool) -> Self {
        self.create_sound_ex_info.ignoresetfilesystem = ignore.into();
        self
    }

    #[must_use]
    pub const fn with_min_midi_granularity(mut self, granularity: c_uint) -> Self {
        self.create_sound_ex_info.minmidigranularity = granularity as _;
        self
    }

    #[must_use]
    pub const fn with_non_block_thread_id(mut self, id: c_int) -> Self {
        self.create_sound_ex_info.nonblockthreadid = id as _;
        self
    }

    // TODO check safety
    #[must_use]
    pub const fn with_fsb_guid(mut self, guid: &'a Guid) -> Self {
        self.create_sound_ex_info.fsbguid = std::ptr::from_ref(guid).cast_mut().cast();
        self
    }

    pub(crate) fn ex_info_is_empty(&self) -> bool {
        self.create_sound_ex_info == EMPTY_EXINFO
    }
}

// getters
impl<'a> SoundBuilder<'a> {
    pub const fn mode(&self) -> Mode {
        Mode::from_bits_truncate(self.mode)
    }

    pub const fn raw_ex_info(&self) -> FMOD_CREATESOUNDEXINFO {
        self.create_sound_ex_info
    }

    pub const fn raw_name_or_data(&self) -> *const c_char {
        self.name_or_data
    }

    pub fn name_or_url(&self) -> Option<&Utf8CStr> {
        if self
            .mode()
            .intersects(Mode::OPEN_MEMORY | Mode::OPEN_MEMORY_POINT | Mode::OPEN_USER)
        {
            None
        } else {
            Some(unsafe { Utf8CStr::from_ptr_unchecked(self.name_or_data) })
        }
    }

    pub fn data(&self) -> Option<&[u8]> {
        if self
            .mode()
            .intersects(Mode::OPEN_MEMORY | Mode::OPEN_MEMORY_POINT)
        {
            Some(unsafe {
                std::slice::from_raw_parts(
                    self.name_or_data.cast(),
                    self.create_sound_ex_info.length as usize,
                )
            })
        } else {
            None
        }
    }

    pub fn length(&self) -> c_uint {
        self.create_sound_ex_info.length
    }

    pub fn file_offset(&self) -> c_uint {
        self.create_sound_ex_info.fileoffset
    }

    pub fn num_channels(&self) -> c_int {
        self.create_sound_ex_info.numchannels
    }

    pub fn default_frequency(&self) -> c_int {
        self.create_sound_ex_info.defaultfrequency
    }

    pub fn format(&self) -> SoundFormat {
        self.create_sound_ex_info.format.try_into().unwrap()
    }

    pub fn decode_buffer_size(&self) -> c_uint {
        self.create_sound_ex_info.decodebuffersize
    }

    pub fn initial_subsound(&self) -> c_int {
        self.create_sound_ex_info.initialsubsound
    }

    pub fn subsound_count(&self) -> c_int {
        self.create_sound_ex_info.numsubsounds
    }

    pub fn inclusion_list(&self) -> Option<&'a [c_int]> {
        if self.create_sound_ex_info.inclusionlist.is_null() {
            None
        } else {
            Some(unsafe {
                std::slice::from_raw_parts(
                    self.create_sound_ex_info.inclusionlist.cast(),
                    self.create_sound_ex_info.inclusionlistnum as usize,
                )
            })
        }
    }

    pub fn dls_name(&self) -> Option<&Utf8CStr> {
        if self.create_sound_ex_info.dlsname.is_null() {
            None
        } else {
            Some(unsafe { Utf8CStr::from_ptr_unchecked(self.create_sound_ex_info.dlsname) })
        }
    }

    pub fn encryption_key(&self) -> Option<&Utf8CStr> {
        if self.create_sound_ex_info.encryptionkey.is_null() {
            None
        } else {
            Some(unsafe { Utf8CStr::from_ptr_unchecked(self.create_sound_ex_info.encryptionkey) })
        }
    }

    pub fn max_polyphony(&self) -> c_int {
        self.create_sound_ex_info.maxpolyphony
    }

    pub fn suggested_sound_type(&self) -> SoundType {
        self.create_sound_ex_info
            .suggestedsoundtype
            .try_into()
            .unwrap()
    }

    pub fn file_buffer_size(&self) -> c_int {
        self.create_sound_ex_info.filebuffersize
    }

    pub fn channel_order(&self) -> ChannelOrder {
        self.create_sound_ex_info.channelorder.try_into().unwrap()
    }

    pub fn initial_sound_group(&self) -> SoundGroup {
        SoundGroup::from(self.create_sound_ex_info.initialsoundgroup)
    }

    pub fn initial_seek_position(&self) -> (c_uint, TimeUnit) {
        (
            self.create_sound_ex_info.initialseekposition,
            self.create_sound_ex_info
                .initialseekpostype
                .try_into()
                .unwrap(),
        )
    }

    pub fn ignore_set_filesystem(&self) -> bool {
        self.create_sound_ex_info.ignoresetfilesystem > 0
    }

    pub fn min_midi_granularity(&self) -> c_uint {
        self.create_sound_ex_info.minmidigranularity
    }

    pub fn non_block_thread_id(&self) -> c_int {
        self.create_sound_ex_info.nonblockthreadid
    }

    pub fn fsb_guid(&self) -> Option<Guid> {
        if self.create_sound_ex_info.fsbguid.is_null() {
            None
        } else {
            Some(unsafe { *(self.create_sound_ex_info.fsbguid.cast()) })
        }
    }
}

impl<'a> SoundBuilder<'a> {
    /// # Safety
    ///
    /// The mode must match the required fields of the [`FMOD_CREATESOUNDEXINFO`] struct.
    /// The [`FMOD_CREATESOUNDEXINFO`] struct's cbsize field must be set to the size of the struct.
    ///
    /// If the mode is not [`Mode::OPEN_MEMORY`] or [`Mode::OPEN_MEMORY_POINT`] `name_or_data` pointer must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// If the mode is [`Mode::OPEN_MEMORY`] or [`Mode::OPEN_MEMORY_POINT`] the data pointer must be valid for reads of bytes up to [`FMOD_CREATESOUNDEXINFO::length`].
    ///
    /// The lifetime of the builder is unbounded and MUST be constrained!
    pub unsafe fn from_ffi(
        name_or_data: *const c_char,
        mode: FMOD_MODE,
        create_sound_ex_info: FMOD_CREATESOUNDEXINFO,
    ) -> Self {
        Self {
            mode,
            create_sound_ex_info,
            name_or_data,
            _phantom: PhantomData,
        }
    }
}
