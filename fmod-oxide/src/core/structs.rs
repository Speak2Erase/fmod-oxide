// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::{
    ffi::{c_float, c_int, c_short, c_uchar, c_uint, c_ushort},
    mem::MaybeUninit,
};

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};

use crate::{DspParameterDataType, TagType};

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
    sample_count: c_int,
    peak_level: [c_float; 32],
    rms_level: [c_float; 32],
    channel_count: c_short,
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
    kind: TagType,
    name: Utf8CString,
    data: TagData,
    updated: bool,
}

pub enum TagData {
    Binary(Vec<u8>),
    Integer(i64),
    Float(f64),
    Utf8String(Utf8CString),
    // TODO other string types
}

impl Tag {
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
                    8 => TagData::Integer(*value.data.cast::<i64>() as i64),
                    _ => panic!("unrecognized integer data len"),
                },
                FMOD_TAGDATATYPE_FLOAT => match value.datalen {
                    4 => TagData::Float(*value.data.cast::<f32>() as f64),
                    8 => TagData::Float(*value.data.cast::<f64>() as f64),
                    _ => panic!("unrecognized float data len"),
                },
                FMOD_TAGDATATYPE_STRING_UTF8 => {
                    let string = Utf8CStr::from_ptr_unchecked(value.data.cast()).to_cstring();
                    TagData::Utf8String(string)
                }
                _ => unimplemented!(), // TODO
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
