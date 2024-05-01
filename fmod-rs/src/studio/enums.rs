// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::{c_float, c_int};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LoadingState {
    Unloading = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADING,
    Unloaded = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADED,
    Loading = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADING,
    Loaded = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADED,
    Error(Error) = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_ERROR,
}

impl LoadingState {
    pub fn try_from_ffi(value: FMOD_STUDIO_LOADING_STATE, error: Option<Error>) -> Result<Self> {
        match value {
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADING => {
                Ok(LoadingState::Unloading)
            }
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADED => {
                Ok(LoadingState::Unloaded)
            }
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADING => {
                Ok(LoadingState::Loading)
            }
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADED => Ok(LoadingState::Loaded),
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_ERROR => error
                .map(LoadingState::Error)
                .ok_or(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM)),
            _ => Err(Error::EnumFromPrivitive {
                name: "LoadingState",
                primitive: value,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum StopMode {
    AllowFadeout = FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_ALLOWFADEOUT,
    Immediate = FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_IMMEDIATE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum ParameterKind {
    GameControlled = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_GAME_CONTROLLED,
    AutomaticDistance = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE,
    AutomaticEventConeAngle =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_CONE_ANGLE,
    AutomaticEventOrientation =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_ORIENTATION,
    AutomaticDirection = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DIRECTION,
    AutomaticElevation = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_ELEVATION,
    AutomaticListenerOrientation =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_LISTENER_ORIENTATION,
    AutomaticSpeed = FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED,
    AutomaticSpeedAbsolute =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED_ABSOLUTE,
    AutomaticDistanceNormalized =
        FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE_NORMALIZED,
}

#[derive(Clone, PartialEq, Debug)]
pub enum UserPropertyKind {
    Int(c_int),
    Bool(bool),
    Float(c_float),
    String(Utf8CString),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum PlaybackState {
    Playing = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_PLAYING,
    Sustaining = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_SUSTAINING,
    Stopped = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPED,
    Starting = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STARTING,
    Stopping = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPING,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum EventProperty {
    ChannelPriority = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY,
    ScheduleDelay = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY,
    ScheduleLookahead = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD,
    MinimumDistance = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE,
    MaximumDistance = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE,
    Cooldown = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum InstanceType {
    None = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_NONE,
    System = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_SYSTEM,
    EventDescription = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION,
    EventInstance = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE,
    ParameterInstance = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE,
    Bus = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BUS,
    Vca = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_VCA,
    Bank = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BANK,
    CommandReplay = FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY,
}
