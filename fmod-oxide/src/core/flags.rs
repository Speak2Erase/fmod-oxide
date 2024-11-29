// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct InitFlags: FMOD_INITFLAGS {
    const NORMAL =                  FMOD_INIT_NORMAL;
    const STREAM_FROM_UPDATE =      FMOD_INIT_STREAM_FROM_UPDATE;
    const MIX_FROM_UPDATE =         FMOD_INIT_MIX_FROM_UPDATE;
    const RIGHTHANDED_3D =          FMOD_INIT_3D_RIGHTHANDED;
    const CLIP_OUTPUT =             FMOD_INIT_CLIP_OUTPUT;
    const CHANNEL_LOWPASS =         FMOD_INIT_CHANNEL_LOWPASS;
    const CHANNEL_DISTANCE_FILTER = FMOD_INIT_CHANNEL_DISTANCEFILTER;
    const PROFILE_ENABLE =          FMOD_INIT_PROFILE_ENABLE;
    const VOL_0_BECOMES_VIRTUAL =   FMOD_INIT_VOL0_BECOMES_VIRTUAL;
    const GEOMETRY_USE_CLOSEST =    FMOD_INIT_GEOMETRY_USECLOSEST;
    const PREFER_DOLBY_DOWNMIX =    FMOD_INIT_PREFER_DOLBY_DOWNMIX;
    /// This flag cannot be used normally as this crate has guardrails preventing it.
    /// It is still here for completeness' sake, though.
    const THREAD_UNSAFE =           FMOD_INIT_THREAD_UNSAFE;
    const PROFILE_METER_ALL =       FMOD_INIT_PROFILE_METER_ALL;
    const MEMORY_TRACKING =         FMOD_INIT_MEMORY_TRACKING;
  }
}

impl From<FMOD_INITFLAGS> for InitFlags {
    fn from(value: FMOD_INITFLAGS) -> Self {
        InitFlags::from_bits_truncate(value)
    }
}

