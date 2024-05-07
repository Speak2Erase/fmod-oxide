// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_uint;

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
