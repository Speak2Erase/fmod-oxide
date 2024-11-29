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

/// Trait for this particular FMOD callback.
///
/// No `self` parameter is passed to the callback!
#[allow(unused_variables)]
pub trait EventInstanceCallback {
    /// Called when an instance is fully created.
    fn created(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// Called when an instance is just about to be destroyed.
    fn destroyed(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// [`EventInstance::start`] has been called on an event which was not already playing.
    /// The event will remain in this state until its sample data has been loaded.
    fn starting(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// The event has commenced playing.
    /// Normally this callback will be issued immediately after [`EventInstanceCallback::starting`], but may be delayed until sample data has loaded.
    fn started(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// [`EventInstance::start`] has been called on an event which was already playing.
    fn restarted(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// The event has stopped.
    fn stopped(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// [`EventInstance::start`] has been called but the polyphony settings did not allow the event to start.
    ///
    /// In this case none of [`EventInstanceCallback::starting`], [`EventInstanceCallback::started`] and [`EventInstanceCallback::stopped`] will be called.
    fn start_failed(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// A programmer sound is about to play. FMOD expects the callback to provide an [`Sound`] object for it to use.
    fn create_programmer_sound(
        event: EventInstance,
        sound_props: ProgrammerSoundProperties<'_>,
    ) -> Result<()> {
        Ok(())
    }

    /// A programmer sound has stopped playing. At this point it is safe to release the [`Sound`] object that was used.
    fn destroy_programmer_sound(
        event: EventInstance,
        sound_props: ProgrammerSoundProperties<'_>,
    ) -> Result<()> {
        Ok(())
    }

    /// Called when a DSP plug-in instance has just been created.
    fn plugin_created(event: EventInstance, plugin_props: PluginInstanceProperties) -> Result<()> {
        Ok(())
    }

    /// Called when a DSP plug-in instance is about to be destroyed.
    fn plugin_destroyed(
        event: EventInstance,
        plugin_props: PluginInstanceProperties,
    ) -> Result<()> {
        Ok(())
    }

    /// Called when the timeline passes a named marker.
    fn timeline_marker(
        event: EventInstance,
        timeline_props: TimelineMarkerProperties,
    ) -> Result<()> {
        Ok(())
    }

    /// Called when the timeline hits a beat in a tempo section.
    fn timeline_beat(event: EventInstance, timeline_beat: TimelineBeatProperties) -> Result<()> {
        Ok(())
    }

    /// Called when the event plays a sound.
    fn sound_played(event: EventInstance, sound: Sound) -> Result<()> {
        Ok(())
    }

    /// Called when the event finishes playing a sound.
    fn sound_stopped(event: EventInstance, sound: Sound) -> Result<()> {
        Ok(())
    }

    /// Called when the event becomes virtual.
    fn real_to_virtual(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// Called when the event becomes real.
    fn virtual_to_real(event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// Called when a new event is started by a start event command.
    fn start_event_command(event: EventInstance, new_event: EventInstance) -> Result<()> {
        Ok(())
    }

    /// Called when the timeline hits a beat in a tempo section of a nested event.
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
    let event = EventInstance::from(event);
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
            let new_event = EventInstance::from(parameters.cast());
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

impl EventInstance {
    /// Sets the event instance user data.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    /// Retrieves the event instance user data.
    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetUserData(self.inner.as_ptr(), &mut userdata)
                .to_result()?;
        }
        Ok(userdata)
    }

    /// Sets the user callback.
    pub fn set_callback<C: EventInstanceCallback>(&self, mask: EventCallbackMask) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetCallback(
                self.inner.as_ptr(),
                Some(event_callback_impl::<C>),
                mask.into(),
            )
            .to_result()
        }
    }
}
