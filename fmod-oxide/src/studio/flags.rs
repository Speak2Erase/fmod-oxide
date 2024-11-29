// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(missing_docs)]

use fmod_sys::*;
use std::ffi::c_uint;

bitflags::bitflags! {
    /// Studio System initialization flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct InitFlags: c_uint {
        /// Use defaults for all initialization options.
        const NORMAL                = FMOD_STUDIO_INIT_NORMAL;
        /// Enable live update.
        const LIVEUPDATE            = FMOD_STUDIO_INIT_LIVEUPDATE;
        /// Load banks even if they reference plug-ins that have not been loaded.
        const ALLOW_MISSING_PLUGINS = FMOD_STUDIO_INIT_ALLOW_MISSING_PLUGINS;
        /// Disable asynchronous processing and perform all processing on the calling thread instead.
        const SYNCHRONOUS_UPDATE    = FMOD_STUDIO_INIT_SYNCHRONOUS_UPDATE;
        /// Defer timeline callbacks until the main update.
        const DEFERRED_CALLBACKS    = FMOD_STUDIO_INIT_DEFERRED_CALLBACKS;
        /// No additional threads are created for bank and resource loading.
        const LOAD_FROM_UPDATE      = FMOD_STUDIO_INIT_LOAD_FROM_UPDATE;
        /// Enables detailed memory usage statistics. Increases memory footprint and impacts performance.
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
    /// Flags for controlling bank loading.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LoadBankFlags: c_uint {
        /// Standard behavior.
        const NORMAL             = FMOD_STUDIO_LOAD_BANK_NORMAL;
        /// Bank loading occurs asynchronously rather than occurring immediately.
        const NONBLOCKING        = FMOD_STUDIO_LOAD_BANK_NONBLOCKING;
        /// Force samples to decompress into memory when they are loaded, rather than staying compressed.
        const DECOMPRESS_SAMPLES = FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES;
        /// Ignore the encryption key specified by `AdvancedSettings` when loading sounds from this bank (assume the sounds in the bank are not encrypted).
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
    /// Flags describing the behavior of a parameter.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ParameterFlags: c_uint {
        /// Read only.
        const READONLY = FMOD_STUDIO_PARAMETER_READONLY;
        /// Automatic parameter.
        const AUTOMATIC = FMOD_STUDIO_PARAMETER_AUTOMATIC;
        /// Global parameter.
        const GLOBAL = FMOD_STUDIO_PARAMETER_GLOBAL;
        /// Discrete parameter that operates on integers (whole numbers) rather than continuous fractional numbers.
        const DISCRETE = FMOD_STUDIO_PARAMETER_DISCRETE;
        /// Labeled discrete parameter that has a label for each integer value.
        /// This flag is never set in banks built with FMOD Studio versions prior to 2.01.10.
        /// If this flag is set, `DISCRETE` is also set.
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
    /// Flags controling command capture.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CommandCaptureFlags: c_uint {
        /// Use default options.
        const NORMAL = FMOD_STUDIO_COMMANDCAPTURE_NORMAL;
        /// Call file flush on every command.
        const FILE_FLUSH = FMOD_STUDIO_COMMANDCAPTURE_FILEFLUSH;
        /// The initial state of banks and instances is captured, unless this flag is set.
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
    /// Flags controlling command replay.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CommandReplayFlags: c_uint {
        /// Use default options.
        const NORMAL = FMOD_STUDIO_COMMANDCAPTURE_NORMAL;
        /// Do not free resources at the end of playback.
        const SKIP_CLEANUP = FMOD_STUDIO_COMMANDREPLAY_SKIP_CLEANUP;
        /// Play back at maximum speed, ignoring the timing of the original replay.
        const FAST_FORWARD = FMOD_STUDIO_COMMANDREPLAY_FAST_FORWARD;
        /// Skip commands related to bank loading.
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
    /// A mask used to determine what callbacks can or cannot be called.
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

#[allow(missing_docs)]
bitflags::bitflags! {
    /// A mask used to determine what callbacks can or cannot be called.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SystemCallbackMask: c_uint {
        const PREUPDATE = FMOD_STUDIO_SYSTEM_CALLBACK_PREUPDATE;
        const POSTUPDATE = FMOD_STUDIO_SYSTEM_CALLBACK_POSTUPDATE;
        const BANK_UNLOAD = FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD;
        const LIVEUPDATE_CONNECTED = FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_CONNECTED;
        const LIVEUPDATE_DISCONNECTED = FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_DISCONNECTED;
    }
}

impl From<FMOD_STUDIO_SYSTEM_CALLBACK_TYPE> for SystemCallbackMask {
    fn from(value: FMOD_STUDIO_SYSTEM_CALLBACK_TYPE) -> Self {
        SystemCallbackMask::from_bits_truncate(value)
    }
}

impl From<SystemCallbackMask> for FMOD_STUDIO_SYSTEM_CALLBACK_TYPE {
    fn from(value: SystemCallbackMask) -> Self {
        value.bits()
    }
}
