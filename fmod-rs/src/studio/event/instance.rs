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

use std::{
    ffi::{c_float, c_int, c_uint},
    marker::PhantomData,
    mem::MaybeUninit,
    sync::Arc,
};

use fmod_sys::*;
use lanyard::Utf8CStr;

use super::{internal_event_callback, EventCallback, InternalUserdata};
use crate::{core::ChannelGroup, Attributes3D};
use crate::{
    studio::{
        EventCallbackMask, EventDescription, EventProperty, MemoryUsage, ParameterID,
        PlaybackState, StopMode,
    },
    UserdataTypes,
};

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct EventInstance<U: UserdataTypes = ()> {
    pub(crate) inner: *mut FMOD_STUDIO_EVENTINSTANCE,
    _phantom: PhantomData<U>,
}

unsafe impl<U: UserdataTypes> Send for EventInstance<U> {}
unsafe impl<U: UserdataTypes> Sync for EventInstance<U> {}
impl<U: UserdataTypes> Clone for EventInstance<U> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<U: UserdataTypes> Copy for EventInstance<U> {}

impl<U: UserdataTypes> EventInstance<U> {
    /// Create a System instance from its FFI equivalent.
    ///
    /// # Safety
    /// This operation is unsafe because it's possible that the [`FMOD_STUDIO_EVENTINSTANCE`] will not have the right userdata type.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_EVENTINSTANCE) -> Self {
        EventInstance {
            inner: value,
            _phantom: PhantomData,
        }
    }
}

impl<U: UserdataTypes> From<EventInstance<U>> for *mut FMOD_STUDIO_EVENTINSTANCE {
    fn from(value: EventInstance<U>) -> Self {
        value.inner
    }
}

