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
    any::Any,
    ffi::{c_float, c_int, CStr},
    mem::MaybeUninit,
    os::raw::c_void,
    sync::Arc,
};

use fmod_sys::*;

use crate::{core::Sound, Guid};

use super::{
    EventCallbackKind, EventCallbackMask, EventInstance, LoadingState, ParameterDescription,
    ParameterID, PluginInstanceProperties, ProgrammerSoundProperties, TimelineMarkerProperties,
    UserProperty,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct EventDescription {
    pub(crate) inner: *mut FMOD_STUDIO_EVENTDESCRIPTION,
}

pub(crate) struct InternalUserdata {
    // this is an arc in case someone releases the event instance or description while holding onto a reference to the userdata
    // also this is logically and actually shared because event instances inherit their description's userdata and callbacks unless modified
    pub(crate) userdata: Option<Userdata>,
    pub(crate) callback: Option<CallbackFn>,
    // used to ensure we don't misfire callbacks (we always subscribe to releasing events to ensure userdata is freed)
    pub(crate) callback_mask: EventCallbackMask,
    // used to prevent use after frees so events don't accidentally free their description's userdata
    pub(crate) is_from_event_instance: bool,
}

// hilariously long type signature because clippy
pub(crate) type CallbackFn =
    Arc<dyn Fn(EventCallbackKind, EventInstance) -> Result<()> + Send + Sync>;
pub(crate) type Userdata = Arc<dyn Any + Send + Sync>;

pub(crate) unsafe extern "C" fn internal_event_callback(
    kind: FMOD_STUDIO_EVENT_CALLBACK_TYPE,
    event: *mut FMOD_STUDIO_EVENTINSTANCE,
    parameters: *mut c_void,
) -> FMOD_RESULT {
    // FIXME: handle unwinding panics

    let mut userdata = std::ptr::null_mut();
    let error = unsafe { FMOD_Studio_EventInstance_GetUserData(event, &mut userdata).to_error() };

    if let Some(error) = error {
        eprintln!("error grabbing the event userdata: {error}");
    }

    #[cfg(debug_assertions)]
    if userdata.is_null() {
        eprintln!("event instance userdata is null. aborting");
        std::process::abort();
    }

    // userdata should always be not null if this function is called, and it should be a valid reference to InternalUserdata
    let userdata = unsafe { &mut *userdata.cast::<InternalUserdata>() };

    let mut result = FMOD_RESULT::FMOD_OK;
    if let Some(callback) = &userdata.callback {
        if userdata.callback_mask.contains(kind.into()) {
            let event_instance = event.into();

            let kind = match kind {
                FMOD_STUDIO_EVENT_CALLBACK_CREATED => EventCallbackKind::Created,
                FMOD_STUDIO_EVENT_CALLBACK_DESTROYED => EventCallbackKind::Destroyed,
                FMOD_STUDIO_EVENT_CALLBACK_STARTING => EventCallbackKind::Starting,
                FMOD_STUDIO_EVENT_CALLBACK_STARTED => EventCallbackKind::Started,
                FMOD_STUDIO_EVENT_CALLBACK_RESTARTED => EventCallbackKind::Restarted,
                FMOD_STUDIO_EVENT_CALLBACK_STOPPED => EventCallbackKind::Stopped,
                FMOD_STUDIO_EVENT_CALLBACK_START_FAILED => EventCallbackKind::StartFailed,
                FMOD_STUDIO_EVENT_CALLBACK_CREATE_PROGRAMMER_SOUND => unsafe {
                    let props = &mut *parameters.cast::<FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES>();
                    EventCallbackKind::CreateProgrammerSound(ProgrammerSoundProperties {
                        name: CStr::from_ptr(props.name),
                        // the casts are safe because sound and *mut FMOD_SOUND are identical
                        sound: &mut *std::ptr::addr_of_mut!(props.sound).cast::<Sound>(),
                        subsound_index: &mut props.subsoundIndex,
                    })
                },
                FMOD_STUDIO_EVENT_CALLBACK_DESTROY_PROGRAMMER_SOUND => {
                    unsafe {
                        let props =
                            &mut *parameters.cast::<FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES>();
                        EventCallbackKind::DestroyProgrammerSound(ProgrammerSoundProperties {
                            name: CStr::from_ptr(props.name),
                            // the casts are safe because sound and *mut FMOD_SOUND are identical
                            sound: &mut *std::ptr::addr_of_mut!(props.sound).cast::<Sound>(),
                            subsound_index: &mut props.subsoundIndex,
                        })
                    }
                }
                FMOD_STUDIO_EVENT_CALLBACK_PLUGIN_CREATED => unsafe {
                    let props = *parameters.cast::<FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES>();
                    EventCallbackKind::PluginCreated(PluginInstanceProperties::from_ffi(props))
                },
                FMOD_STUDIO_EVENT_CALLBACK_PLUGIN_DESTROYED => unsafe {
                    let props = *parameters.cast::<FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES>();
                    EventCallbackKind::PluginDestroyed(PluginInstanceProperties::from_ffi(props))
                },
                FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_MARKER => unsafe {
                    let marker = *parameters.cast::<FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES>();
                    EventCallbackKind::TimelineMarker(TimelineMarkerProperties::from_ffi(marker))
                },
                FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_BEAT => unsafe {
                    let beat = *parameters.cast::<FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES>();
                    EventCallbackKind::TimelineBeat(beat.into())
                },
                FMOD_STUDIO_EVENT_CALLBACK_SOUND_PLAYED => {
                    EventCallbackKind::SoundPlayed(parameters.cast::<FMOD_SOUND>().into())
                }
                FMOD_STUDIO_EVENT_CALLBACK_SOUND_STOPPED => {
                    EventCallbackKind::SoundStopped(parameters.cast::<FMOD_SOUND>().into())
                }
                FMOD_STUDIO_EVENT_CALLBACK_REAL_TO_VIRTUAL => EventCallbackKind::RealToVirtual,
                FMOD_STUDIO_EVENT_CALLBACK_VIRTUAL_TO_REAL => EventCallbackKind::VirtualToReal,
                FMOD_STUDIO_EVENT_CALLBACK_START_EVENT_COMMAND => {
                    EventCallbackKind::StartEventCommand(
                        parameters.cast::<FMOD_STUDIO_EVENTINSTANCE>().into(),
                    )
                }
                FMOD_STUDIO_EVENT_CALLBACK_NESTED_TIMELINE_BEAT => unsafe {
                    let beat = *parameters.cast::<FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES>();
                    EventCallbackKind::NestedTimelineBeat(beat.into())
                },
                _ => {
                    eprintln!("wrong event callback type {kind}, aborting");
                    std::process::abort()
                }
            };

            result = callback(kind, event_instance).into();
        }
    }

    if kind == FMOD_STUDIO_EVENT_CALLBACK_DESTROYED && userdata.is_from_event_instance {
        let userdata = unsafe { Box::from_raw(userdata) };
        drop(userdata);

        // deallocate the userdata, and set it to null. this shouldn't be necessary but just in case
        let error = unsafe {
            FMOD_Studio_EventInstance_SetUserData(event, std::ptr::null_mut()).to_error()
        };
        if let Some(error) = error {
            eprintln!("error setting the event userdata to null: {error}");
        }
    }

    result
}

unsafe impl Send for EventDescription {}
unsafe impl Sync for EventDescription {}

impl From<*mut FMOD_STUDIO_EVENTDESCRIPTION> for EventDescription {
    fn from(value: *mut FMOD_STUDIO_EVENTDESCRIPTION) -> Self {
        EventDescription { inner: value }
    }
}

impl From<EventDescription> for *mut FMOD_STUDIO_EVENTDESCRIPTION {
    fn from(value: EventDescription) -> Self {
        value.inner
    }
}

impl EventDescription {
    /// Creates a playable instance.
    ///
    /// When an event instance is created, any required non-streaming sample data is loaded asynchronously.
    ///
    /// Use [`EventDescription::get_sample_loading_state`] to check the loading status.
    ///
    /// Sample data can be loaded ahead of time with [`EventDescription::load_sample_data`] or [`super::Bank::load_sample_data`]. See Sample Data Loading for more information.
    pub fn create_instance(&self) -> Result<EventInstance> {
        let mut instance = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventDescription_CreateInstance(self.inner, &mut instance).to_result()?;
        }
        Ok(instance.into())
    }

    /// Retrieves the number of instances.
    pub fn instance_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetInstanceCount(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    pub fn get_instance_list(&self) -> Result<Vec<EventInstance>> {
        let expected_count = self.instance_count()?;
        let mut count = 0;
        let mut list = vec![
            EventInstance {
                inner: std::ptr::null_mut()
            };
            expected_count as usize
        ];

        unsafe {
            FMOD_Studio_EventDescription_GetInstanceList(
                self.inner,
                // eventinstance is repr transparent and has the same layout as *mut FMOD_STUDIO_EVENTINSTANCE, so this cast is ok
                list.as_mut_ptr().cast::<*mut FMOD_STUDIO_EVENTINSTANCE>(),
                list.capacity() as c_int,
                &mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(list)
        }
    }

    /// Releases all instances.
    ///
    /// This function immediately stops and releases all instances of the event.
    pub fn release_all_instances(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventDescription_ReleaseAllInstances(self.inner).to_result() }
    }
}

impl EventDescription {
    /// Loads non-streaming sample data used by the event.
    ///
    /// This function will load all non-streaming sample data required by the event and any referenced events.
    ///
    /// Sample data is loaded asynchronously, [`EventDescription::get_sample_loading_state`] may be used to poll the loading state.
    pub fn load_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventDescription_LoadSampleData(self.inner).to_result() }
    }

    /// Unloads all non-streaming sample data.
    ///
    /// Sample data will not be unloaded until all instances of the event are released.
    pub fn unload_sample_data(&self) -> Result<()> {
        unsafe { FMOD_Studio_EventDescription_UnloadSampleData(self.inner).to_result() }
    }

    /// Retrieves the sample data loading state.
    ///
    /// If the event is invalid, then the returned state is [`LoadingState::Unloaded`] and this function returns [`FMOD_RESULT::FMOD_ERR_INVALID_HANDLE`].
    pub fn get_sample_loading_state(&self) -> (LoadingState, Option<Error>) {
        let mut state = 0;
        unsafe {
            let error = FMOD_Studio_EventDescription_GetSampleLoadingState(self.inner, &mut state)
                .to_error();
            let state = state.into();

            (state, error)
        }
    }
}

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
        let mut is_3d = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_EventDescription_Is3D(self.inner, &mut is_3d).to_result()?;
        }
        Ok(is_3d.into())
    }

    /// Retrieves the event's doppler status.
    ///
    /// Note: If the event was built to a bank using versions of FMOD Studio prior to 2.01.09, then this function will return false regardless of the event's doppler state.
    pub fn is_doppler_enabled(&self) -> Result<bool> {
        let mut is_doppler = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_EventDescription_IsDopplerEnabled(self.inner, &mut is_doppler)
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
        let mut is_oneshot = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_EventDescription_IsOneshot(self.inner, &mut is_oneshot).to_result()?;
        }
        Ok(is_oneshot.into())
    }

    /// Retrieves the event's snapshot status.
    pub fn is_snapshot(&self) -> Result<bool> {
        let mut is_snapshot = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_EventDescription_IsSnapshot(self.inner, &mut is_snapshot).to_result()?;
        }
        Ok(is_snapshot.into())
    }

    /// Retrieves the event's stream status.
    ///
    /// Note: If the event contains nested events built to separate banks and those banks have not been loaded then this function may fail to correctly determine the event's stream status.
    pub fn is_stream(&self) -> Result<bool> {
        let mut is_stream = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_EventDescription_IsStream(self.inner, &mut is_stream).to_result()?;
        }
        Ok(is_stream.into())
    }

    /// Retrieves whether the event has any sustain points.
    pub fn has_sustain_point(&self) -> Result<bool> {
        let mut sustain_point = FMOD_BOOL(0);
        unsafe {
            FMOD_Studio_EventDescription_HasSustainPoint(self.inner, &mut sustain_point)
                .to_result()?;
        }
        Ok(sustain_point.into())
    }

    /// Retrieves the minimum and maximum distances for 3D attenuation.
    pub fn get_min_max_distance(&self) -> Result<(c_float, c_float)> {
        let mut min = 0.0;
        let mut max = 0.0;
        unsafe {
            FMOD_Studio_EventDescription_GetMinMaxDistance(self.inner, &mut min, &mut max)
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
            FMOD_Studio_EventDescription_GetSoundSize(self.inner, &mut size).to_result()?;
        }
        Ok(size)
    }
}

