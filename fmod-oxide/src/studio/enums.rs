// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::{c_float, c_int};

/// Loading state of various objects.
#[derive(Debug, Clone, PartialEq, Eq)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum LoadingState {
    /// Currently unloading.
    Unloading = FMOD_STUDIO_LOADING_STATE_UNLOADING,
    /// Not loaded.
    Unloaded = FMOD_STUDIO_LOADING_STATE_UNLOADED,
    /// Loading in progress.
    Loading = FMOD_STUDIO_LOADING_STATE_LOADING,
    /// Loaded and ready to play.
    Loaded = FMOD_STUDIO_LOADING_STATE_LOADED,
    /// Failed to load.
    Error(Error) = FMOD_STUDIO_LOADING_STATE_ERROR,
}

impl LoadingState {
    /// Try creating a `LoadingState` from its FFI equivalent.
    pub fn try_from_ffi(value: FMOD_STUDIO_LOADING_STATE, error: Option<Error>) -> Result<Self> {
        match value {
            FMOD_STUDIO_LOADING_STATE_UNLOADING => Ok(LoadingState::Unloading),
            FMOD_STUDIO_LOADING_STATE_UNLOADED => Ok(LoadingState::Unloaded),
            FMOD_STUDIO_LOADING_STATE_LOADING => Ok(LoadingState::Loading),
            FMOD_STUDIO_LOADING_STATE_LOADED => Ok(LoadingState::Loaded),
            FMOD_STUDIO_LOADING_STATE_ERROR => error
                .map(LoadingState::Error)
                .ok_or(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM)),
            _ => Err(Error::EnumFromPrivitive {
                name: "LoadingState",
                primitive: i64::from(value),
            }),
        }
    }
}

/// Loaded and ready to play.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum StopMode {
    /// Allows AHDSR modulators to complete their release, and DSP effect tails to play out.
    AllowFadeout = FMOD_STUDIO_STOP_ALLOWFADEOUT,
    /// Stops the event instance immediately.
    Immediate = FMOD_STUDIO_STOP_IMMEDIATE,
}

/// Event parameter types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum ParameterKind {
    /// API settable parameter.
    GameControlled = FMOD_STUDIO_PARAMETER_GAME_CONTROLLED,
    /// Distance between the event and the listener.
    AutomaticDistance = FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE,
    /// Angle between the event's forward vector and the vector pointing from the event to the listener (0 to 180 degrees).
    AutomaticEventConeAngle = FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_CONE_ANGLE,
    /// Horizontal angle between the event's forward vector and listener's forward vector (-180 to 180 degrees).
    AutomaticEventOrientation = FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_ORIENTATION,
    /// Horizontal angle between the listener's forward vector and the vector pointing from the listener to the event (-180 to 180 degrees).
    AutomaticDirection = FMOD_STUDIO_PARAMETER_AUTOMATIC_DIRECTION,
    /// Angle between the listener's XZ plane and the vector pointing from the listener to the event (-90 to 90 degrees).
    AutomaticElevation = FMOD_STUDIO_PARAMETER_AUTOMATIC_ELEVATION,
    /// Horizontal angle between the listener's forward vector and the global positive Z axis (-180 to 180 degrees).
    AutomaticListenerOrientation = FMOD_STUDIO_PARAMETER_AUTOMATIC_LISTENER_ORIENTATION,
    /// Magnitude of the relative velocity of the event and the listener.
    AutomaticSpeed = FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED,
    /// Magnitude of the absolute velocity of the event.
    AutomaticSpeedAbsolute = FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED_ABSOLUTE,
    /// Distance between the event and the listener in the range min distance - max distance represented as 0 - 1.
    AutomaticDistanceNormalized = FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE_NORMALIZED,
}

/// User property types.
#[derive(Clone, PartialEq, Debug)]
pub enum UserPropertyKind {
    /// Integer.
    Int(c_int),
    /// Boolean.
    Bool(bool),
    /// Float.
    Float(c_float),
    /// String.
    String(Utf8CString),
}

/// Playback state of various objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum PlaybackState {
    /// Playing.
    Playing = FMOD_STUDIO_PLAYBACK_PLAYING,
    /// The timeline cursor is paused on a sustain point. (`EventInstance` only.)
    Sustaining = FMOD_STUDIO_PLAYBACK_SUSTAINING,
    /// Stopped.
    Stopped = FMOD_STUDIO_PLAYBACK_STOPPED,
    /// Preparing to start.
    Starting = FMOD_STUDIO_PLAYBACK_STARTING,
    /// Preparing to stop.
    Stopping = FMOD_STUDIO_PLAYBACK_STOPPING,
}

/// These definitions describe built-in event properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum EventProperty {
    /// Priority to set on Core API Channels created by this event instance, or -1 for default.
    ChannelPriority = FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY,
    /// Schedule delay in DSP clocks, or -1 for default.
    ScheduleDelay = FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY,
    /// Schedule look-ahead on the timeline in DSP clocks, or -1 for default.
    ScheduleLookahead = FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD,
    /// Override the event's 3D minimum distance, or -1 for default.
    MinimumDistance = FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE,
    /// Override the event's 3D maximum distance, or -1 for default.
    MaximumDistance = FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE,
    /// Override the event's cooldown, or -1 for default.
    Cooldown = FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN,
}

/// Command replay command instance handle types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum InstanceType {
    /// No type, handle is unused.
    None = FMOD_STUDIO_INSTANCETYPE_NONE,
    /// `System`.
    System = FMOD_STUDIO_INSTANCETYPE_SYSTEM,
    /// `EventDescription`.
    EventDescription = FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION,
    /// `EventInstance`.
    EventInstance = FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE,
    /// `ParameterInstance`.
    ParameterInstance = FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE,
    /// `Bus`.
    Bus = FMOD_STUDIO_INSTANCETYPE_BUS,
    /// `Vca`.
    Vca = FMOD_STUDIO_INSTANCETYPE_VCA,
    /// `Bank`.
    Bank = FMOD_STUDIO_INSTANCETYPE_BANK,
    /// `CommandReplay`.
    CommandReplay = FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY,
}
