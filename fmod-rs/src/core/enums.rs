// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

#[repr(u32)]
pub enum SpeakerMode {
    Default = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_DEFAULT,
    Raw = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_RAW,
    Mono = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_MONO,
    Stereo = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_STEREO,
    Quad = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_QUAD,
    Surround = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_SURROUND,
    FivePointOne = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_5POINT1,
    SevenPointOne = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_7POINT1,
    SevenPointOneFour = FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_7POINT1POINT4,
}

impl From<FMOD_SPEAKERMODE> for SpeakerMode {
    fn from(value: FMOD_SPEAKERMODE) -> Self {
        match value {
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_DEFAULT => SpeakerMode::Default,
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_RAW => SpeakerMode::Raw,
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_MONO => SpeakerMode::Mono,
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_STEREO => SpeakerMode::Stereo,
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_QUAD => SpeakerMode::Quad,
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_SURROUND => SpeakerMode::Surround,
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_5POINT1 => SpeakerMode::FivePointOne,
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_7POINT1 => SpeakerMode::SevenPointOne,
            FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_7POINT1POINT4 => SpeakerMode::SevenPointOneFour,
            // TODO: is this the right way to handle invalid states?
            v => panic!("invalid loading state {v}"),
        }
    }
}

impl From<SpeakerMode> for FMOD_SPEAKERMODE {
    fn from(value: SpeakerMode) -> Self {
        value as FMOD_SPEAKERMODE
    }
}
