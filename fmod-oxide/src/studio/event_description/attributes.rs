// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_float;

use fmod_sys::*;

use crate::studio::EventDescription;

#[cfg(doc)]
use crate::studio::EventInstance;

impl EventDescription {
    /// Retrieves the event's 3D status.
    ///
    /// An event is considered 3D if any of these conditions are met:
    ///  - The event has a Spatializer, 3D Object Spatializer, or a 3rd party spatializer on its master track.
    ///  - The event contains an automatic parameter that depends on the event's 3D attributes:
    ///    - Distance
    ///    - Event Cone Angle
    ///    - Event Orientation
    ///    - Direction
    ///    - Elevation
    ///    - Speed
    ///    - Speed (Absolute)
    ///  - The event contains any nested events which are 3D.
    ///
    /// Note: If the event contains nested events built to separate banks using versions of FMOD Studio prior to 2.00.10 and those banks have not been loaded then this function may fail to correctly determine the event's 3D status.
    pub fn is_3d(&self) -> Result<bool> {
        let mut is_3d = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_EventDescription_Is3D(self.inner.as_ptr(), &mut is_3d).to_result()?;
        }
        Ok(is_3d.into())
    }

    /// Retrieves the event's doppler status.
    ///
    /// Note: If the event was built to a bank using versions of FMOD Studio prior to 2.01.09, then this function will return false regardless of the event's doppler state.
    pub fn is_doppler_enabled(&self) -> Result<bool> {
        let mut is_doppler = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_EventDescription_IsDopplerEnabled(self.inner.as_ptr(), &mut is_doppler)
                .to_result()?;
        }
        Ok(is_doppler.into())
    }

    /// Retrieves the event's oneshot status.
    ///
    /// An event is considered oneshot if it is guaranteed to terminate without intervention in bounded time after being started.
    /// Instances of such events can be played in a fire-and-forget fashion by calling [`EventInstance::start`] immediately followed by [`EventInstance::release`].
    ///
    /// Note: If the event contains nested events built to separate banks and those banks have not been loaded then this function may fail to correctly determine the event's oneshot status.
    pub fn is_oneshot(&self) -> Result<bool> {
        let mut is_oneshot = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_EventDescription_IsOneshot(self.inner.as_ptr(), &mut is_oneshot)
                .to_result()?;
        }
        Ok(is_oneshot.into())
    }

    /// Retrieves the event's snapshot status.
    pub fn is_snapshot(&self) -> Result<bool> {
        let mut is_snapshot = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_EventDescription_IsSnapshot(self.inner.as_ptr(), &mut is_snapshot)
                .to_result()?;
        }
        Ok(is_snapshot.into())
    }

    /// Retrieves the event's stream status.
    ///
    /// Note: If the event contains nested events built to separate banks and those banks have not been loaded then this function may fail to correctly determine the event's stream status.
    pub fn is_stream(&self) -> Result<bool> {
        let mut is_stream = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_EventDescription_IsStream(self.inner.as_ptr(), &mut is_stream)
                .to_result()?;
        }
        Ok(is_stream.into())
    }

    /// Retrieves whether the event has any sustain points.
    pub fn has_sustain_point(&self) -> Result<bool> {
        let mut sustain_point = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_EventDescription_HasSustainPoint(self.inner.as_ptr(), &mut sustain_point)
                .to_result()?;
        }
        Ok(sustain_point.into())
    }

    /// Retrieves the minimum and maximum distances for 3D attenuation.
    pub fn get_min_max_distance(&self) -> Result<(c_float, c_float)> {
        let mut min = 0.0;
        let mut max = 0.0;
        unsafe {
            FMOD_Studio_EventDescription_GetMinMaxDistance(self.inner.as_ptr(), &mut min, &mut max)
                .to_result()?;
        }
        Ok((min, max))
    }

    /// Retrieves the sound size for 3D panning.
    ///
    /// Retrieves the largest Sound Size value of all Spatializers and 3D Object Spatializers on the event's master track. Returns zero if there are no Spatializers or 3D Object Spatializers.
    pub fn get_sound_size(&self) -> Result<c_float> {
        let mut size = 0.0;
        unsafe {
            FMOD_Studio_EventDescription_GetSoundSize(self.inner.as_ptr(), &mut size)
                .to_result()?;
        }
        Ok(size)
    }
}
