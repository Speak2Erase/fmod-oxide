// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
