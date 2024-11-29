// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_float, c_int};

use fmod_sys::*;
use lanyard::Utf8CStr;

use crate::studio::{EventInstance, ParameterID};

#[cfg(doc)]
use crate::studio::{ParameterKind, PlaybackState};

impl EventInstance {
    /// Sets a parameter value by name.
    ///
    /// The value will be set instantly regardless of `ignore_seek_speed` when the Event playback state is [`PlaybackState::Stopped`].
    ///
    /// If the specified parameter is read only, is an automatic parameter or is not of type [`ParameterKind::GameControlled`] then [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`] is returned.
    ///
    /// If the event has no parameter matching name then [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    pub fn set_parameter_by_name(
        &self,
        name: &Utf8CStr,
        value: c_float,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetParameterByName(
                self.inner.as_ptr(),
                name.as_ptr(),
                value,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Sets a parameter value by name, looking up the value label.
    ///
    /// The label will be set instantly regardless of `ignore_seek_speed` when the Event playback state is [`PlaybackState::Stopped`].
    ///
    /// If the specified parameter is read only, is an automatic parameter or is not of type [`ParameterKind::GameControlled`] then [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`] is returned.
    ///
    /// If the event has no parameter matching name then [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    ///
    /// If the specified label is not found, [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned. This lookup is case sensitive.
    pub fn set_parameter_by_name_with_label(
        &self,
        name: &Utf8CStr,
        label: &Utf8CStr,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetParameterByNameWithLabel(
                self.inner.as_ptr(),
                name.as_ptr(),
                label.as_ptr(),
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Retrieves a parameter value by name.
    ///
    /// Automatic parameters always return value as 0 since they can never have their value set from the public API.
    ///
    /// The second returned tuple field is the final value of the parameter after applying adjustments due to automation, modulation, seek speed, and parameter velocity to value.
    /// This is calculated asynchronously when the Studio system updates.
    pub fn get_parameter_by_name(&self, name: &Utf8CStr) -> Result<(c_float, c_float)> {
        let mut value = 0.0;
        let mut final_value = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetParameterByName(
                self.inner.as_ptr(),
                name.as_ptr(),
                &mut value,
                &mut final_value,
            )
            .to_result()?;
        }
        Ok((value, final_value))
    }

    /// Sets a parameter value by unique identifier.
    ///
    /// The value will be set instantly regardless of `ignore_seek_speed` when the Event playback state is [`PlaybackState::Stopped`].
    ///
    /// If the specified parameter is read only, is an automatic parameter or is not of type [`ParameterKind::GameControlled`] then [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`] is returned.
    pub fn set_parameter_by_id(
        &self,
        id: ParameterID,
        value: c_float,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetParameterByID(
                self.inner.as_ptr(),
                id.into(),
                value,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Sets a parameter value by unique identifier, looking up the value label.
    ///
    /// The label will be set instantly regardless of `ignore_seek_speed` when the Event playback state is [`PlaybackState::Stopped`].
    ///
    /// If the specified parameter is read only, is an automatic parameter or is not of type [`ParameterKind::GameControlled`] then [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`] is returned.
    ///
    /// If the specified label is not found, [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned. This lookup is case sensitive.
    pub fn set_parameter_by_id_with_label(
        &self,
        id: ParameterID,
        label: &Utf8CStr,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetParameterByIDWithLabel(
                self.inner.as_ptr(),
                id.into(),
                label.as_ptr(),
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Retrieves a parameter value by unique identifier.
    ///
    /// Automatic parameters always return value as 0 since they can never have their value set from the public API.
    ///
    /// The second returned tuple field is the final value of the parameter after applying adjustments due to automation, modulation, seek speed, and parameter velocity to value.
    /// This is calculated asynchronously when the Studio system updates.
    pub fn get_parameter_by_id(&self, id: ParameterID) -> Result<(c_float, c_float)> {
        let mut value = 0.0;
        let mut final_value = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetParameterByID(
                self.inner.as_ptr(),
                id.into(),
                &mut value,
                &mut final_value,
            )
            .to_result()?;
        }
        Ok((value, final_value))
    }

    /// Sets multiple parameter values by unique identifier.
    ///
    /// All values will be set instantly regardless of `ingore_seek_speed` when the Event playback state is [`PlaybackState::Stopped`].
    ///
    /// If any ID is set to all zeroes then the corresponding value will be ignored.
    // TODO iterator version?
    pub fn set_parameters_by_ids(
        &self,
        ids: &[ParameterID], // TODO fmod says that the size of this must range from 1-32. do we need to enforce this?
        values: &mut [c_float], // TODO is this &mut correct? does fmod perform any writes?
        ignore_seek_speed: bool,
    ) -> Result<()> {
        // TODO don't panic, return result
        assert_eq!(ids.len(), values.len());

        unsafe {
            FMOD_Studio_EventInstance_SetParametersByIDs(
                self.inner.as_ptr(),
                ids.as_ptr().cast(),
                values.as_mut_ptr(),
                ids.len() as c_int,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }
}