impl From<InitFlags> for FMOD_INITFLAGS {
    fn from(value: InitFlags) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct ThreadAffinity: FMOD_THREAD_AFFINITY {
    const GROUP_DEFAULT      = FMOD_THREAD_AFFINITY_GROUP_DEFAULT       as FMOD_THREAD_AFFINITY;
    const GROUP_A            = FMOD_THREAD_AFFINITY_GROUP_A             as FMOD_THREAD_AFFINITY;
    const GROUP_B            = FMOD_THREAD_AFFINITY_GROUP_B             as FMOD_THREAD_AFFINITY;
    const GROUP_C            = FMOD_THREAD_AFFINITY_GROUP_C             as FMOD_THREAD_AFFINITY;
    const MIXER              = FMOD_THREAD_AFFINITY_MIXER               as FMOD_THREAD_AFFINITY;
    const FEEDER             = FMOD_THREAD_AFFINITY_FEEDER              as FMOD_THREAD_AFFINITY;
    const STREAM             = FMOD_THREAD_AFFINITY_STREAM              as FMOD_THREAD_AFFINITY;
    const FILE               = FMOD_THREAD_AFFINITY_FILE                as FMOD_THREAD_AFFINITY;
    const NONBLOCKING        = FMOD_THREAD_AFFINITY_NONBLOCKING         as FMOD_THREAD_AFFINITY;
    const RECORD             = FMOD_THREAD_AFFINITY_RECORD              as FMOD_THREAD_AFFINITY;
    const GEOMETRY           = FMOD_THREAD_AFFINITY_GEOMETRY            as FMOD_THREAD_AFFINITY;
    const PROFILER           = FMOD_THREAD_AFFINITY_PROFILER            as FMOD_THREAD_AFFINITY;
    const STUDIO_UPDATE      = FMOD_THREAD_AFFINITY_STUDIO_UPDATE       as FMOD_THREAD_AFFINITY;
    const STUDIO_LOAD_BANK   = FMOD_THREAD_AFFINITY_STUDIO_LOAD_BANK    as FMOD_THREAD_AFFINITY;
    const STUDIO_LOAD_SAMPLE = FMOD_THREAD_AFFINITY_STUDIO_LOAD_SAMPLE  as FMOD_THREAD_AFFINITY;
    const CORE_ALL           = FMOD_THREAD_AFFINITY_CORE_ALL            as FMOD_THREAD_AFFINITY;
    const CORE_0             = FMOD_THREAD_AFFINITY_CORE_0              as FMOD_THREAD_AFFINITY;
    const CORE_1             = FMOD_THREAD_AFFINITY_CORE_1              as FMOD_THREAD_AFFINITY;
    const CORE_2             = FMOD_THREAD_AFFINITY_CORE_2              as FMOD_THREAD_AFFINITY;
    const CORE_3             = FMOD_THREAD_AFFINITY_CORE_3              as FMOD_THREAD_AFFINITY;
    const CORE_4             = FMOD_THREAD_AFFINITY_CORE_4              as FMOD_THREAD_AFFINITY;
    const CORE_5             = FMOD_THREAD_AFFINITY_CORE_5              as FMOD_THREAD_AFFINITY;
    const CORE_6             = FMOD_THREAD_AFFINITY_CORE_6              as FMOD_THREAD_AFFINITY;
    const CORE_7             = FMOD_THREAD_AFFINITY_CORE_7              as FMOD_THREAD_AFFINITY;
    const CORE_8             = FMOD_THREAD_AFFINITY_CORE_8              as FMOD_THREAD_AFFINITY;
    const CORE_9             = FMOD_THREAD_AFFINITY_CORE_9              as FMOD_THREAD_AFFINITY;
    const CORE_10            = FMOD_THREAD_AFFINITY_CORE_10             as FMOD_THREAD_AFFINITY;
    const CORE_11            = FMOD_THREAD_AFFINITY_CORE_11             as FMOD_THREAD_AFFINITY;
    const CORE_12            = FMOD_THREAD_AFFINITY_CORE_12             as FMOD_THREAD_AFFINITY;
    const CORE_13            = FMOD_THREAD_AFFINITY_CORE_13             as FMOD_THREAD_AFFINITY;
    const CORE_14            = FMOD_THREAD_AFFINITY_CORE_14             as FMOD_THREAD_AFFINITY;
    const CORE_15            = FMOD_THREAD_AFFINITY_CORE_15             as FMOD_THREAD_AFFINITY;
  }
}

impl From<FMOD_THREAD_AFFINITY> for ThreadAffinity {
    fn from(value: FMOD_THREAD_AFFINITY) -> Self {
        ThreadAffinity::from_bits_truncate(value)
    }
}

impl From<ThreadAffinity> for FMOD_THREAD_AFFINITY {
    fn from(value: ThreadAffinity) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct Mode: FMOD_MODE {
    const DEFAULT                   = FMOD_DEFAULT;
    const LOOP_OFF                  = FMOD_LOOP_OFF;
    const LOOP_NORMAL               = FMOD_LOOP_NORMAL;
    const LOOP_BIDI                 = FMOD_LOOP_BIDI;
    const D2                        = FMOD_2D;
    const D3                        = FMOD_3D;
    const CREATE_STREAM             = FMOD_CREATESTREAM;
    const CREATE_SAMPLE             = FMOD_CREATESAMPLE;
    const CREATE_COMPRESSED_SAMPLE  = FMOD_CREATECOMPRESSEDSAMPLE;
    const OPEN_USER                 = FMOD_OPENUSER;
    const OPEN_MEMORY               = FMOD_OPENMEMORY;
    const OPEN_MEMORY_POINT         = FMOD_OPENMEMORY_POINT;
    const OPEN_RAW                  = FMOD_OPENRAW;
    const OPEN_ONLY                 = FMOD_OPENONLY;
    const ACCURATE_TIME             = FMOD_ACCURATETIME;
    const MPEG_SEARCH               = FMOD_MPEGSEARCH;
    const NONBLOCKING               = FMOD_NONBLOCKING;
    const UNIQUE                    = FMOD_UNIQUE;
    const HEADRELATIVE_3D           = FMOD_3D_HEADRELATIVE;
    const WORLDRELATIVE_3D          = FMOD_3D_WORLDRELATIVE;
    const INVERSE_ROLLOFF_3D        = FMOD_3D_INVERSEROLLOFF;
    const LINEAR_ROLLOFF_3D         = FMOD_3D_LINEARROLLOFF;
    const LINEAR_SQUARE_ROLLOFF_3D  = FMOD_3D_LINEARSQUAREROLLOFF;
    const INVERSE_TAPERED_ROLLOFF_3D = FMOD_3D_INVERSETAPEREDROLLOFF;
    const CUSTOM_ROLLOFF_3D         = FMOD_3D_CUSTOMROLLOFF;
    const IGNORE_GEOMETRY_3D        = FMOD_3D_IGNOREGEOMETRY;
    const IGNORE_TAGS               = FMOD_IGNORETAGS;
    const LOWMEM                    = FMOD_LOWMEM;
    const VIRTUAL_PLAYFROM_START    = FMOD_VIRTUAL_PLAYFROMSTART;
  }
}

