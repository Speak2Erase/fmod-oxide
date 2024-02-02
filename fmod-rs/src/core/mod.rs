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

use std::ffi::c_uint;

use fmod_sys::*;

mod channel_group;
pub use channel_group::*;

mod system;
pub use system::*;

mod sound;
pub use sound::*;

mod dsp;
pub use dsp::*;

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct InitFlags: c_uint {
    const NORMAL = FMOD_INIT_NORMAL;
    const STREAM_FROM_UPDATE = FMOD_INIT_STREAM_FROM_UPDATE;
    const MIX_FROM_UPDATE = FMOD_INIT_MIX_FROM_UPDATE;
    const RIGHTHANDED_3D = FMOD_INIT_3D_RIGHTHANDED;
    const CLIP_OUTPUT = FMOD_INIT_CLIP_OUTPUT;
    const CHANNEL_LOWPASS = FMOD_INIT_CHANNEL_LOWPASS;
    const CHANNEL_DISTANCE_FILTER = FMOD_INIT_CHANNEL_DISTANCEFILTER;
    const PROFILE_ENABLE = FMOD_INIT_PROFILE_ENABLE;
    const VOL_0_BECOMES_VIRTUAL = FMOD_INIT_VOL0_BECOMES_VIRTUAL;
    const GEOMETRY_USE_CLOSEST = FMOD_INIT_GEOMETRY_USECLOSEST;
    const PREFER_DOLBY_DOWNMIX = FMOD_INIT_PREFER_DOLBY_DOWNMIX;
    const THREAD_UNSAFE = FMOD_INIT_THREAD_UNSAFE;
    const PROFILE_METER_ALL = FMOD_INIT_PROFILE_METER_ALL;
    const MEMORY_TRACKING = FMOD_INIT_MEMORY_TRACKING;
  }
}
