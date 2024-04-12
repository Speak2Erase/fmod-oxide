// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::{c_float, c_int};

use super::{
    Bank, EventInstance, PluginInstanceProperties, ProgrammerSoundProperties,
    TimelineBeatProperties, TimelineMarkerProperties, TimelineNestedBeatProperties,
};
use crate::{core::Sound, UserdataTypes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LoadingState {
    Unloading = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADING,
    Unloaded = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADED,
    Loading = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADING,
    Loaded = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADED,
    Error = FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_ERROR,
}

impl From<FMOD_STUDIO_LOADING_STATE> for LoadingState {
    fn from(value: FMOD_STUDIO_LOADING_STATE) -> Self {
        match value {
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADING => {
                LoadingState::Unloading
            }
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_UNLOADED => LoadingState::Unloaded,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADING => LoadingState::Loading,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_LOADED => LoadingState::Loaded,
            FMOD_STUDIO_LOADING_STATE_FMOD_STUDIO_LOADING_STATE_ERROR => LoadingState::Error,
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<LoadingState> for FMOD_STUDIO_LOADING_STATE {
    fn from(value: LoadingState) -> Self {
        value as FMOD_STUDIO_LOADING_STATE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StopMode {
    AllowFadeout = FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_ALLOWFADEOUT,
    Immediate = FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_IMMEDIATE,
}

impl From<FMOD_STUDIO_STOP_MODE> for StopMode {
    fn from(value: FMOD_STUDIO_STOP_MODE) -> Self {
        match value {
            FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_ALLOWFADEOUT => StopMode::AllowFadeout,
            FMOD_STUDIO_STOP_MODE_FMOD_STUDIO_STOP_IMMEDIATE => StopMode::Immediate,
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<StopMode> for FMOD_STUDIO_STOP_MODE {
    fn from(value: StopMode) -> Self {
        value as FMOD_STUDIO_STOP_MODE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<FMOD_STUDIO_PARAMETER_TYPE> for ParameterKind {
    fn from(value: FMOD_STUDIO_PARAMETER_TYPE) -> Self {
        match value {
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_GAME_CONTROLLED => {
                ParameterKind::GameControlled
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE => {
                ParameterKind::AutomaticDistance
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_CONE_ANGLE => {
                ParameterKind::AutomaticEventConeAngle
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_ORIENTATION => {
                ParameterKind::AutomaticEventOrientation
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DIRECTION => {
                ParameterKind::AutomaticDirection
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_ELEVATION => {
                ParameterKind::AutomaticElevation
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_LISTENER_ORIENTATION => {
                ParameterKind::AutomaticListenerOrientation
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED => {
                ParameterKind::AutomaticSpeed
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED_ABSOLUTE => {
                ParameterKind::AutomaticSpeedAbsolute
            }
            FMOD_STUDIO_PARAMETER_TYPE_FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE_NORMALIZED => {
                ParameterKind::AutomaticDistanceNormalized
            }
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<ParameterKind> for FMOD_STUDIO_PARAMETER_TYPE {
    fn from(value: ParameterKind) -> Self {
        value as FMOD_STUDIO_PARAMETER_TYPE
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum UserPropertyKind {
    Int(c_int),
    Bool(bool),
    Float(c_float),
    String(Utf8CString),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PlaybackState {
    Playing = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_PLAYING,
    Sustaining = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_SUSTAINING,
    Stopped = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPED,
    Starting = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STARTING,
    Stopping = FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPING,
}

impl From<FMOD_STUDIO_PLAYBACK_STATE> for PlaybackState {
    fn from(value: FMOD_STUDIO_PLAYBACK_STATE) -> Self {
        match value {
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_PLAYING => PlaybackState::Playing,
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_SUSTAINING => PlaybackState::Sustaining,
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPED => PlaybackState::Stopped,
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STARTING => PlaybackState::Starting,
            FMOD_STUDIO_PLAYBACK_STATE_FMOD_STUDIO_PLAYBACK_STOPPING => PlaybackState::Stopping,
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<PlaybackState> for FMOD_STUDIO_PLAYBACK_STATE {
    fn from(value: PlaybackState) -> Self {
        value as FMOD_STUDIO_PLAYBACK_STATE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EventProperty {
    ChannelPriority = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY,
    ScheduleDelay = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY,
    ScheduleLookahead = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD,
    MinimumDistance = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE,
    MaximumDistance = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE,
    Cooldown = FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN,
}

impl From<FMOD_STUDIO_EVENT_PROPERTY> for EventProperty {
    fn from(value: FMOD_STUDIO_EVENT_PROPERTY) -> Self {
        match value {
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY => {
                EventProperty::ChannelPriority
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY => {
                EventProperty::ScheduleDelay
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD => {
                EventProperty::ScheduleLookahead
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE => {
                EventProperty::MinimumDistance
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE => {
                EventProperty::MaximumDistance
            }
            FMOD_STUDIO_EVENT_PROPERTY_FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN => {
                EventProperty::Cooldown
            }
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<EventProperty> for FMOD_STUDIO_EVENT_PROPERTY {
    fn from(value: EventProperty) -> Self {
        value as FMOD_STUDIO_EVENT_PROPERTY
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemCallbackKind<U: UserdataTypes> {
    Preupdate,
    Postupdate,
    BankUnload(Bank<U>),
    LiveupdateConnected,
    LiveupdateDisconnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<FMOD_STUDIO_INSTANCETYPE> for InstanceType {
    fn from(value: FMOD_STUDIO_INSTANCETYPE) -> Self {
        match value {
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_NONE => InstanceType::None,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_SYSTEM => InstanceType::System,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION => {
                InstanceType::EventDescription
            }
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE => {
                InstanceType::EventInstance
            }
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE => {
                InstanceType::ParameterInstance
            }
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BUS => InstanceType::Bus,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_VCA => InstanceType::Vca,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BANK => InstanceType::Bank,
            FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY => {
                InstanceType::CommandReplay
            }
            _ => panic!("invalid instance type"),
        }
    }
}

impl From<InstanceType> for FMOD_STUDIO_INSTANCETYPE {
    fn from(value: InstanceType) -> Self {
        match value {
            InstanceType::None => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_NONE,
            InstanceType::System => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_SYSTEM,
            InstanceType::EventDescription => {
                FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION
            }
            InstanceType::EventInstance => {
                FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE
            }
            InstanceType::ParameterInstance => {
                FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE
            }
            InstanceType::Bus => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BUS,
            InstanceType::Vca => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_VCA,
            InstanceType::Bank => FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_BANK,
            InstanceType::CommandReplay => {
                FMOD_STUDIO_INSTANCETYPE_FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY
            }
        }
    }
}

pub enum EventCallbackKind<U: UserdataTypes> {
    Created,
    Destroyed,
    Starting,
    Started,
    Restarted,
    Stopped,
    StartFailed,
    CreateProgrammerSound(ProgrammerSoundProperties),
    DestroyProgrammerSound(ProgrammerSoundProperties),
    PluginCreated(PluginInstanceProperties),
    PluginDestroyed(PluginInstanceProperties),
    TimelineMarker(TimelineMarkerProperties),
    TimelineBeat(TimelineBeatProperties),
    SoundPlayed(Sound),
    SoundStopped(Sound),
    RealToVirtual,
    VirtualToReal,
    StartEventCommand(EventInstance<U>),
    NestedTimelineBeat(TimelineNestedBeatProperties),
}