impl EventDescription {
    /// Retrieves an event parameter description by name.
    pub fn get_parameter_description_by_name(&self, name: &CStr) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetParameterDescriptionByName(
                self.inner,
                name.as_ptr(),
                description.as_mut_ptr(),
            )
            .to_result()?;

            // FIXME lifetimes are incorrect and MUST be relaxed from 'static
            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves an event parameter description by id.
    pub fn get_parameter_description_by_id(&self, id: ParameterID) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetParameterDescriptionByID(
                self.inner,
                id.into(),
                description.as_mut_ptr(),
            )
            .to_result()?;

            // FIXME lifetimes are incorrect and MUST be relaxed from 'static
            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves an event parameter description by index.
    ///
    /// May be used in combination with [`EventDescription::parameter_description_count`] to enumerate event parameters.
    ///
    /// Note: The order of parameters is not necessarily the same as what is shown in the FMOD Studio event editor.
    pub fn get_parameter_description_by_index(&self, index: c_int) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetParameterDescriptionByIndex(
                self.inner,
                index,
                description.as_mut_ptr(),
            )
            .to_result()?;

            // FIXME lifetimes are incorrect and MUST be relaxed from 'static
            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves the number of parameters in the event.
    ///
    /// May be used in conjunction with [`EventDescription::get_parameter_description_by_index`] to enumerate event parameters.
    pub fn parameter_description_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetParameterDescriptionCount(self.inner, &mut count)
                .to_result()?;
        }
        Ok(count)
    }

    /// Retrieves an event parameter label by name or path.
    ///
    /// `name` can be the short name (such as `Wind`) or the full path (such as `parameter:/Ambience/Wind`).
    /// Path lookups will only succeed if the strings bank has been loaded.
    pub fn get_parameter_label_by_name(&self, name: &CStr, label_index: c_int) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_EventDescription_GetParameterLabelByName(
                self.inner,
                name.as_ptr(),
                label_index,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error.code != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_EventDescription_GetParameterLabelByName(
                self.inner,
                name.as_ptr(),
                label_index,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }

    /// Retrieves an event parameter label by ID.
    pub fn get_parameter_label_by_id(&self, id: ParameterID, label_index: c_int) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_EventDescription_GetParameterLabelByID(
                self.inner,
                id.into(),
                label_index,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error.code != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_EventDescription_GetParameterLabelByID(
                self.inner,
                id.into(),
                label_index,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }

    /// Retrieves an event parameter label by index.
    ///
    /// May be used in combination with [`EventDescription::parameter_description_count`] to enumerate event parameters.
    pub fn get_parameter_label_by_index(&self, index: c_int, label_index: c_int) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_EventDescription_GetParameterLabelByIndex(
                self.inner,
                index,
                label_index,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error.code != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_EventDescription_GetParameterLabelByIndex(
                self.inner,
                index,
                label_index,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }
}

impl EventDescription {
    /// Retrieves a user property by name.
    pub fn get_user_property(&self, name: &CStr) -> Result<UserProperty> {
        let mut property = MaybeUninit::uninit();
        unsafe {
            FMOD_Studio_EventDescription_GetUserProperty(
                self.inner,
                name.as_ptr(),
                property.as_mut_ptr(),
            )
            .to_result()?;

            // FIXME wrong lifetimes + wildly unsafe
            let property = UserProperty::from_ffi(property.assume_init());
            Ok(property)
        }
    }

    /// Retrieves a user property by index.
    ///
    /// May be used in combination with [`EventDescription::user_property_count`] to enumerate event user properties.
    pub fn get_user_property_by_index(&self, index: c_int) -> Result<UserProperty> {
        let mut property = MaybeUninit::uninit();
        unsafe {
            FMOD_Studio_EventDescription_GetUserPropertyByIndex(
                self.inner,
                index,
                property.as_mut_ptr(),
            )
            .to_result()?;

            // FIXME wrong lifetimes + wildly unsafe
            let property = UserProperty::from_ffi(property.assume_init());
            Ok(property)
        }
    }

    pub fn user_property_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetUserPropertyCount(self.inner, &mut count)
                .to_result()?;
        }
        Ok(count)
    }
}

