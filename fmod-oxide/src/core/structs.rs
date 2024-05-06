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

use crate::DspParameterDataType;

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
    decay_time: c_float,
    early_delay: c_float,
    late_delay: c_float,
    hf_reference: c_float,
    hf_decay_ratio: c_float,
    diffusion: c_float,
    density: c_float,
    low_shelf_frequency: c_float,
    low_shelf_gain: c_float,
    high_cut: c_float,
    early_late_mix: c_float,
    wet_level: c_float,
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

impl ReverbProperties {
    // bindgen doesn't generate these so we have to do this ourselves
    pub const OFF: Self = Self {
        decay_time: 1000.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 100.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 20.0,
        early_late_mix: 96.0,
        wet_level: -80.0,
    };
    pub const GENERIC: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 83.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 14500.0,
        early_late_mix: 96.0,
        wet_level: -8.0,
    };
    pub const PADDEDCELL: Self = Self {
        decay_time: 170.0,
        early_delay: 1.0,
        late_delay: 2.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 10.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 160.0,
        early_late_mix: 84.0,
        wet_level: -7.8,
    };
    pub const ROOM: Self = Self {
        decay_time: 400.0,
        early_delay: 2.0,
        late_delay: 3.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 83.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 6050.0,
        early_late_mix: 88.0,
        wet_level: -9.4,
    };
    pub const BATHROOM: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 54.0,
        diffusion: 100.0,
        density: 60.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 2900.0,
        early_late_mix: 83.0,
        wet_level: 0.5,
    };
    pub const LIVINGROOM: Self = Self {
        decay_time: 500.0,
        early_delay: 3.0,
        late_delay: 4.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 10.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 160.0,
        early_late_mix: 58.0,
        wet_level: -19.0,
    };
    pub const STONEROOM: Self = Self {
        decay_time: 2300.0,
        early_delay: 12.0,
        late_delay: 17.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 64.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 7800.0,
        early_late_mix: 71.0,
        wet_level: -8.5,
    };
    pub const AUDITORIUM: Self = Self {
        decay_time: 4300.0,
        early_delay: 20.0,
        late_delay: 30.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 59.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 5850.0,
        early_late_mix: 64.0,
        wet_level: -11.7,
    };
    pub const CONCERTHALL: Self = Self {
        decay_time: 3900.0,
        early_delay: 20.0,
        late_delay: 29.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 70.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 5650.0,
        early_late_mix: 80.0,
        wet_level: -9.8,
    };
    pub const CAVE: Self = Self {
        decay_time: 2900.0,
        early_delay: 15.0,
        late_delay: 22.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 100.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 20000.0,
        early_late_mix: 59.0,
        wet_level: -11.3,
    };
    pub const ARENA: Self = Self {
        decay_time: 7200.0,
        early_delay: 20.0,
        late_delay: 30.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 33.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 4500.0,
        early_late_mix: 80.0,
        wet_level: -9.6,
    };
    pub const HANGAR: Self = Self {
        decay_time: 10000.0,
        early_delay: 20.0,
        late_delay: 30.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 23.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 3400.0,
        early_late_mix: 72.0,
        wet_level: -7.4,
    };
    pub const CARPETTEDHALLWAY: Self = Self {
        decay_time: 300.0,
        early_delay: 2.0,
        late_delay: 30.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 10.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 500.0,
        early_late_mix: 56.0,
        wet_level: -24.0,
    };
    pub const HALLWAY: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 59.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 7800.0,
        early_late_mix: 87.0,
        wet_level: -5.5,
    };
    pub const STONECORRIDOR: Self = Self {
        decay_time: 270.0,
        early_delay: 13.0,
        late_delay: 20.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 79.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 9000.0,
        early_late_mix: 86.0,
        wet_level: -6.0,
    };
    pub const ALLEY: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 86.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 8300.0,
        early_late_mix: 80.0,
        wet_level: -9.8,
    };
    pub const FOREST: Self = Self {
        decay_time: 1500.0,
        early_delay: 162.0,
        late_delay: 88.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 54.0,
        diffusion: 79.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 760.0,
        early_late_mix: 94.0,
        wet_level: -12.3,
    };
    pub const CITY: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 67.0,
        diffusion: 50.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 4050.0,
        early_late_mix: 66.0,
        wet_level: -26.0,
    };
    pub const MOUNTAINS: Self = Self {
        decay_time: 1500.0,
        early_delay: 300.0,
        late_delay: 100.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 21.0,
        diffusion: 27.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 1220.0,
        early_late_mix: 82.0,
        wet_level: -24.0,
    };
    pub const QUARRY: Self = Self {
        decay_time: 1500.0,
        early_delay: 61.0,
        late_delay: 25.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 83.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 3400.0,
        early_late_mix: 100.0,
        wet_level: -5.0,
    };
    pub const PLAIN: Self = Self {
        decay_time: 1500.0,
        early_delay: 179.0,
        late_delay: 100.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 50.0,
        diffusion: 21.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 1670.0,
        early_late_mix: 65.0,
        wet_level: -28.0,
    };
    pub const PARKINGLOT: Self = Self {
        decay_time: 1700.0,
        early_delay: 8.0,
        late_delay: 12.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 100.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 20000.0,
        early_late_mix: 56.0,
        wet_level: -19.5,
    };
    pub const SEWERPIPE: Self = Self {
        decay_time: 2800.0,
        early_delay: 14.0,
        late_delay: 21.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 14.0,
        diffusion: 80.0,
        density: 60.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 3400.0,
        early_late_mix: 66.0,
        wet_level: 1.2,
    };
    pub const UNDERWATER: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 10.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 500.0,
        early_late_mix: 92.0,
        wet_level: 7.0,
    };
}

pub struct DspParameterDescription {
    kind: DspParameterType,
    name: Utf8CString,
    label: Utf8CString,
    description: Utf8CString,
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
            FMOD_DSP_PARAMETER_TYPE_FMOD_DSP_PARAMETER_TYPE_FLOAT => {
                let floatdesc = unsafe { value.__bindgen_anon_1.floatdesc };
                DspParameterType::Float {
                    min: floatdesc.min,
                    max: floatdesc.max,
                    default: floatdesc.defaultval,
                    mapping: FloatMapping {},
                }
            }
            FMOD_DSP_PARAMETER_TYPE_FMOD_DSP_PARAMETER_TYPE_INT => {
                let intdesc = unsafe { value.__bindgen_anon_1.intdesc };
                DspParameterType::Int {
                    min: intdesc.min,
                    max: intdesc.max,
                    default: intdesc.defaultval,
                    goes_to_infinity: intdesc.goestoinf.into(),
                }
            }
            FMOD_DSP_PARAMETER_TYPE_FMOD_DSP_PARAMETER_TYPE_BOOL => {
                let booldesc = unsafe { value.__bindgen_anon_1.booldesc };
                DspParameterType::Bool {
                    default: booldesc.defaultval.into(),
                }
            }
            FMOD_DSP_PARAMETER_TYPE_FMOD_DSP_PARAMETER_TYPE_DATA => {
                let datadesc = unsafe { value.__bindgen_anon_1.datadesc };
                DspParameterType::Data {
                    data_type: datadesc.datatype.into(),
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
