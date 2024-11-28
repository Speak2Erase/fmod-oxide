// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int};

use crate::studio::{EventInstance, EventProperty};

#[cfg(doc)]
use crate::studio::EventDescription;

impl EventInstance {
    /// Sets the pitch multiplier.
    ///
    /// The pitch multiplier is used to modulate the event instance's pitch.
    /// The pitch multiplier can be set to any value greater than or equal to zero but the final combined pitch is clamped to the range [0, 100] before being applied.
    pub fn set_pitch(&self, pitch: c_float) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetPitch(self.inner, pitch).to_result() }
    }

    /// Retrieves the pitch multiplier.
    ///
    /// The final combined value returned in second tuple field combines the pitch set using [`EventInstance::set_pitch`] with the result of any automation or modulation.
    /// The final combined pitch is calculated asynchronously when the Studio system updates.
    pub fn get_pitch(&self) -> Result<(c_float, c_float)> {
        let mut pitch = 0.0;
        let mut final_pitch = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetPitch(self.inner, &mut pitch, &mut final_pitch)
                .to_result()?;
        }
        Ok((pitch, final_pitch))
    }

    /// Sets the value of a built-in property.
    ///
    /// This will override the value set in Studio. Using the default [`EventProperty`] value will revert back to the default values set in Studio.
    ///
    /// An FMOD spatializer or object spatializer may override the values set for [`EventProperty::MinimumDistance`] and [`EventProperty::MaximumDistance`]].
    pub fn set_property(&self, property: EventProperty, value: c_float) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetProperty(self.inner, property.into(), value).to_result()
        }
    }

    /// Retrieves the value of a built-in property.
    ///
    /// A default [`EventProperty`] value means that the Instance is using the value set in Studio and it has not been overridden using [`EventInstance::set_property`].
    /// Access the default property values through the [`EventDescription`].
    pub fn get_property(&self, property: EventProperty) -> Result<c_float> {
        let mut value = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetProperty(self.inner, property.into(), &mut value)
                .to_result()?;
        }
        Ok(value)
    }

    /// Sets the timeline cursor position.
    pub fn set_timeline_position(&self, position: c_int) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetTimelinePosition(self.inner, position).to_result() }
    }

    /// Retrieves the timeline cursor position.
    pub fn get_timeline_position(&self) -> Result<c_int> {
        let mut position = 0;
        unsafe {
            FMOD_Studio_EventInstance_GetTimelinePosition(self.inner, &mut position).to_result()?;
        }
        Ok(position)
    }

    /// Sets the volume level.
    ///
    /// This volume is applied as a scaling factor for the event volume. It does not override the volume level set in FMOD Studio, nor any internal volume automation or modulation.
    pub fn set_volume(&self, volume: c_float) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetVolume(self.inner, volume).to_result() }
    }

    /// Retrieves the volume level.
    ///
    /// The value returned in the second tuple field combines the volume set using the public API with the result of any automation or modulation.
    /// The final combined volume is calculated asynchronously when the Studio system updates.
    pub fn get_volume(&self) -> Result<(c_float, c_float)> {
        let mut volume = 0.0;
        let mut final_volume = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetVolume(self.inner, &mut volume, &mut final_volume)
                .to_result()?;
        }
        Ok((volume, final_volume))
    }

    /// Retrieves the virtualization state.
    ///
    /// This function checks whether an event instance has been virtualized due to the polyphony limit being exceeded.
    pub fn is_virtual(&self) -> Result<bool> {
        let mut is_virtual = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Studio_EventInstance_IsVirtual(self.inner, &mut is_virtual).to_result()?;
        }
        Ok(is_virtual.into())
    }
}
