// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CStr;
use std::ffi::c_void;

use crate::{
    studio::{
        EventCallbackMask, EventInstance, PluginInstanceProperties, ProgrammerSoundProperties,
        TimelineBeatProperties, TimelineMarkerProperties, TimelineNestedBeatProperties,
    },
    Sound,
};

#[cfg(feature = "userdata-abstraction")]
use crate::userdata::{get_userdata, insert_userdata, set_userdata, Userdata};

#[allow(unused_variables)]
pub trait EventInstanceCallback {
    fn created(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn destroyed(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn starting(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn started(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn restarted(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn stopped(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn start_failed(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn create_programmer_sound(
        event: EventInstance,
        sound_props: ProgrammerSoundProperties<'_>,
    ) -> Result<()> {
        Ok(())
    }

    fn destroy_programmer_sound(
        event: EventInstance,
        sound_props: ProgrammerSoundProperties<'_>,
    ) -> Result<()> {
        Ok(())
    }

    fn plugin_created(event: EventInstance, plugin_props: PluginInstanceProperties) -> Result<()> {
        Ok(())
    }

    fn plugin_destroyed(
        event: EventInstance,
        plugin_props: PluginInstanceProperties,
    ) -> Result<()> {
        Ok(())
    }

    fn timeline_marker(
        event: EventInstance,
        timeline_props: TimelineMarkerProperties,
    ) -> Result<()> {
        Ok(())
    }

    fn timeline_beat(event: EventInstance, timeline_beat: TimelineBeatProperties) -> Result<()> {
        Ok(())
    }

    fn sound_played(event: EventInstance, sound: Sound) -> Result<()> {
        Ok(())
    }

    fn sound_stopped(event: EventInstance, sound: Sound) -> Result<()> {
        Ok(())
    }

    fn real_to_virtual(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn virtual_to_real(event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn start_event_command(event: EventInstance, new_event: EventInstance) -> Result<()> {
        Ok(())
    }

    fn nested_timeline_beat(
        event: EventInstance,
        timeline_props: TimelineNestedBeatProperties,
    ) -> Result<()> {
        Ok(())
    }
}

pub(crate) unsafe extern "C" fn event_callback_impl<C: EventInstanceCallback>(
    kind: FMOD_STUDIO_EVENT_CALLBACK_TYPE,
    event: *mut FMOD_STUDIO_EVENTINSTANCE,
    parameters: *mut c_void,
) -> FMOD_RESULT {
    // FIXME handle panics
    let event = unsafe { EventInstance::from_ffi(event) };
    let result = match kind {
        FMOD_STUDIO_EVENT_CALLBACK_CREATED => C::created(event),
        FMOD_STUDIO_EVENT_CALLBACK_DESTROYED => C::destroyed(event),
        FMOD_STUDIO_EVENT_CALLBACK_STARTING => C::starting(event),
        FMOD_STUDIO_EVENT_CALLBACK_STARTED => C::started(event),
        FMOD_STUDIO_EVENT_CALLBACK_RESTARTED => C::restarted(event),
        FMOD_STUDIO_EVENT_CALLBACK_STOPPED => C::stopped(event),
        FMOD_STUDIO_EVENT_CALLBACK_START_FAILED => C::start_failed(event),
        FMOD_STUDIO_EVENT_CALLBACK_CREATE_PROGRAMMER_SOUND => {
            let props = unsafe {
                let props = &mut *parameters.cast::<FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES>();
                ProgrammerSoundProperties {
                    name: Utf8CStr::from_ptr_unchecked(props.name).to_cstring(),
                    sound: &mut *std::ptr::addr_of_mut!(props.sound).cast(),
                    subsound_index: &mut props.subsoundIndex,
                }
            };
            C::create_programmer_sound(event, props)
        }
        FMOD_STUDIO_EVENT_CALLBACK_DESTROY_PROGRAMMER_SOUND => {
            let props = unsafe {
                let props = &mut *parameters.cast::<FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES>();
                ProgrammerSoundProperties {
                    name: Utf8CStr::from_ptr_unchecked(props.name).to_cstring(),
                    sound: &mut *std::ptr::addr_of_mut!(props.sound).cast(),
                    subsound_index: &mut props.subsoundIndex,
                }
            };
            C::destroy_programmer_sound(event, props)
        }
        FMOD_STUDIO_EVENT_CALLBACK_PLUGIN_CREATED => {
            let props = unsafe { PluginInstanceProperties::from_ffi(*parameters.cast()) };
            C::plugin_created(event, props)
        }
        FMOD_STUDIO_EVENT_CALLBACK_PLUGIN_DESTROYED => {
            let props = unsafe { PluginInstanceProperties::from_ffi(*parameters.cast()) };
            C::plugin_destroyed(event, props)
        }
        FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_MARKER => {
            let props = unsafe { TimelineMarkerProperties::from_ffi(*parameters.cast()) };
            C::timeline_marker(event, props)
        }
        FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_BEAT => {
            let props = unsafe {
                TimelineBeatProperties::from(
                    *parameters.cast::<FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES>(),
                )
            };
            C::timeline_beat(event, props)
        }
        FMOD_STUDIO_EVENT_CALLBACK_SOUND_PLAYED => {
            let sound = parameters.cast::<FMOD_SOUND>().into();
            C::sound_played(event, sound)
        }
        FMOD_STUDIO_EVENT_CALLBACK_SOUND_STOPPED => {
            let sound = parameters.cast::<FMOD_SOUND>().into();
            C::sound_stopped(event, sound)
        }
        FMOD_STUDIO_EVENT_CALLBACK_REAL_TO_VIRTUAL => C::real_to_virtual(event),
        FMOD_STUDIO_EVENT_CALLBACK_VIRTUAL_TO_REAL => C::virtual_to_real(event),
        FMOD_STUDIO_EVENT_CALLBACK_START_EVENT_COMMAND => {
            let new_event = unsafe { EventInstance::from_ffi(parameters.cast()) };
            C::start_event_command(event, new_event)
        }
        FMOD_STUDIO_EVENT_CALLBACK_NESTED_TIMELINE_BEAT => {
            let props = unsafe {
                TimelineNestedBeatProperties::from(
                    *parameters.cast::<FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES>(),
                )
            };
            C::nested_timeline_beat(event, props)
        }
        _ => {
            eprintln!("warning: unknown event callback type {kind}");
            return FMOD_RESULT::FMOD_OK;
        }
    };
    result.into()
}

#[cfg(feature = "userdata-abstraction")]
impl EventInstance {
    pub fn set_userdata(&self, userdata: Userdata) -> Result<()> {
        let pointer = self.get_raw_userdata()?;
        let desc_pointer = self.get_description()?.get_raw_userdata()?;

        // if the pointer is null or the same as the description pointer, insert the userdata
        if pointer.is_null() || pointer == desc_pointer {
            let key = insert_userdata(userdata, *self);
            self.set_raw_userdata(key.into())?;
        // if not then we already have a key, so just set the userdata
        } else {
            set_userdata(pointer.into(), userdata);
        }

        Ok(())
    }

    pub fn get_userdata(&self) -> Result<Option<Userdata>> {
        let pointer = self.get_raw_userdata()?;
        Ok(get_userdata(pointer.into()))
    }
}

impl EventInstance {
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_raw_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetUserData(self.inner, userdata).to_result() }
    }

    pub fn get_raw_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetUserData(self.inner, &mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    pub fn set_callback<C: EventInstanceCallback>(&self, mask: EventCallbackMask) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetCallback(
                self.inner,
                Some(event_callback_impl::<C>),
                mask.into(),
            )
            .to_result()
        }
    }
}
