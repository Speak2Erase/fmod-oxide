use std::ffi::c_int;

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
use fmod_sys::*;

mod bank;
pub use bank::*;

mod bus;
pub use bus::*;

mod system;
pub use system::*;

mod event_description;
pub use event_description::*;

mod vca;
pub use vca::*;

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
pub struct MemoryUsage {
    pub exclusive: c_int,
    pub inclusive: c_int,
    pub sample_data: c_int,
}

impl From<FMOD_STUDIO_MEMORY_USAGE> for MemoryUsage {
    fn from(value: FMOD_STUDIO_MEMORY_USAGE) -> Self {
        MemoryUsage {
            exclusive: value.exclusive,
            inclusive: value.inclusive,
            sample_data: value.sampledata,
        }
    }
}

impl From<MemoryUsage> for FMOD_STUDIO_MEMORY_USAGE {
    fn from(value: MemoryUsage) -> Self {
        FMOD_STUDIO_MEMORY_USAGE {
            exclusive: value.exclusive,
            inclusive: value.inclusive,
            sampledata: value.sample_data,
        }
    }
}
