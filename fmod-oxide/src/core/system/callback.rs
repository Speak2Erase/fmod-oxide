// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_int, c_void};

use fmod_sys::*;
use lanyard::Utf8CStr;

use crate::{
    studio, Channel, ChannelControl, ChannelGroup, Dsp, DspConnection, Geometry, OutputType,
    Reverb3D, Sound, SoundGroup, System,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCallbackInfo<'a> {
    pub error: Error,
    pub instance: Instance,
    pub function_name: &'a Utf8CStr,
    pub function_params: &'a Utf8CStr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instance {
    None,
    System(System),
    Channel(Channel),
    ChannelGroup(ChannelGroup),
    ChannelControl(ChannelControl),
    Sound(Sound),
    SoundGroup(SoundGroup),
    Dsp(Dsp),
    DspConnection(DspConnection),
    Geometry(Geometry),
    Reverb3D(Reverb3D),
    StudioSystem(studio::System),
    StudioEventDescription(studio::EventDescription),
    StudioEventInstance(studio::EventInstance),
    StudioParameterInstance,
    StudioBus(studio::Bus),
    StudioVCA(studio::Vca),
    StudioBank(studio::Bank),
    StudioCommandReplay(studio::CommandReplay),
}

impl ErrorCallbackInfo<'_> {
    /// # Safety
    ///
    /// The function name and function params fields of [`FMOD_ERRORCALLBACK_INFO`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_ERRORCALLBACK_INFO) -> Self {
        Self {
            error: value.result.into(),
            instance: match value.instancetype {
                FMOD_ERRORCALLBACK_INSTANCETYPE_NONE => Instance::None,
                FMOD_ERRORCALLBACK_INSTANCETYPE_SYSTEM => {
                    Instance::System(System::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNEL => {
                    Instance::Channel(Channel::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNELGROUP => {
                    Instance::ChannelGroup(ChannelGroup::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNELCONTROL => {
                    Instance::ChannelControl(ChannelControl::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_SOUND => {
                    Instance::Sound(Sound::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_SOUNDGROUP => {
                    Instance::SoundGroup(SoundGroup::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_DSP => {
                    Instance::Dsp(Dsp::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_DSPCONNECTION => {
                    Instance::DspConnection(DspConnection::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_GEOMETRY => {
                    Instance::Geometry(Geometry::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_REVERB3D => {
                    Instance::Reverb3D(Reverb3D::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_SYSTEM => {
                    Instance::StudioSystem(studio::System::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTDESCRIPTION => {
                    Instance::StudioEventDescription(studio::EventDescription::from(
                        value.instance.cast(),
                    ))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTINSTANCE => {
                    Instance::StudioEventInstance(studio::EventInstance::from(
                        value.instance.cast(),
                    ))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_PARAMETERINSTANCE => {
                    Instance::StudioParameterInstance
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BUS => {
                    Instance::StudioBus(studio::Bus::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_VCA => {
                    Instance::StudioVCA(studio::Vca::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BANK => {
                    Instance::StudioBank(studio::Bank::from(value.instance.cast()))
                }
                FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_COMMANDREPLAY => {
                    Instance::StudioCommandReplay(studio::CommandReplay::from(
                        value.instance.cast(),
                    ))
                }
                _ => {
                    eprintln!("warning: unknown instance type {}", value.instancetype);
                    Instance::None
                }
            },
            function_name: unsafe { Utf8CStr::from_ptr_unchecked(value.functionname) },
            function_params: unsafe { Utf8CStr::from_ptr_unchecked(value.functionparams) },
        }
    }
}

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct SystemCallbackMask: FMOD_SYSTEM_CALLBACK_TYPE {
      const DEVICELISTCHANGED     = FMOD_SYSTEM_CALLBACK_DEVICELISTCHANGED;
      const DEVICELOST            = FMOD_SYSTEM_CALLBACK_DEVICELOST;
      const MEMORYALLOCATIONFAILED= FMOD_SYSTEM_CALLBACK_MEMORYALLOCATIONFAILED;
      const THREADCREATED         = FMOD_SYSTEM_CALLBACK_THREADCREATED;
      const BADDSPCONNECTION      = FMOD_SYSTEM_CALLBACK_BADDSPCONNECTION;
      const PREMIX                = FMOD_SYSTEM_CALLBACK_PREMIX;
      const POSTMIX               = FMOD_SYSTEM_CALLBACK_POSTMIX;
      const ERROR                 = FMOD_SYSTEM_CALLBACK_ERROR;
      const MIDMIX                = FMOD_SYSTEM_CALLBACK_MIDMIX;
      const THREADDESTROYED       = FMOD_SYSTEM_CALLBACK_THREADDESTROYED;
      const PREUPDATE             = FMOD_SYSTEM_CALLBACK_PREUPDATE;
      const POSTUPDATE            = FMOD_SYSTEM_CALLBACK_POSTUPDATE;
      const RECORDLISTCHANGED     = FMOD_SYSTEM_CALLBACK_RECORDLISTCHANGED;
      const BUFFEREDNOMIX         = FMOD_SYSTEM_CALLBACK_BUFFEREDNOMIX;
      const DEVICEREINITIALIZE    = FMOD_SYSTEM_CALLBACK_DEVICEREINITIALIZE;
      const OUTPUTUNDERRUN        = FMOD_SYSTEM_CALLBACK_OUTPUTUNDERRUN;
      const RECORDPOSITIONCHANGED = FMOD_SYSTEM_CALLBACK_RECORDPOSITIONCHANGED ;
      const ALL                   = FMOD_SYSTEM_CALLBACK_ALL;
  }
}

impl From<SystemCallbackMask> for FMOD_SYSTEM_CALLBACK_TYPE {
    fn from(mask: SystemCallbackMask) -> Self {
        mask.bits()
    }
}

impl From<FMOD_SYSTEM_CALLBACK_TYPE> for SystemCallbackMask {
    fn from(mask: FMOD_SYSTEM_CALLBACK_TYPE) -> Self {
        Self::from_bits_truncate(mask)
    }
}

#[allow(unused_variables)]
pub trait SystemCallback {
    fn device_list_changed(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn device_lost(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn memory_allocation_failed(
        system: System,
        file: &Utf8CStr,
        size: c_int,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn thread_created(
        system: System,
        handle: *mut c_void,
        thread_name: &Utf8CStr,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn bad_dsp_connection(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn premix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn postmix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn error(
        system: System,
        error_info: ErrorCallbackInfo<'_>,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn mid_mix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn thread_destroyed(
        system: System,
        handle: *mut c_void,
        thread_name: &Utf8CStr,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn pre_update(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn post_update(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn record_list_changed(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn buffered_no_mix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn device_reinitialize(
        system: System,
        output_type: OutputType,
        driver_index: c_int,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn output_underrun(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn record_position_changed(
        system: System,
        sound: Sound,
        record_position: c_int,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }
}

unsafe extern "C" fn callback_impl<C: SystemCallback>(
    system: *mut FMOD_SYSTEM,
    callback_type: FMOD_SYSTEM_CALLBACK_TYPE,
    command_data_1: *mut c_void,
    command_data_2: *mut c_void,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let system = System::from(system);
    match callback_type {
        FMOD_SYSTEM_CALLBACK_DEVICELISTCHANGED => C::device_list_changed(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_DEVICELOST => C::device_lost(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_MEMORYALLOCATIONFAILED => {
            let file = unsafe { Utf8CStr::from_ptr_unchecked(command_data_1.cast()) };
            C::memory_allocation_failed(system, file, command_data_2 as c_int, userdata).into()
        }
        FMOD_SYSTEM_CALLBACK_THREADCREATED => {
            let thread_name = unsafe { Utf8CStr::from_ptr_unchecked(command_data_2.cast()) };
            C::thread_created(system, command_data_1, thread_name, userdata).into()
        }
        FMOD_SYSTEM_CALLBACK_BADDSPCONNECTION => C::bad_dsp_connection(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_PREMIX => C::premix(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_POSTMIX => C::postmix(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_ERROR => {
            let error_info = unsafe { ErrorCallbackInfo::from_ffi(*command_data_1.cast()) };
            C::error(system, error_info, userdata).into()
        }
        FMOD_SYSTEM_CALLBACK_MIDMIX => C::mid_mix(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_THREADDESTROYED => {
            let thread_name = unsafe { Utf8CStr::from_ptr_unchecked(command_data_2.cast()) };
            C::thread_destroyed(system, command_data_1, thread_name, userdata).into()
        }
        FMOD_SYSTEM_CALLBACK_PREUPDATE => C::pre_update(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_POSTUPDATE => C::post_update(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_RECORDLISTCHANGED => C::record_list_changed(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_BUFFEREDNOMIX => C::buffered_no_mix(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_DEVICEREINITIALIZE => {
            let output_type = OutputType::try_from(command_data_1 as FMOD_OUTPUTTYPE)
                .expect("invalid output type");
            C::device_reinitialize(system, output_type, command_data_2 as c_int, userdata).into()
        }
        FMOD_SYSTEM_CALLBACK_OUTPUTUNDERRUN => C::output_underrun(system, userdata).into(),
        FMOD_SYSTEM_CALLBACK_RECORDPOSITIONCHANGED => {
            let sound = Sound::from(command_data_1.cast());
            C::record_position_changed(system, sound, command_data_2 as c_int, userdata).into()
        }
        _ => {
            eprintln!("warning: unknown callback type {callback_type}");
            FMOD_RESULT::FMOD_OK
        }
    }
}

impl System {
    pub fn set_callback<C: SystemCallback>(&self, mask: SystemCallbackMask) -> Result<()> {
        unsafe { FMOD_System_SetCallback(self.inner, Some(callback_impl::<C>), mask.into()).into() }
    }
}