impl<U: UserdataTypes> EventInstance<U> {
    /// Starts playback.
    ///
    ///If the instance was already playing then calling this function will restart the event.
    ///
    /// Generally it is a best practice to call [`EventInstance::release`] on event instances immediately after starting them,
    /// unless you want to play the event instance multiple times or explicitly stop it and start it again later.
    pub fn start(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_Start(self.inner).to_result() }
    }

    /// Stops playback.
    pub fn stop(&self, mode: StopMode) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_Stop(self.inner, mode.into()).to_result() }
    }

    /// Retrieves the playback state.
    ///
    /// You can poll this function to track the playback state of an event instance.
    ///
    /// If the instance is invalid, then the state will be set to [`PlaybackState::Stopped`].
    pub fn get_playback_state(&self) -> Result<PlaybackState> {
        let mut state = 0;
        unsafe {
            FMOD_Studio_EventInstance_GetPlaybackState(self.inner, &mut state).to_result()?;
        }
        Ok(state.into())
    }

    /// Sets the pause state.
    ///
    /// This function allows pausing/unpausing of an event instance.
    pub fn set_paused(&self, paused: bool) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetPaused(self.inner, paused.into()).to_result() }
    }

    /// Retrieves the paused state.
    pub fn get_paused(&self) -> Result<bool> {
        let mut paused = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_EventInstance_GetPaused(self.inner, &mut paused).to_result()?;
        }
        Ok(paused.into())
    }

    /// Allow an event to continue past a sustain point.
    ///
    /// Multiple sustain points may be bypassed ahead of time and the key off count will be decremented each time the timeline cursor passes a sustain point.
    ///
    /// This function returns [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] if the event has no sustain points.
    pub fn key_off(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_KeyOff(self.inner).to_result() }
    }

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
        let mut is_virtual = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_EventInstance_IsVirtual(self.inner, &mut is_virtual).to_result()?;
        }
        Ok(is_virtual.into())
    }

    /// Sets the 3D attributes.
    ///
    /// An event's 3D attributes specify its position, velocity and orientation.
    /// The 3D attributes are used to calculate 3D panning, doppler and the values of automatic distance and angle parameters.
    pub fn set_3d_attributes(&self, attributes: Attributes3D) -> Result<()> {
        let mut attributes = attributes.into();
        unsafe {
            // FIXME this is not supposed to take an &mut
            FMOD_Studio_EventInstance_Set3DAttributes(self.inner, &mut attributes).to_result()
        }
    }

    /// Retrieves the 3D attributes.
    pub fn get_3d_attributes(&self) -> Result<Attributes3D> {
        let mut attributes = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventInstance_Get3DAttributes(self.inner, attributes.as_mut_ptr())
                .to_result()?;

            let attributes = attributes.assume_init().into();
            Ok(attributes)
        }
    }

    /// Sets the listener mask.
    ///
    /// The listener mask controls which listeners are considered when calculating 3D panning and the values of listener relative automatic parameters.
    ///
    /// To create the mask you must perform bitwise OR and shift operations, the basic form is 1 << `listener_index` or'd together with other required listener indices.
    /// For example to create a mask for listener index `0` and `2` the calculation would be `mask = (1 << 0) | (1 << 2)`, to include all listeners use the default mask of `0xFFFFFFFF`.
    pub fn set_listener_mask(&self, mask: c_uint) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetListenerMask(self.inner, mask).to_result() }
    }

    /// Retrieves the listener mask.
    pub fn get_listener_mask(&self) -> Result<c_uint> {
        let mut mask = 0;
        unsafe {
            FMOD_Studio_EventInstance_GetListenerMask(self.inner, &mut mask).to_result()?;
        }
        Ok(mask)
    }

    /// Retrieves the minimum and maximum distances for 3D attenuation.
    pub fn get_min_max_distance(&self) -> Result<(c_float, c_float)> {
        let mut min = 0.0;
        let mut max = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetMinMaxDistance(self.inner, &mut min, &mut max)
                .to_result()?;
        }
        Ok((min, max))
    }

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
                self.inner,
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
                self.inner,
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
                self.inner,
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
                self.inner,
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
                self.inner,
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
                self.inner,
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
                self.inner,
                ids.as_ptr().cast(),
                values.as_mut_ptr(),
                ids.len() as c_int,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Retrieves the core [`ChannelGroup`].
    ///
    /// Until the event instance has been fully created this function will return [`FMOD_RESULT::FMOD_ERR_STUDIO_NOT_LOADED`].
    pub fn get_channel_group(&self) -> Result<ChannelGroup> {
        let mut channel_group = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetChannelGroup(self.inner, &mut channel_group)
                .to_result()?;
        }
        Ok(channel_group.into())
    }

    /// Sets the core reverb send level.
    ///          
    /// This function controls the send level for the signal from the event instance to a core reverb instance.
    pub fn set_reverb_level(&self, index: c_int, level: c_float) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetReverbLevel(self.inner, index, level).to_result() }
    }

    /// Retrieves the core reverb send level.
    pub fn get_reverb_level(&self, index: c_int) -> Result<c_float> {
        let mut level = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetReverbLevel(self.inner, index, &mut level).to_result()?;
        }
        Ok(level)
    }

    /// Retrieves the event CPU usage data.
    ///
    /// [`crate::InitFlags::PROFILE_ENABLE`] with [`crate::SystemBuilder::build`] is required to call this function.
    // TODO fmod core docs
    pub fn get_cpu_usage(&self) -> Result<(c_uint, c_uint)> {
        let mut exclusive = 0;
        let mut inclusive = 0;
        unsafe {
            FMOD_Studio_EventInstance_GetCPUUsage(self.inner, &mut exclusive, &mut inclusive)
                .to_result()?;
        }
        Ok((exclusive, inclusive))
    }

    /// Retrieves memory usage statistics.
    ///
    /// Memory usage statistics are only available in logging builds, in release builds the return value will contain zero for all values this function.
    pub fn get_memory_usage(&self) -> Result<MemoryUsage> {
        let mut memory_usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventInstance_GetMemoryUsage(self.inner, memory_usage.as_mut_ptr())
                .to_result()?;

            let memory_usage = memory_usage.assume_init().into();
            Ok(memory_usage)
        }
    }

    /// Sets the user callback.
    ///
    /// See the event callbacks section in the FMOD docs for more information about when callbacks occur.
    pub fn set_callback(
        &self,
        callback: Option<Arc<dyn EventCallback<U>>>,
        mask: EventCallbackMask,
    ) -> Result<()> {
        // Always enable destroyed to deallocate any userdata attached to events
        let raw_mask = (mask | EventCallbackMask::DESTROYED).into();

        unsafe {
            let userdata = &mut *self.get_or_insert_userdata()?;
            userdata.callback_mask = mask;

            if let Some(f) = callback {
                userdata.callback = Some(f);
                self.set_callback_raw(Some(internal_event_callback::<U>), raw_mask)
            } else {
                userdata.callback = None;
                self.set_callback_raw(None, raw_mask)
            }
        }
    }

    /// Sets the event instance user data.
    ///
    /// This function allows arbitrary user data to be attached to this object.
    /// The provided data may be shared/accessed from multiple threads, and so must implement Send + Sync 'static.
    pub fn set_user_data(&self, data: Option<Arc<U::Event>>) -> Result<()> {
        unsafe {
            let userdata: &mut InternalUserdata<U> = &mut *self.get_or_insert_userdata()?;
            userdata.userdata = data;
        }

        Ok(())
    }

    /// Retrieves the event instance user data.
    ///
    /// This function allows arbitrary user data to be retrieved from this object.
    pub fn get_user_data(&self) -> Result<Option<Arc<U::Event>>> {
        unsafe {
            let userdata = self.get_raw_user_data()?.cast::<InternalUserdata<U>>();

            if userdata.is_null() {
                return Ok(None);
            }

            // userdata should ALWAYS be InternalUserdata
            let userdata = &mut *userdata;
            let userdata = userdata.userdata.clone();
            Ok(userdata)
        }
    }

    /// Retrieves the event instance raw userdata.
    ///
    /// This function is safe because accessing the pointer is unsafe.
    pub fn get_raw_user_data(&self) -> Result<*mut std::ffi::c_void> {
        unsafe {
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_EventInstance_GetUserData(self.inner, &mut userdata).to_result()?;
            Ok(userdata)
        }
    }

    /// Sets the event instance raw userdata.
    ///
    /// This function is UNSAFE (more unsafe than most in this crate!) because this crate makes assumptions about the type of userdata.
    ///
    /// # Safety
    /// When calling this function with *any* pointer not recieved from a prior call to [`Self::get_raw_user_data`] you must call [`Self::set_callback_raw`]!
    /// Calbacks in this crate always assume that the userdata pointer always points to an internal struct.
    pub unsafe fn set_raw_userdata(&self, userdata: *mut std::ffi::c_void) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetUserData(self.inner, userdata).to_result() }
    }

    /// Sets the raw callback.
    ///
    /// Unlike [`Self::set_raw_userdata`], this crate makes no assumptions about callbacks.
    /// It expects them to be set (for memory management reasons) but setting it to a raw callback is ok.
    ///
    /// It's worth noting that this crate sets userdata to an internal structure by default. You will generally want to use [`Self::set_callback_raw`].
    pub fn set_callback_raw(&self, callback: FMOD_STUDIO_EVENT_CALLBACK, mask: u32) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetCallback(self.inner, callback, mask).to_result() }
    }

    unsafe fn get_or_insert_userdata(&self) -> Result<*mut InternalUserdata<U>> {
        unsafe {
            let mut userdata = self.get_raw_user_data()?.cast::<InternalUserdata<U>>();

            // create and set the userdata if we haven't already
            if userdata.is_null() {
                let boxed_userdata = Box::new(InternalUserdata::instance());
                userdata = Box::into_raw(boxed_userdata);

                self.set_raw_userdata(userdata.cast())?;
            } else {
                // if it is set, check that it's not from an event instance
                let desc_userdata = &*userdata;
                if !desc_userdata.is_from_event_instance {
                    // create new userdata and copy over anything set from the original userdata
                    let boxed_userdata = Box::new(desc_userdata.as_instance());
                    userdata = Box::into_raw(boxed_userdata);

                    self.set_raw_userdata(userdata.cast())?;
                }
            }

            Ok(userdata)
        }
    }

    /// Retrieves the event description.
    pub fn get_description(&self) -> Result<EventDescription<U>> {
        let mut description = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetDescription(self.inner, &mut description).to_result()?;
            Ok(EventDescription::from_ffi(description))
        }
    }

    /// Marks the event instance for release.
    ///
    /// This function marks the event instance to be released.
    /// Event instances marked for release are destroyed by the asynchronous update when they are in the stopped state ([`PlaybackState::Stopped`]).
    ///
    /// Generally it is a best practice to release event instances immediately after calling [`EventInstance::start`],
    /// unless you want to play the event instance multiple times or explicitly stop it and start it again later.
    /// It is possible to interact with the instance after falling [`EventInstance::release`], however if the sound has stopped [`FMOD_RESULT::FMOD_ERR_INVALID_HANDLE`] will be returned.
    pub fn release(self) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_Release(self.inner).to_result() }
    }

    /// Checks that the [`EventInstance`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_EventInstance_IsValid(self.inner).into() }
    }
}
