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
// along with fmod-rs.  If not, see <https://www.gnu.org/licenses/>.

use std::{ffi::CStr, os::raw::c_void, sync::Arc};

use fmod_sys::*;

use crate::{core::Sound, Shareable, UserdataTypes};

use super::{
    EventCallbackKind, EventCallbackMask, PluginInstanceProperties, ProgrammerSoundProperties,
    TimelineMarkerProperties,
};

mod description;
mod instance;

pub use description::*;
pub use instance::*;

pub(crate) struct InternalUserdata<U: UserdataTypes> {
    // this is an arc in case someone releases the event instance or description while holding onto a reference to the userdata
    // also this is logically and actually shared because event instances inherit their description's userdata and callbacks unless modified
    pub(crate) userdata: Option<Arc<U::Event>>,
    pub(crate) callback: Option<Arc<dyn EventCallback<U>>>,
    // used to ensure we don't misfire callbacks (we always subscribe to releasing events to ensure userdata is freed)
    pub(crate) callback_mask: EventCallbackMask,
    // used to prevent use after frees so events don't accidentally free their description's userdata
    pub(crate) is_from_event_instance: bool,
}

// hilariously long type signature because clippy
pub trait EventCallback<U: UserdataTypes>:
    Fn(EventCallbackKind<U>, EventInstance<U>) -> Result<()> + Shareable
{
}
impl<T, U> EventCallback<U> for T
where
    T: Fn(EventCallbackKind<U>, EventInstance<U>) -> Result<()> + Shareable,
    U: UserdataTypes,
{
}

pub(crate) unsafe extern "C" fn internal_event_callback<U: UserdataTypes>(
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
    let userdata = unsafe { &mut *userdata.cast::<InternalUserdata<U>>() };

    let mut result = FMOD_RESULT::FMOD_OK;
    if let Some(callback) = &userdata.callback {
        if userdata.callback_mask.contains(kind.into()) {
            let event_instance = unsafe { EventInstance::from_ffi(event) };

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
                    EventCallbackKind::StartEventCommand(unsafe {
                        EventInstance::from_ffi(parameters.cast::<FMOD_STUDIO_EVENTINSTANCE>())
                    })
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