impl EventDescription {
    /// Retrieves the GUID.
    pub fn get_id(&self) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetID(self.inner, guid.as_mut_ptr()).to_result()?;

            let guid = guid.assume_init().into();

            Ok(guid)
        }
    }

    /// Retrieves the length of the timeline.
    ///
    /// A timeline's length is the largest of any logic markers, transition leadouts and the end of any trigger boxes on the timeline.
    pub fn get_length(&self) -> Result<c_int> {
        let mut length = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetLength(self.inner, &mut length).to_result()?;
        }
        Ok(length)
    }

    /// Retrieves the path.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    // TODO: convert into possible macro for the sake of reusing code
    pub fn get_path(&self) -> Result<String> {
        let mut string_len = 0;

        // retrieve the length of the string.
        // this includes the null terminator, so we don't need to account for that.
        unsafe {
            let error = FMOD_Studio_EventDescription_GetPath(
                self.inner,
                std::ptr::null_mut(),
                0,
                &mut string_len,
            )
            .to_error();

            // we expect the error to be fmod_err_truncated.
            // if it isn't, we return the error.
            match error {
                Some(error) if error.code != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(error),
                _ => {}
            }
        };

        let mut path = vec![0u8; string_len as usize];
        let mut expected_string_len = 0;

        unsafe {
            FMOD_Studio_EventDescription_GetPath(
                self.inner,
                // u8 and i8 have the same layout, so this is ok
                path.as_mut_ptr().cast(),
                string_len,
                &mut expected_string_len,
            )
            .to_result()?;

            debug_assert_eq!(string_len, expected_string_len);

            // all public fmod apis return UTF-8 strings. this should be safe.
            // if i turn out to be wrong, perhaps we should add extra error types?
            let path = String::from_utf8_unchecked(path);

            Ok(path)
        }
    }

    /// Retrieves the event user data.
    ///
    /// This function allows arbitrary user data to be retrieved from this object.
    pub fn get_user_data<T>(&self) -> Result<Option<Arc<T>>>
    where
        T: Send + Sync + 'static,
    {
        unsafe {
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_EventDescription_GetUserData(self.inner, &mut userdata).to_result()?;

            if userdata.is_null() {
                return Ok(None);
            }

            // userdata should ALWAYS be InternalUserdata
            let userdata = &mut *userdata.cast::<InternalUserdata>();
            let userdata = userdata
                .userdata
                .clone()
                .map(Arc::downcast::<T>)
                .and_then(std::result::Result::ok);
            Ok(userdata)
        }
    }

    /// Sets the user callback.
    ///
    /// This function sets a user callback which will be assigned to all event instances subsequently created from the event.
    /// The callback for individual instances can be set with [`EventInstance::set_callback`].
    ///
    /// The provided callback may be shared/accessed from multiple threads, and so must implement Send + Sync 'static
    pub fn set_callback<F>(&self, callback: F, mask: EventCallbackMask) -> Result<()>
    where
        F: Fn(EventCallbackKind, EventInstance) -> Result<()> + Send + Sync + 'static,
    {
        // Always enable destroyed to deallocate any userdata attached to events
        let raw_mask = (mask | EventCallbackMask::DESTROYED).into();

        unsafe {
            let userdata = &mut *self.get_or_insert_userdata()?;
            userdata.callback = Some(Arc::new(callback));
            userdata.callback_mask = mask;

            // is this allowed to be null?
            FMOD_Studio_EventDescription_SetCallback(
                self.inner,
                Some(internal_event_callback),
                raw_mask,
            )
            .to_result()
        }
    }

    /// Sets the event user data.
    ///
    /// This function allows arbitrary user data to be attached to this object.
    /// The provided data may be shared/accessed from multiple threads, and so must implement Send + Sync 'static.
    pub fn set_user_data<T>(&self, data: Option<T>) -> Result<()>
    where
        T: Any + Send + Sync + 'static,
    {
        unsafe {
            let userdata = &mut *self.get_or_insert_userdata()?;
            userdata.userdata = data.map(|d| Arc::new(d) as _); // closure is necessary to unsize type
        }

        Ok(())
    }

    unsafe fn get_or_insert_userdata(&self) -> Result<*mut InternalUserdata> {
        unsafe {
            let mut userdata = std::ptr::null_mut();
            FMOD_Studio_EventDescription_GetUserData(self.inner, &mut userdata).to_result()?;

            // FIXME extract this common behavior into a macro or something
            // create and set the userdata if we haven't already
            if userdata.is_null() {
                let boxed_userdata = Box::new(InternalUserdata {
                    callback: None,
                    callback_mask: EventCallbackMask::empty(),
                    userdata: None,
                    is_from_event_instance: false,
                });
                userdata = Box::into_raw(boxed_userdata).cast();

                FMOD_Studio_EventDescription_SetUserData(self.inner, userdata).to_result()?;
                // set the callback if we haven't set the userdata.
                // we should only need to do this here, because the callback is inherited by all event instances, unless modified.
                // since we always keep the FMOD_STUDIO_EVENT_CALLBACK_DESTROYED bit set when modifying callbacks, this is ok.
                FMOD_Studio_EventDescription_SetCallback(
                    self.inner,
                    Some(internal_event_callback),
                    FMOD_STUDIO_EVENT_CALLBACK_DESTROYED,
                )
                .to_result()?;
            }

            Ok(userdata.cast::<InternalUserdata>())
        }
    }

    /// Checks that the [`EventDescription`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_EventDescription_IsValid(self.inner).into() }
    }
}