impl From<FMOD_MODE> for Mode {
    fn from(value: FMOD_MODE) -> Self {
        Mode::from_bits_truncate(value)
    }
}

impl From<Mode> for FMOD_MODE {
    fn from(value: Mode) -> Self {
        value.bits()
    }
}

// FIXME: this is deprecated..?
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct ChannelMask: FMOD_CHANNELMASK {
    const FRONT_LEFT        = FMOD_CHANNELMASK_FRONT_LEFT;
    const FRONT_RIGHT       = FMOD_CHANNELMASK_FRONT_RIGHT;
    const FRONT_CENTER      = FMOD_CHANNELMASK_FRONT_CENTER;
    const LOW_FREQUENCY     = FMOD_CHANNELMASK_LOW_FREQUENCY;
    const SURROUND_LEFT     = FMOD_CHANNELMASK_SURROUND_LEFT;
    const SURROUND_RIGHT    = FMOD_CHANNELMASK_SURROUND_RIGHT ;
    const BACK_LEFT         = FMOD_CHANNELMASK_BACK_LEFT;
    const BACK_RIGHT        = FMOD_CHANNELMASK_BACK_RIGHT;
    const BACK_CENTER       = FMOD_CHANNELMASK_BACK_CENTER;
    const MONO              = FMOD_CHANNELMASK_MONO;
    const STEREO            = FMOD_CHANNELMASK_STEREO;
    const LRC               = FMOD_CHANNELMASK_LRC;
    const QUAD              = FMOD_CHANNELMASK_QUAD;
    const SURROUND          = FMOD_CHANNELMASK_SURROUND;
    const _5POINT1          = FMOD_CHANNELMASK_5POINT1;
    const _5POINT1_REARS    = FMOD_CHANNELMASK_5POINT1_REARS;
    const _7POINT0          = FMOD_CHANNELMASK_7POINT0;
    const _7POINT1          = FMOD_CHANNELMASK_7POINT1;
  }
}

impl From<FMOD_CHANNELMASK> for ChannelMask {
    fn from(value: FMOD_CHANNELMASK) -> Self {
        ChannelMask::from_bits_truncate(value)
    }
}

impl From<ChannelMask> for FMOD_CHANNELMASK {
    fn from(value: ChannelMask) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct DriverState: FMOD_DRIVER_STATE {
    const CONNECTED = FMOD_DRIVER_STATE_CONNECTED;
    const DEFAULT   = FMOD_DRIVER_STATE_DEFAULT;
  }
}

impl From<FMOD_DRIVER_STATE> for DriverState {
    fn from(value: FMOD_DRIVER_STATE) -> Self {
        DriverState::from_bits_truncate(value)
    }
}

impl From<DriverState> for FMOD_DRIVER_STATE {
    fn from(value: DriverState) -> Self {
        value.bits()
    }
}
