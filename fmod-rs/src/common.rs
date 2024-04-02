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

use std::{
    ffi::{c_float, c_uchar, c_uint, c_ushort, CString},
    mem::MaybeUninit,
};

use fmod_sys::*;

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
    pub fn parse(string: &str) -> Result<Self> {
        let mut guid = MaybeUninit::uninit();
        let string = CString::new(string)?;
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

        f.write_fmt(format_args!("{{{data_1:0>8x}-{data_2:0>4x}-{data_3:0>4x}-"))?;
        f.write_fmt(format_args!("{:0>2x}{:0>2x}-", data_4[0], data_4[1]))?;
        for b in &data_4[2..] {
            f.write_fmt(format_args!("{b:0>2x}"))?;
        }

        f.write_fmt(format_args!("}}"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
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
