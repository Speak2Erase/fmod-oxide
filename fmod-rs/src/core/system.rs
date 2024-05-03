// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_int, c_longlong, c_uint, c_void},
    mem::MaybeUninit,
};

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};

use crate::{
    Channel, ChannelGroup, CpuUsage, Dsp, DspType, Geometry, Guid, OutputType, PluginType,
    PortType, Reverb3D, ReverbProperties, Sound, SoundGroup, SoundMode, Speaker, SpeakerMode,
    TimeUnit, Vector,
};

use super::InitFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct System {
    pub(crate) inner: *mut FMOD_SYSTEM,
}

pub struct SystemBuilder {
    pub(crate) system: *mut FMOD_SYSTEM,
}

unsafe impl Send for System {}
unsafe impl Sync for System {}

impl From<*mut FMOD_SYSTEM> for System {
    fn from(value: *mut FMOD_SYSTEM) -> Self {
        System { inner: value }
    }
}

impl From<System> for *mut FMOD_SYSTEM {
    fn from(value: System) -> Self {
        value.inner
    }
}

impl SystemBuilder {
    /// Creates a new [`SystemBuilder`].
    ///
    /// # Safety
    ///
    /// This must be called first to create an FMOD System object before any other API calls (except for [`memory_initialize`](crate::memory_initialize) and [`debug_initialize`](crate::debug_initialize)).
    /// Use this function to create 1 or multiple instances of FMOD System objects.
    ///
    /// Calls to [`SystemBuilder::new`] and [`System::release`] are not thread-safe.
    /// Do not call these functions simultaneously from multiple threads at once.
    pub unsafe fn new() -> Result<Self> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_System_Create(&mut system, FMOD_VERSION).to_result()? };

        Ok(SystemBuilder { system })
    }

    pub fn software_format(
        &mut self,
        sample_rate: c_int,
        speaker_mode: SpeakerMode,
        raw_speakers: c_int,
    ) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetSoftwareFormat(
                self.system,
                sample_rate,
                speaker_mode.into(),
                raw_speakers,
            )
            .to_result()?;
        };
        Ok(self)
    }

    pub fn software_channels(&mut self, software_channels: c_int) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetSoftwareChannels(self.system, software_channels).to_result()?;
        };
        Ok(self)
    }

    pub fn dsp_buffer_size(
        &mut self,
        buffer_size: c_uint,
        buffer_count: c_int,
    ) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetDSPBufferSize(self.system, buffer_size, buffer_count).to_result()?;
        };
        Ok(self)
    }

    pub fn output(&mut self, kind: OutputType) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetOutput(self.system, kind.into()).to_result()?;
        };
        Ok(self)
    }

    pub fn output_by_plugin(&mut self, handle: c_uint) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetOutputByPlugin(self.system, handle).to_result()?;
        };
        Ok(self)
    }

    pub fn build(self, max_channels: c_int, flags: InitFlags) -> Result<System> {
        unsafe { self.build_with_extra_driver_data(max_channels, flags, std::ptr::null_mut()) }
    }

    /// # Safety
    ///
    /// See the FMOD docs explaining driver data for more safety information.
    pub unsafe fn build_with_extra_driver_data(
        self,
        max_channels: c_int,
        flags: InitFlags,
        driver_data: *mut c_void,
    ) -> Result<System> {
        unsafe {
            FMOD_System_Init(self.system, max_channels, flags.bits(), driver_data).to_result()?;
        }
        Ok(System { inner: self.system })
    }
}

impl System {
    /// Close the connection to the output and return to an uninitialized state without releasing the object.
    ///
    /// Closing renders objects created with this System invalid.
    /// Make sure any Sound, [`crate::ChannelGroup`], Geometry and DSP objects are released before calling this.
    ///
    /// All pre-initialize configuration settings will remain and the System can be reinitialized as needed.
    pub fn close(&self) -> Result<SystemBuilder> {
        unsafe {
            FMOD_System_Close(self.inner).to_result()?;
            Ok(SystemBuilder { system: self.inner })
        }
    }

    /// Closes and frees this object and its resources.
    ///
    /// This will internally call [`System::close`], so calling [`System::close`] before this function is not necessary.
    ///
    /// # Safety
    ///
    /// [`System::release`] is not thread-safe. Do not call this function simultaneously from multiple threads at once.
    pub unsafe fn release(&self) -> Result<()> {
        unsafe { FMOD_System_Release(self.inner).to_result() }
    }

    /// Updates the FMOD system.
    ///
    /// Should be called once per 'game' tick, or once per frame in your application to perform actions such as:
    /// - Panning and reverb from 3D attributes changes.
    /// - Virtualization of Channels based on their audibility.
    /// - Mixing for non-realtime output types. See comment below.
    /// - Streaming if using [`InitFlags::STREAM_FROM_UPDATE`].
    /// - Mixing if using [`InitFlags::MIX_FROM_UPDATE`]
    /// - Firing callbacks that are deferred until Update.
    /// - DSP cleanup.
    ///
    /// If [`OutputType::NoSoundNRT`] or  [`OutputType::WavWriterNRT`] output modes are used,
    /// this function also drives the software / DSP engine, instead of it running asynchronously in a thread as is the default behavior.
    /// This can be used for faster than realtime updates to the decoding or DSP engine which might be useful if the output is the wav writer for example.
    ///
    /// If [`InitFlags::STREAM_FROM_UPDATE`]. is used, this function will update the stream engine.
    /// Combining this with the non realtime output will mean smoother captured output.
    pub fn update(&self) -> Result<()> {
        unsafe { FMOD_System_Update(self.inner).to_result() }
    }

    /// Suspend mixer thread and relinquish usage of audio hardware while maintaining internal state.
    ///
    /// Used on mobile platforms when entering a backgrounded state to reduce CPU to 0%.
    ///
    /// All internal state will be maintained, i.e. created [`Sound`] and [`Channel`]s will stay available in memory.
    pub fn suspend_mixer(&self) -> Result<()> {
        unsafe { FMOD_System_MixerSuspend(self.inner).to_result() }
    }

    /// Resume mixer thread and reacquire access to audio hardware.
    ///
    /// Used on mobile platforms when entering the foreground after being suspended.
    ///
    /// All internal state will resume, i.e. created [`Sound`] and [`Channel`]s are still valid and playback will continue.
    pub fn resume_mixer(&self) -> Result<()> {
        unsafe { FMOD_System_MixerResume(self.inner).to_result() }
    }

    #[allow(clippy::doc_markdown)]
    /// Sets the type of output interface used to run the mixer.
    ///
    /// This function is typically used to select between different OS specific audio APIs which may have different features.
    ///
    /// It is only necessary to call this function if you want to specifically switch away from the default output mode for the operating system.
    /// The most optimal mode is selected by default for the operating system.
    ///
    /// (Windows, UWP, GameCore, Android, MacOS, iOS, Linux Only) This function can be called from outside the builder.
    ///
    /// When using the Studio API, switching to an NRT (non-realtime) output type after FMOD is already initialized
    /// will not behave correctly unless the Studio API was initialized with [`crate::studio::InitFlags::SYNCHRONOUS_UPDATE`].
    pub fn set_output(&self, output_type: OutputType) -> Result<()> {
        unsafe { FMOD_System_SetOutput(self.inner, output_type.into()).to_result() }
    }

    /// Retrieves the type of output interface used to run the mixer.
    pub fn get_output_type(&self) -> Result<OutputType> {
        let mut output_type = 0;
        unsafe {
            FMOD_System_GetOutput(self.inner, &mut output_type).to_result()?;
        }
        let output_type = output_type.try_into()?;
        Ok(output_type)
    }

    /// Retrieves the number of output drivers available for the selected output type.
    ///
    /// If [`SystemBuilder::output`]/[`System::set_output`] has not been called,
    /// this function will return the number of drivers available for the default output type.
    /// A possible use for this function is to iterate through available sound devices for the current output type,
    /// and use [`System::get_driver_info`] to get the device's name and other attributes.
    pub fn get_driver_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_System_GetNumDrivers(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves identification information about a sound device specified by its index, and specific to the selected output mode.
    pub fn get_driver_info(
        &self,
        id: c_int,
    ) -> Result<(Utf8CString, Guid, c_int, SpeakerMode, c_int)> {
        unsafe {
            let mut name = [0_i8; 512];
            let mut guid = MaybeUninit::zeroed();
            let mut system_rate = 0;
            let mut speaker_mode = 0;
            let mut speaker_mode_channels = 0;

            FMOD_System_GetDriverInfo(
                self.inner,
                id,
                name.as_mut_ptr(),
                512,
                guid.as_mut_ptr(),
                &mut system_rate,
                &mut speaker_mode,
                &mut speaker_mode_channels,
            )
            .to_result()?;

            // FIXME is this right?
            let name = name
                .into_iter()
                .take_while(|&v| v != 0)
                .map(|v| v as u8)
                .collect();
            let name = Utf8CString::from_utf8_with_nul_unchecked(name);
            let guid = guid.assume_init().into();
            let speaker_mode = speaker_mode.try_into()?;

            Ok((name, guid, system_rate, speaker_mode, speaker_mode_channels))
        }
    }

    /// Sets the output driver for the selected output type.
    ///
    /// When an output type has more than one driver available, this function can be used to select between them.
    ///
    /// When this function is called, the current driver will be shutdown and the newly selected driver will be initialized / started.
    pub fn set_driver(&self, driver: c_int) -> Result<()> {
        unsafe { FMOD_System_SetDriver(self.inner, driver).to_result() }
    }

    /// Retrieves the output driver for the selected output type.
    pub fn get_driver(&self) -> Result<c_int> {
        let mut driver = 0;
        unsafe {
            FMOD_System_GetDriver(self.inner, &mut driver).to_result()?;
        }
        Ok(driver)
    }

    /// Retrieves the maximum number of software mixed Channels possible.
    ///
    /// Software [`Channel`]s refers to real voices that will play,
    /// with the return value being the maximum number of voices before successive voices start becoming virtual.
    /// For differences between real and virtual voices see the Virtual Voices guide.
    pub fn get_software_channels(&self) -> Result<c_int> {
        let mut channels = 0;
        unsafe {
            FMOD_System_GetSoftwareChannels(self.inner, &mut channels).to_result()?;
        }
        Ok(channels)
    }

    /// Retrieves the output format for the software mixer.
    pub fn get_software_format(&self) -> Result<(c_int, SpeakerMode, c_int)> {
        let mut sample_rate = 0;
        let mut speaker_mode = 0;
        let mut raw_speakers = 0;
        unsafe {
            FMOD_System_GetSoftwareFormat(
                self.inner,
                &mut sample_rate,
                &mut speaker_mode,
                &mut raw_speakers,
            )
            .to_result()?;
        }
        let speaker_mode = speaker_mode.try_into()?;
        Ok((sample_rate, speaker_mode, raw_speakers))
    }

    /// Retrieves the buffer size settings for the FMOD software mixing engine.
    ///
    /// To get the buffer length in milliseconds, divide it by the output rate and multiply the result by 1000.
    /// For a buffer length of 1024 and an output rate of 48khz (see [`SystemBuilder::software_format`]), milliseconds = 1024 / 48000 * 1000 = 21.33ms.
    /// This means the mixer updates every 21.33ms.
    ///
    /// To get the total buffer size multiply the buffer length by the buffer count value.
    /// By default this would be 41024 = 4096 samples, or 421.33ms = 85.33ms.
    /// This would generally be the total latency of the software mixer, but in reality due to one of the buffers being written to constantly,
    /// and the cursor position of the buffer that is audible, the latency is typically more like the (number of buffers - 1.5) multiplied by the buffer length.
    ///
    /// To convert from milliseconds back to 'samples', simply multiply the value in milliseconds by the sample rate of the output
    /// (ie 48000 if that is what it is set to), then divide by 1000.
    pub fn get_dsp_buffer_size(&self) -> Result<(c_uint, c_int)> {
        let mut buffer_length = 0;
        let mut buffer_count = 0;
        unsafe {
            FMOD_System_GetDSPBufferSize(self.inner, &mut buffer_length, &mut buffer_count)
                .to_result()?;
        }
        Ok((buffer_length, buffer_count))
    }

    /// Sets the default file buffer size for newly opened streams.
    ///
    /// Larger values will consume more memory, whereas smaller values may cause buffer under-run / starvation / stuttering caused by large delays in disk access (ie netstream),
    /// or CPU usage in slow machines, or by trying to play too many streams at once.
    ///
    /// Does not affect streams created with FMOD_OPENUSER, as the buffer size is specified in System::createSound.
    ///
    /// Does not affect latency of playback. All streams are pre-buffered (unless opened with FMOD_OPENONLY), so they will always start immediately.
    ///
    /// Seek and Play operations can sometimes cause a reflush of this buffer.
    ///
    /// If FMOD_TIMEUNIT_RAWBYTES is used, the memory allocated is two times the size passed in, because fmod allocates a double buffer.
    ///
    /// If FMOD_TIMEUNIT_MS, FMOD_TIMEUNIT_PCM or FMOD_TIMEUNIT_PCMBYTES is used, and the stream is infinite (such as a shoutcast netstream),
    /// or VBR, then FMOD cannot calculate an accurate compression ratio to work with when the file is opened.
    /// This means it will then base the buffersize on FMOD_TIMEUNIT_PCMBYTES, or in other words the number of PCM bytes,
    /// but this will be incorrect for some compressed formats. Use FMOD_TIMEUNIT_RAWBYTES for these type (infinite / undetermined length) of streams for more accurate read sizes.
    ///
    /// To determine the actual memory usage of a stream, including sound buffer and other overhead, use Memory_GetStats before and after creating a sound.
    ///
    /// Stream may still stutter if the codec uses a large amount of cpu time, which impacts the smaller, internal 'decode' buffer.
    /// The decode buffer size is changeable via FMOD_CREATESOUNDEXINFO.
    pub fn set_stream_buffer_size(&self, file_buffer_size: c_uint, kind: TimeUnit) -> Result<()> {
        unsafe {
            FMOD_System_SetStreamBufferSize(self.inner, file_buffer_size, kind.into()).to_result()
        }
    }

    /// Retrieves the default file buffer size for newly opened streams.
    pub fn get_stream_buffer_size(&self) -> Result<(c_uint, TimeUnit)> {
        let mut file_buffer_size = 0;
        let mut time_unit = 0;
        unsafe {
            FMOD_System_GetStreamBufferSize(self.inner, &mut file_buffer_size, &mut time_unit)
                .to_result()?;
        }
        let time_unit = time_unit.try_into()?;
        Ok((file_buffer_size, time_unit))
    }

    // TODO advanced settings

    /// Sets the position of the specified speaker for the current speaker mode.
    ///
    /// This function allows the user to specify the position of their speaker to account for non standard setups.
    /// It also allows the user to disable speakers from 3D consideration in a game.
    ///
    /// This allows you to customize the position of the speakers for the current FMOD_SPEAKERMODE by giving X (left to right) and Y (front to back) coordinates.
    /// When disabling a speaker, 3D spatialization will be redistributed around the missing speaker so signal isn't lost.
    ///
    /// Stereo setup would look like this:
    ///
    /// ```rs
    /// system.set_speaker_position(fmod::Speaker::FrontLeft, -1.0,  0.0, true);
    /// system.set_speaker_position(system, fmod::Speaker::FrontRight, 1.0f,  0.0f, true);
    /// ```
    ///
    /// 7.1 setup would look like this:
    /// ```rs
    /// system.set_speaker_position(fmod::Speaker::FrontLeft,      -30_f32.to_radians().sin(),  -30_f32.to_radians().cos(), true);
    /// system.set_speaker_position(fmod::Speaker::FrontRight,      30_f32.to_radians().sin(),   30_f32.to_radians().cos(), true);
    /// system.set_speaker_position(fmod::Speaker::FrontCenter,      0_f32.to_radians().sin(),    0_f32.to_radians().cos(), true);
    /// system.set_speaker_position(fmod::Speaker::LowFrequency,     0_f32.to_radians().sin(),    0_f32.to_radians().cos(), true);
    /// system.set_speaker_position(fmod::Speaker::SurroundLeft,   -90_f32.to_radians().sin(),  -90_f32.to_radians().cos(), true);
    /// system.set_speaker_position(fmod::Speaker::SurroundRight,   90_f32.to_radians().sin(),   90_f32.to_radians().cos(), true);
    /// system.set_speaker_position(fmod::Speaker::BackLeft,      -150_f32.to_radians().sin(), -150_f32.to_radians().cos(), true);
    /// system.set_speaker_position(fmod::Speaker::BackRight,      150_f32.to_radians().sin(),  150_f32.to_radians().cos(), true);
    /// ```
    ///
    /// Calling System::setSoftwareFormat will override any customization made with this function.
    ///
    /// Users of the Studio API should be aware this function does not affect the speaker positions used by the Spatializer DSPs,
    /// it is purely for Core API spatialization via ChannelControl::set3DAttributes.
    pub fn set_speaker_position(
        &self,
        speaker: Speaker,
        x: c_float,
        y: c_float,
        active: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_System_SetSpeakerPosition(self.inner, speaker.into(), x, y, active.into())
                .to_result()
        }
    }

    /// Retrieves the position of the specified speaker for the current speaker mode.
    pub fn get_speaker_position(&self, speaker: Speaker) -> Result<(c_float, c_float, bool)> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut active = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_System_GetSpeakerPosition(self.inner, speaker.into(), &mut x, &mut y, &mut active)
                .to_result()?;
        }
        Ok((x, y, active.into()))
    }

    /// Sets the global doppler scale, distance factor and log roll-off scale for all 3D sound in FMOD.
    ///
    ///          
    ///
    /// The `doppler_scale` is a general scaling factor for how much the pitch varies due to doppler shifting in 3D sound.
    /// Doppler is the pitch bending effect when a sound comes towards the listener or moves away from it, much like the effect you hear when a train goes past you with its horn sounding.
    /// With "`doppler_scale`" you can exaggerate or diminish the effect.
    /// FMOD's effective speed of sound at a doppler factor of 1.0 is 340 m/s.
    ///
    /// The `distance_factor` is the FMOD 3D engine relative distance factor, compared to 1.0 meters.
    /// Another way to put it is that it equates to "how many units per meter does your engine have".
    /// For example, if you are using feet then "scale" would equal 3.28.
    /// This only affects doppler. If you keep your min/max distance, custom roll-off curves, and positions in scale relative to each other, the volume roll-off will not change.
    /// If you set this, the min_distance of a sound will automatically set itself to this value when it is created in case the user forgets to set the min_distance to match the new distance_factor.
    ///
    /// The `rolloff_scale` is a global factor applied to the roll-off of sounds using roll-off modes other than FMOD_3D_CUSTOMROLLOFF.
    /// When a sound uses a roll-off mode other than FMOD_3D_CUSTOMROLLOFF and the distance is greater than the sound's minimum distance,
    /// the distance for the purposes of distance attenuation is calculated according to the formula `distance = (distance - min_distance) * rolloff_scale + min_distance`.
    pub fn set_3d_settings(
        &self,
        doppler_scale: c_float,
        distance_factor: c_float,
        rollof_scale: c_float,
    ) -> Result<()> {
        unsafe {
            FMOD_System_Set3DSettings(self.inner, doppler_scale, distance_factor, rollof_scale)
                .to_result()
        }
    }

    /// Retrieves the global doppler scale, distance factor and roll-off scale for all 3D sounds.
    pub fn get_rd_settings(&self) -> Result<(c_float, c_float, c_float)> {
        let mut doppler_scale = 0.0;
        let mut distance_factor = 0.0;
        let mut rolloff_scale = 0.0;
        unsafe {
            FMOD_System_Get3DSettings(
                self.inner,
                &mut doppler_scale,
                &mut distance_factor,
                &mut rolloff_scale,
            )
            .to_result()?;
        }
        Ok((doppler_scale, distance_factor, rolloff_scale))
    }

    /// Sets the number of 3D 'listeners' in the 3D sound scene.
    ///
    /// This function is useful mainly for split-screen game purposes.
    ///
    /// If the number of listeners is set to more than 1, then panning and doppler are turned off. All sound effects will be mono.
    /// FMOD uses a 'closest sound to the listener' method to determine what should be heard in this case.
    ///
    /// Users of the Studio API should call [`crate::studio::System::set_listener_count`] instead of this function.
    pub fn set_3d_listener_count(&self, count: c_int) -> Result<()> {
        unsafe { FMOD_System_Set3DNumListeners(self.inner, count).to_result() }
    }

    /// Retrieves the number of 3D listeners.
    pub fn get_3d_listener_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_System_Get3DNumListeners(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    // TODO rolloff callback
    // TODO filesystem

    /// Specify a base search path for plugins so they can be placed somewhere else than the directory of the main executable.
    pub fn set_plugin_path(&self, path: &Utf8CStr) -> Result<()> {
        unsafe { FMOD_System_SetPluginPath(self.inner, path.as_ptr()).to_result() }
    }

    /// Loads an FMOD (DSP, Output or Codec) plugin from file.
    ///
    /// Once loaded DSP plugins can be used via System::createDSPByPlugin, output plugins can be use via System::setOutputByPlugin and codec plugins will be used automatically.
    ///
    /// When opening a file each codec tests whether it can support the file format in priority order.
    ///
    /// The format of the plugin is dependant on the operating system:
    ///  - Windows / UWP / Xbox One: .dll
    ///  - Linux / Android: .so
    ///  - Macintosh: .dylib
    ///  - PS4: .prx
    // FIXME do we mark this as unsafe? it is loading arbitrary code
    pub fn load_plugin(&self, filename: &Utf8CStr, priority: u32) -> Result<c_uint> {
        let mut handle = 0;
        unsafe {
            FMOD_System_LoadPlugin(self.inner, filename.as_ptr(), &mut handle, priority)
                .to_result()?;
        }
        Ok(handle)
    }

    /// Unloads an FMOD (DSP, Output or Codec) plugin.
    pub fn unload_plugin(&self, handle: c_uint) -> Result<()> {
        unsafe { FMOD_System_UnloadPlugin(self.inner, handle).to_result() }
    }

    /// Retrieves the number of nested plugins from the selected plugin.
    ///
    /// Most plugins contain a single definition, in which case the count is 1, however some have a list of definitions.
    /// his function returns the number of plugins that have been defined.
    ///
    /// See the DSP Plug-in API guide for more information.
    pub fn get_nested_plugin_count(&self, handle: c_uint) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_System_GetNumNestedPlugins(self.inner, handle, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves the handle of a nested plugin.
    ///
    /// This function is used to iterate handles for plugins that have a list of definitions.
    ///
    /// Most plugins contain a single definition.
    /// If this is the case, only index 0 is valid, and the returned handle is the same as the handle passed in.
    ///
    /// See the DSP Plug-in API guide for more information.
    pub fn get_nested_plugin(&self, handle: c_uint, index: c_int) -> Result<c_uint> {
        let mut nested_handle = 0;
        unsafe {
            FMOD_System_GetNestedPlugin(self.inner, handle, index, &mut nested_handle)
                .to_result()?;
        }
        Ok(nested_handle)
    }

    /// Retrieves the number of loaded plugins.
    pub fn get_plugin_count(&self, kind: PluginType) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_System_GetNumPlugins(self.inner, kind.into(), &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves the handle of a plugin based on its type and relative index.
    ///
    /// All plugins whether built in or loaded can be enumerated using this and [`System::get_plugin_count`].
    pub fn get_plugin_handle(&self, kind: PluginType, index: c_int) -> Result<c_uint> {
        let mut handle = 0;
        unsafe {
            FMOD_System_GetPluginHandle(self.inner, kind.into(), index, &mut handle).to_result()?;
        }
        Ok(handle)
    }

    /// Retrieves information for the selected plugin.
    pub fn get_plugin_info(&self, handle: c_uint) -> Result<(PluginType, Utf8CString, c_uint)> {
        let mut plugin_type = 0;
        let mut name = [0; 512];
        let mut version = 0;

        unsafe {
            FMOD_System_GetPluginInfo(
                self.inner,
                handle,
                &mut plugin_type,
                name.as_mut_ptr(),
                512,
                &mut version,
            )
            .to_result()?;

            // FIXME is this right?
            let name = name
                .into_iter()
                .take_while(|&v| v != 0)
                .map(|v| v as u8)
                .collect();
            let name = Utf8CString::from_utf8_with_nul_unchecked(name);
            let plugin_type = plugin_type.try_into()?;
            Ok((plugin_type, name, version))
        }
    }

    /// Selects an output type given a plugin handle.
    pub fn set_output_by_plugin(&self, handle: c_uint) -> Result<()> {
        unsafe { FMOD_System_SetOutputByPlugin(self.inner, handle).to_result() }
    }

    /// Retrieves the plugin handle for the currently selected output type.
    pub fn get_output_by_plugin(&self) -> Result<c_uint> {
        let mut handle = 0;
        unsafe {
            FMOD_System_GetOutputByPlugin(self.inner, &mut handle).to_result()?;
        }
        Ok(handle)
    }

    // TODO create dsp stuff
    // TODO register codec
    // TODO register dsp
    // TODO register output

    /// Set a proxy server to use for all subsequent internet connections.
    ///
    /// Specify the proxy in `host:port` format e.g. `www.fmod.com:8888` (defaults to port 80 if no port is specified).
    ///
    /// Basic authentication is supported using `user:password@host:port` format e.g. `bob:sekrit123@www.fmod.com:8888`
    pub fn set_network_proxy(&self, proxy: &Utf8CStr) -> Result<()> {
        unsafe { FMOD_System_SetNetworkProxy(self.inner, proxy.as_ptr()).to_result() }
    }

    /// Retrieves the URL of the proxy server used in internet streaming.
    pub fn get_network_proxy(&self) -> Result<Utf8CString> {
        let mut proxy = [0; 512];

        unsafe {
            FMOD_System_GetNetworkProxy(self.inner, proxy.as_mut_ptr(), 512).to_result()?;

            // FIXME is this right?
            let name = proxy
                .into_iter()
                .take_while(|&v| v != 0)
                .map(|v| v as u8)
                .collect();
            let name = Utf8CString::from_utf8_with_nul_unchecked(name);
            Ok(name)
        }
    }

    /// Set the timeout for network streams.
    pub fn set_network_timeout(&self, timeout: c_int) -> Result<()> {
        unsafe { FMOD_System_SetNetworkTimeout(self.inner, timeout).to_result() }
    }

    /// Retrieve the timeout value for network streams.
    pub fn get_network_timeout(&self) -> Result<c_int> {
        let mut timeout = 0;
        unsafe {
            FMOD_System_GetNetworkTimeout(self.inner, &mut timeout).to_result()?;
        }
        Ok(timeout)
    }

    /// Retrieves the FMOD version number.
    ///
    /// The version is a 32 bit hexadecimal value formatted as 16:8:8, with the upper 16 bits being the product version,
    /// the middle 8 bits being the major version and the bottom 8 bits being the minor version.
    /// For example a value of 0x00010203 is equal to 1.02.03.
    ///
    /// Compare against [`crate::VERSION`] to make sure crate and runtime library versions match.
    pub fn get_version(&self) -> Result<u32> {
        let mut version = 0;
        unsafe {
            FMOD_System_GetVersion(self.inner, &mut version).to_result()?;
        }
        Ok(version)
    }

    /// Retrieves an output type specific internal native interface.
    ///
    /// Reinterpret the returned handle based on the selected output type, not all types return something.
    ///   [`OutputType::WavWriter`] Pointer to stdio FILE is returned
    ///   [`OutputType::WavWriterNRT`] Pointer to stdio FILE is returned
    ///   [`OutputType::WASAPI`] Pointer to type `IAudioClient` is returned.
    ///   [`OutputType::Alsa`] Pointer to type `snd_pcm_t` is returned.
    ///   [`OutputType::CoreAudio`] Handle of type `AudioUnit` is returned.
    ///   [`OutputType::AudioOut`] Pointer to type int is returned. Handle returned from sceAudioOutOpen.
    ///
    ///
    /// NOTE: Calling this function is safe, but doing anything with the returned pointer is not!!
    pub fn get_output_handle(&self) -> Result<*mut c_void> {
        let mut handle = std::ptr::null_mut();
        unsafe {
            FMOD_System_GetOutputHandle(self.inner, &mut handle).to_result()?;
        }
        Ok(handle)
    }

    /// Retrieves the number of currently playing Channels.
    ///
    /// For differences between real and virtual voices see the Virtual Voices guide for more information.
    pub fn get_playing_channels(&self) -> Result<(c_int, c_int)> {
        let mut channels = 0;
        let mut real_channels = 0;
        unsafe {
            FMOD_System_GetChannelsPlaying(self.inner, &mut channels, &mut real_channels)
                .to_result()?;
        }
        Ok((channels, real_channels))
    }

    /// Retrieves the amount of CPU used for different parts of the Core engine.
    ///
    /// For readability, the percentage values are smoothed to provide a more stable output.
    pub fn get_cpu_usage(&self) -> Result<CpuUsage> {
        let mut cpu_usage = MaybeUninit::zeroed();
        unsafe {
            FMOD_System_GetCPUUsage(self.inner, cpu_usage.as_mut_ptr()).to_result()?;
            let cpu_usage = cpu_usage.assume_init().into();
            Ok(cpu_usage)
        }
    }

    /// Retrieves information about file reads.
    ///
    /// The values returned are running totals that never reset.
    pub fn get_file_usage(&self) -> Result<(c_longlong, c_longlong, c_longlong)> {
        let mut sample_read = 0;
        let mut stream_read = 0;
        let mut other_read = 0;
        unsafe {
            FMOD_System_GetFileUsage(
                self.inner,
                &mut sample_read,
                &mut stream_read,
                &mut other_read,
            )
            .to_result()?;
        }
        Ok((sample_read, stream_read, other_read))
    }

    /// Retrieves the default matrix used to convert from one speaker mode to another.
    ///
    /// The gain for source channel 's' to target channel 't' is `matrix[t * <number of source channels> + s]`.
    ///
    /// If '`source_mode`' or '`target_mode`' is [`SpeakerMode::Raw`], this function will return [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`].
    /// The number of source channels can be found from [`System::get_speaker_mode_channels`].
    // FIXME: do we take an out slice param?
    pub fn get_default_mix_matric(
        &self,
        source_mode: SpeakerMode,
        target_mode: SpeakerMode,
    ) -> Result<Vec<f32>> {
        let source_channels = self.get_speaker_mode_channels(source_mode)?;
        let target_channels = self.get_speaker_mode_channels(target_mode)?;
        debug_assert!(source_channels <= FMOD_MAX_CHANNEL_WIDTH as c_int);
        debug_assert!(target_channels <= FMOD_MAX_CHANNEL_WIDTH as c_int);
        let mut matrix = vec![0.0; source_channels as usize * target_channels as usize];

        unsafe {
            FMOD_System_GetDefaultMixMatrix(
                self.inner,
                source_mode.into(),
                target_mode.into(),
                matrix.as_mut_ptr(),
                source_channels,
            )
            .to_result()?;
        }
        Ok(matrix)
    }

    /// Retrieves the channel count for a given speaker mode.
    pub fn get_speaker_mode_channels(&self, speaker_mode: SpeakerMode) -> Result<c_int> {
        let mut channels = 0;
        unsafe {
            FMOD_System_GetSpeakerModeChannels(self.inner, speaker_mode.into(), &mut channels)
                .to_result()?;
        }
        Ok(channels)
    }

    /// WARNING: At the moment this function has no guardrails and WILL cause undefined behaviour if used incorrectly.
    /// The [`FMOD_CREATESOUNDEXINFO`] API is *really* complicated and I felt it was better to provide an (unsafe) way to use it until I can figure out a better way to handle it.
    ///
    /// Loads a sound into memory, opens it for streaming or sets it up for callback based sounds.
    ///
    /// [`SoundMode::CREATE_SAMPLE`] will try to load and decompress the whole sound into memory, use [`SoundMode::CREATE_STREAM`] to open it as a stream and have it play back in realtime from disk or another medium.
    /// [`SoundMode::CREATE_COMPRESSED_SAMPLE`] can also be used for certain formats to play the sound directly in its compressed format from the mixer.
    /// - To open a file or URL as a stream, so that it decompresses / reads at runtime, instead of loading / decompressing into memory all at the time of this call, use the [`SoundMode::CREATE_STREAM`] flag.
    /// - To open a file or URL as a compressed sound effect that is not streamed and is not decompressed into memory at load time, use [`SoundMode::CREATE_COMPRESSED_SAMPLE`].
    /// This is supported with MPEG (mp2/mp3), ADPCM/FADPCM, XMA, AT9 and FSB Vorbis files only. This is useful for those who want realtime compressed soundeffects, but not the overhead of disk access.
    /// - To open a sound as 2D, so that it is not affected by 3D processing, use the [`SoundMode::D2`] flag. 3D sound commands will be ignored on these types of sounds.
    /// - To open a sound as 3D, so that it is treated as a 3D sound, use the [`SoundMode::D3`] flag.
    ///
    /// Note that [`SoundMode::OPEN_RAW`], [`SoundMode::OPEN_MEMORY`], [`SoundMode::OPEN_MEMORY_POINT`] and [`SoundMode::OPEN_USER`] will not work here without the exinfo structure present, as more information is needed.
    ///
    /// Use [`SoundMode::NONBLOCKING`] to have the sound open or load in the background.
    /// You can use Sound::getOpenState to determine if it has finished loading / opening or not. While it is loading (not ready), sound functions are not accessible for that sound.
    /// Do not free memory provided with [`SoundMode::OPEN_MEMORY`] if the sound is not in a ready state, as it will most likely lead to a crash.
    ///
    /// To account for slow media that might cause buffer underrun (skipping / stuttering / repeating blocks of audio) with sounds created with [`FMOD_CREATESTREAM`],
    /// use System::setStreamBufferSize to increase read ahead.
    ///
    /// As using [`SoundMode::OPEN_USER`] causes FMOD to ignore whatever is passed as the first argument `name_or_data`, recommended practice is to pass None.
    ///
    /// Specifying [`SoundMode::OPEN_MEMORY_POINT`] will POINT to your memory rather allocating its own sound buffers and duplicating it internally,
    /// this means you cannot free the memory while FMOD is using it, until after Sound::release is called.
    ///
    /// With [`SoundMode::OPEN_MEMORY_POINT`], only PCM formats and compressed formats using [`SoundMode::CREATE_COMPRESSED_SAMPLE`] are supported.
    // FIXME: SAFE SOUNDINFO!!!!!!!
    pub fn create_sound(
        &self,
        name_or_data: Option<&[u8]>,
        mode: SoundMode,
        ex_info: Option<&mut FMOD_CREATESOUNDEXINFO>,
    ) -> Result<Sound> {
        let name_or_data = name_or_data.map_or(std::ptr::null(), <[u8]>::as_ptr).cast();
        let ex_info = ex_info.map_or(std::ptr::null_mut(), std::ptr::from_mut);
        let mut sound = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateSound(self.inner, name_or_data, mode.into(), ex_info, &mut sound)
                .to_result()?;
        }
        Ok(sound.into())
    }

    /// WARNING: At the moment this function has no guardrails and WILL cause undefined behaviour if used incorrectly.
    /// The [`FMOD_CREATESOUNDEXINFO`] API is *really* complicated and I felt it was better to provide an (unsafe) way to use it until I can figure out a better way to handle it.
    ///
    /// Opens a sound for streaming.
    ///
    /// This is a convenience function for [`System::create_sound`] with the [`SoundMode::CREATE_STREAM`] flag added.
    ///
    /// A stream only has one decode buffer and file handle, and therefore can only be played once.
    /// It cannot play multiple times at once because it cannot share a stream buffer if the stream is playing at different positions.
    /// Open multiple streams to have them play concurrently.
    pub fn create_stream(
        &self,
        name_or_data: Option<&[u8]>,
        mode: SoundMode,
        ex_info: Option<&mut FMOD_CREATESOUNDEXINFO>,
    ) -> Result<Sound> {
        let name_or_data = name_or_data.map_or(std::ptr::null(), <[u8]>::as_ptr).cast();
        let ex_info = ex_info.map_or(std::ptr::null_mut(), std::ptr::from_mut);
        let mut sound = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateStream(self.inner, name_or_data, mode.into(), ex_info, &mut sound)
                .to_result()?;
        }
        Ok(sound.into())
    }

    /// WARNING: At the moment this function has no guardrails and WILL cause undefined behaviour if used incorrectly.
    /// The [`FMOD_DSP_DESCRIPTION`] API is *really* complicated and I felt it was better to provide an (unsafe) way to use it until I can figure out a better way to handle it.
    ///
    /// Create a DSP object given a plugin description structure.
    ///
    /// A DSP object is a module that can be inserted into the mixing graph to allow sound filtering or sound generation.
    /// See the DSP architecture guide for more information.
    ///
    /// DSPs must be attached to the DSP graph before they become active, either via ChannelControl::addDSP or DSP::addInput.
    pub fn create_dsp(&self, description: &FMOD_DSP_DESCRIPTION) -> Result<Dsp> {
        let mut dsp = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateDSP(self.inner, description, &mut dsp).to_result()?;
        }
        Ok(dsp.into())
    }

    ///Create a DSP object given a built in type index.
    ///
    /// A DSP object is a module that can be inserted into the mixing graph to allow sound filtering or sound generation. See the DSP architecture guide for more information.
    ///
    /// DSPs must be attached to the DSP graph before they become active, either via ChannelControl::addDSP or DSP::addInput.
    ///
    /// Using [`DspType::VstPlugin`] or [`DspType::WinampPlugin`] will return the first loaded plugin of this type.
    /// To access other plugins of these types, use System::createDSPByPlugin instead.
    pub fn create_dsp_by_type(&self, kind: DspType) -> Result<Dsp> {
        let mut dsp = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateDSPByType(self.inner, kind.into(), &mut dsp).to_result()?;
        }
        Ok(dsp.into())
    }

    /// Create a [`ChannelGroup`] object.
    ///
    /// [`ChannelGroup`]s can be used to assign / group [`Channel`]s, for things such as volume scaling.
    /// [`ChannelGroup`]s are also used for sub-mixing.
    /// Any [`Channel`]s that are assigned to a [`ChannelGroup`] get submixed into that [`ChannelGroup`]'s 'tail' [`Dsp`]. See FMOD_CHANNELCONTROL_DSP_TAIL.
    ///
    /// If a [`ChannelGroup`] has an effect added to it, the effect is processed post-mix from the [`Channel`]s and [`ChannelGroup`]s below it in the mix hierarchy.
    /// See the DSP architecture guide for more information.
    ///
    /// All [`ChannelGroup`]s will initially output directly to the master [`ChannelGroup`] (See System::getMasterChannelGroup).
    /// [`ChannelGroup`]s can be re-parented this with ChannelGroup::addGroup.
    pub fn create_channel_group(&self, name: &Utf8CStr) -> Result<ChannelGroup> {
        let mut channel_group = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateChannelGroup(self.inner, name.as_ptr(), &mut channel_group)
                .to_result()?;
        }
        Ok(channel_group.into())
    }

    /// Creates a [`SoundGroup`] object.
    ///
    /// A [`SoundGroup`] is a way to address multiple [`Sound`]s at once with group level commands, such as:
    ///
    /// - Attributes of Sounds that are playing or about to be played, such as volume. See (SoundGroup::setVolume).
    /// - Control of playback, such as stopping [`Sound`]s. See (SoundGroup::stop).
    /// - Playback behavior such as 'max audible', to limit playback of certain types of [`Sound`]s. See (SoundGroup::setMaxAudible).
    ///
    /// Once a [`SoundGroup`] is created, Sound::setSoundGroup is used to put a [`Sound`] in a [`SoundGroup`].
    pub fn create_sound_group(&self, name: &Utf8CStr) -> Result<SoundGroup> {
        let mut sound_group = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateSoundGroup(self.inner, name.as_ptr(), &mut sound_group)
                .to_result()?;
        }
        Ok(sound_group.into())
    }

    /// Creates a 'virtual reverb' object.
    /// This object reacts to 3D location and morphs the reverb environment based on how close it is to the reverb object's center.
    ///
    /// Multiple reverb objects can be created to achieve a multi-reverb environment.
    /// 1 reverb object is used for all 3D reverb objects (slot 0 by default).
    ///
    /// The 3D reverb object is a sphere having 3D attributes (position, minimum distance, maximum distance) and reverb properties.
    ///
    /// The properties and 3D attributes of all reverb objects collectively determine, along with the listener's position,
    /// the settings of and input gains into a single 3D reverb [`Dsp`].
    ///
    /// When the listener is within the sphere of effect of one or more 3D reverbs,
    /// the listener's 3D reverb properties are a weighted combination of such 3D reverbs.
    ///
    /// When the listener is outside all of the reverbs, no reverb is applied.
    ///
    /// System::setReverbProperties can be used to create an alternative reverb that can be used for 2D and background global reverb.
    ///
    /// To avoid this reverb interfering with the reverb slot used by the 3D reverb, 2D reverb should use a different slot id with System::setReverbProperties,
    /// otherwise FMOD_ADVANCEDSETTINGS::reverb3Dinstance can also be used to place 3D reverb on a different reverb slot.
    ///
    /// Use ChannelControl::setReverbProperties to turn off reverb for 2D sounds (ie set wet = 0).
    ///
    /// Creating multiple reverb objects does not impact performance.
    /// These are 'virtual reverbs'.
    /// There will still be only one reverb [`Dsp`] running that just morphs between the different virtual reverbs.
    ///
    /// Note about reverb [`Dsp`] unit allocation.
    /// To remove the [`Dsp`] unit and the associated CPU cost, first make sure all 3D reverb objects are released.
    /// Then call System::setReverbProperties with the 3D reverb's slot ID (default is 0) with a property point of 0 or NULL, to signal that the reverb instance should be deleted.
    ///
    /// If a 3D reverb is still present, and System::setReverbProperties function is called to free the reverb,
    /// the 3D reverb system will immediately recreate it upon the next System::update call.
    ///
    /// Note that the 3D reverb system will not affect Studio events unless it is explicitly enabled by calling Studio::EventInstance::setReverbLevel on each event instance.
    pub fn create_reverb_3d(&self) -> Result<Reverb3D> {
        let mut reverb = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateReverb3D(self.inner, &mut reverb).to_result()?;
        }
        Ok(reverb.into())
    }

    /// Plays a Sound on a Channel.
    ///
    /// When a sound is played, it will use the sound's default frequency and priority. See Sound::setDefaults.
    ///
    /// A sound defined as [`SoundMode::D3`] will by default play at the 3D position of the listener.
    /// To set the 3D position of the Channel before the sound is audible, start the Channel paused by setting the paused parameter to true, and call ChannelControl::set3DAttributes.
    ///
    /// Specifying a channelgroup as part of playSound is more efficient than using Channel::setChannelGroup after playSound, and could avoid audible glitches if the playSound is not in a paused state.
    ///
    /// Channels are reference counted to handle dead or stolen Channel handles.
    /// See the white paper on Channel handles for more information.
    ///
    /// Playing more Sounds than physical Channels allow is handled with virtual voices.
    /// See the white paper on Virtual Voices for more information.
    pub fn play_sound(
        &self,
        sound: Sound,
        channel_group: Option<ChannelGroup>,
        paused: bool,
    ) -> Result<Channel> {
        let mut channel = std::ptr::null_mut();
        unsafe {
            FMOD_System_PlaySound(
                self.inner,
                sound.into(),
                channel_group.map_or(std::ptr::null_mut(), ChannelGroup::into),
                paused.into(),
                &mut channel,
            )
            .to_result()?;
        }
        Ok(channel.into())
    }

    /// Plays a [`Dsp`] along with any of its inputs on a [`Channel`].
    ///
    /// Specifying a `channel_group` as part of playDSP is more efficient than using Channel::setChannelGroup after playDSP,
    /// and could avoid audible glitches if the playDSP is not in a paused state.
    ///
    /// [`Channel`]s are reference counted to handle dead or stolen [`Channel`] handles. See the white paper on [`Channel`] handles for more information.
    ///
    /// Playing more Sounds or [`Dsp`]s than physical [`Channel`]s allowed is handled with virtual voices.
    /// See the white paper on Virtual Voices for more information.
    pub fn play_dsp(
        &self,
        dsp: Dsp,
        channel_group: Option<ChannelGroup>,
        paused: bool,
    ) -> Result<Channel> {
        let mut channel = std::ptr::null_mut();
        unsafe {
            FMOD_System_PlayDSP(
                self.inner,
                dsp.into(),
                channel_group.map_or(std::ptr::null_mut(), ChannelGroup::into),
                paused.into(),
                &mut channel,
            )
            .to_result()?;
        }
        Ok(channel.into())
    }

    /// Retrieves a handle to a [`Channel`] by ID.
    ///
    /// This function is mainly for getting handles to existing (playing) [`Channel`]s and setting their attributes.
    /// The only way to 'create' an instance of a [`Channel`] for playback is to use [`System::play_sound`] or [`System::play_dsp`].
    pub fn get_channel(&self, channel_id: c_int) -> Result<Channel> {
        let mut channel = std::ptr::null_mut();
        unsafe {
            FMOD_System_GetChannel(self.inner, channel_id, &mut channel).to_result()?;
        }
        Ok(channel.into())
    }

    // TODO dsp info

    /// Retrieves the master [`ChannelGroup`] that all sounds ultimately route to.
    ///
    /// This is the default [`ChannelGroup`] that [`Channel`]s play on,
    /// unless a different [`ChannelGroup`] is specified with [`System::play_sound`], [`System::play_dsp`] or Channel::setChannelGroup.
    /// A master [`ChannelGroup`] can be used to do things like set the 'master volume' for all playing [`Channel`]s. See ChannelControl::setVolume.
    pub fn get_master_channel_group(&self) -> Result<ChannelGroup> {
        let mut channel_group = std::ptr::null_mut();
        unsafe {
            FMOD_System_GetMasterChannelGroup(self.inner, &mut channel_group).to_result()?;
        }
        Ok(channel_group.into())
    }

    /// Retrieves the default [`SoundGroup`], where all sounds are placed when they are created.
    ///
    /// If [`SoundGroup`] is released, the [`Sound`]s will be put back into this [`SoundGroup`].
    pub fn get_master_sound_group(&self) -> Result<SoundGroup> {
        let mut sound_group = std::ptr::null_mut();
        unsafe {
            FMOD_System_GetMasterSoundGroup(self.inner, &mut sound_group).to_result()?;
        }
        Ok(sound_group.into())
    }

    /// Sets the position, velocity and orientation of the specified 3D sound listener.
    ///
    /// The forward and up vectors must be perpendicular and be of unit length (magnitude of each vector should be 1).
    ///
    /// Vectors must be provided in the correct handedness.
    ///
    /// For velocity, remember to use units per second, and not units per frame.
    /// This is a common mistake and will make the doppler effect sound wrong if velocity is based on movement per frame rather than a fixed time period.
    /// If velocity per frame is calculated, it can be converted to velocity per second by dividing it by the time taken between frames as a fraction of a second.
    /// i.e.
    ///
    /// `velocity_units_per_second = (position_currentframe - position_lastframe) / time_taken_since_last_frame_in_seconds`.
    ///
    /// At 60fps the formula would look like `velocity_units_per_second = (position_current_frame - position_last_frame) / 0.0166667`.
    ///
    /// Users of the Studio API should call [`crate::studio::System::set_listener_attributes`] instead of this function.
    pub fn set_3d_listener_attributes(
        &self,
        listener: c_int,
        position: Option<Vector>,
        velocity: Option<Vector>,
        forward: Option<Vector>,
        up: Option<Vector>,
    ) -> Result<()> {
        // these casts are ok as Vector is layout equivalent with FMOD_VECTOR
        let position = position
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        let velocity = velocity
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        let forward = forward
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        let up = up
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        unsafe {
            FMOD_System_Set3DListenerAttributes(
                self.inner, listener, position, velocity, forward, up,
            )
            .to_result()
        }
    }

    /// Retrieves the position, velocity and orientation of the specified 3D sound listener.
    ///
    /// Users of the Studio API should call [`crate::studio::System::get_listener_attributes`] instead of this function.
    pub fn get_3d_listener_attributes(
        &self,
        listener: c_int,
    ) -> Result<(Vector, Vector, Vector, Vector)> {
        let mut position = MaybeUninit::zeroed();
        let mut velocity = MaybeUninit::zeroed();
        let mut forward = MaybeUninit::zeroed();
        let mut up = MaybeUninit::zeroed();
        unsafe {
            FMOD_System_Get3DListenerAttributes(
                self.inner,
                listener,
                position.as_mut_ptr(),
                velocity.as_mut_ptr(),
                forward.as_mut_ptr(),
                up.as_mut_ptr(),
            )
            .to_result()?;

            let position = position.assume_init();
            let velocity = velocity.assume_init();
            let forward = forward.assume_init();
            let up = up.assume_init();

            Ok((position.into(), velocity.into(), forward.into(), up.into()))
        }
    }

    /// Sets parameters for the global reverb environment.
    ///
    /// To assist in defining reverb properties there are several presets available,
    /// see the associated constants on [`ReverbProperties.`].
    ///
    /// When using each instance for the first time,
    /// FMOD will create an SFX reverb [`Dsp`] unit that takes up several hundred kilobytes of memory and some CPU.
    pub fn set_reverb_properties(
        &self,
        instance: c_int,
        properties: Option<ReverbProperties>,
    ) -> Result<()> {
        let properties = properties
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        unsafe { FMOD_System_SetReverbProperties(self.inner, instance, properties).to_result() }
    }

    /// Retrieves the current reverb environment for the specified reverb instance.
    pub fn get_reverb_properties(&self, instance: c_int) -> Result<ReverbProperties> {
        let mut properties = MaybeUninit::zeroed();
        unsafe {
            FMOD_System_GetReverbProperties(self.inner, instance, properties.as_mut_ptr())
                .to_result()?;
            let properties = properties.assume_init().into();
            Ok(properties)
        }
    }

    /// Connect the output of the specified [`ChannelGroup`] to an audio port on the output driver.
    ///
    /// Ports are additional outputs supported by some [`OutputType`] plugins and can include things like controller headsets or dedicated background music streams.
    /// See the Port Support section (where applicable) of each platform's getting started guide found in the platform details chapter.
    pub fn attach_channel_group_to_port(
        &self,
        kind: PortType,
        index: Option<FMOD_PORT_INDEX>,
        channel_group: ChannelGroup,
        pass_through: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_System_AttachChannelGroupToPort(
                self.inner,
                kind.into(),
                index.unwrap_or(FMOD_PORT_INDEX_NONE as FMOD_PORT_INDEX),
                channel_group.into(),
                pass_through.into(),
            )
            .to_result()
        }
    }

    /// Disconnect the output of the specified [`ChannelGroup`] from an audio port on the output driver.
    ///
    /// Removing a [`ChannelGroup`] from a port will reroute the audio back to the main mix.
    pub fn detach_channel_group_from_port(&self, channel_group: ChannelGroup) -> Result<()> {
        unsafe {
            FMOD_System_DetachChannelGroupFromPort(self.inner, channel_group.into()).to_result()
        }
    }

    /// Retrieves the number of recording devices available for this output mode.
    /// Use this to enumerate all recording devices possible so that the user can select one.
    pub fn get_recording_driver_count(&self) -> Result<(c_int, c_int)> {
        let mut drivers = 0;
        let mut connected = 0;
        unsafe {
            FMOD_System_GetRecordNumDrivers(self.inner, &mut drivers, &mut connected)
                .to_result()?;
        }
        Ok((drivers, connected))
    }

    // TODO get recording driver info

    /// Retrieves the current recording position of the record buffer in PCM samples.
    ///
    /// Will return [`FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED`] if the driver is unplugged.
    ///
    /// The position will return to 0 when System::recordStop is called or when a non-looping recording reaches the end.
    ///
    /// PS4 specific note: Record devices are virtual so 'position' will continue to update if the device is unplugged (the OS is generating silence).
    /// This function will still report [`FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED`] for your information though.
    pub fn get_record_position(&self, id: c_int) -> Result<c_uint> {
        let mut position = 0;
        unsafe {
            FMOD_System_GetRecordPosition(self.inner, id, &mut position).to_result()?;
        }
        Ok(position)
    }

    /// Starts the recording engine recording to a pre-created Sound object.
    ///
    /// Will return [`FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED`] if the driver is unplugged.
    ///
    /// Sound must be created as [`SoundMode::CREATE_SAMPLE`].
    /// Raw PCM data can be accessed with Sound::lock, Sound::unlock and System::getRecordPosition.
    ///
    /// Recording from the same driver a second time will stop the first recording.
    ///
    /// For lowest latency set the [`Sound`] sample rate to the rate returned by System::getRecordDriverInfo,
    /// otherwise a resampler will be allocated to handle the difference in frequencies, which adds latency.
    pub fn record_start(&self, id: c_int, sound: Sound, do_loop: bool) -> Result<()> {
        unsafe { FMOD_System_RecordStart(self.inner, id, sound.into(), do_loop.into()).to_result() }
    }

    /// Stops the recording engine from recording to a pre-created Sound object.
    ///
    /// Returns no error if unplugged or already stopped.
    pub fn record_stop(&self, id: c_int) -> Result<()> {
        unsafe { FMOD_System_RecordStop(self.inner, id).to_result() }
    }

    /// Retrieves the state of the FMOD recording API, ie if it is currently recording or not.
    ///
    /// Recording can be started with [`System::record_start`] and stopped with [`System::record_stop`].
    ///
    /// Will return [`FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED`] if the driver is unplugged.
    ///
    /// PS4 specific note: Record devices are virtual so 'position' will continue to update if the device is unplugged (the OS is generating silence).
    /// This function will still report [`FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED`] for your information though.
    pub fn is_recording(&self, id: c_int) -> Result<bool> {
        let mut recording = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_System_IsRecording(self.inner, id, &mut recording).to_result()?;
        }
        Ok(recording.into())
    }

    /// [`Geometry`] creation function. This function will create a base geometry object which can then have polygons added to it.
    ///
    /// Polygons can be added to a geometry object using Geometry::addPolygon. For best efficiency, avoid overlapping of polygons and long thin polygons.
    ///
    /// A geometry object stores its polygons in a group to allow optimization for line testing, insertion and updating of geometry in real-time.
    /// Geometry objects also allow for efficient rotation, scaling and translation of groups of polygons.
    ///
    /// It is important to set the value of maxworldsize to an appropriate value using [`System::set_geometry_settings`].
    pub fn create_geometry(&self, max_polygons: c_int, max_vertices: c_int) -> Result<Geometry> {
        let mut geometry = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateGeometry(self.inner, max_polygons, max_vertices, &mut geometry)
                .to_result()?;
        }
        Ok(geometry.into())
    }

    /// Sets the maximum world size for the geometry engine for performance / precision reasons.
    ///
    /// FMOD uses an efficient spatial partitioning system to store polygons for ray casting purposes.
    /// The maximum size of the world should (`max_world_size`) be set to allow processing within a known range.
    /// Outside of this range, objects and polygons will not be processed as efficiently.
    /// Excessive world size settings can also cause loss of precision and efficiency.
    ///
    /// Setting `max_world_size` should be done first before creating any geometry.
    /// It can be done any time afterwards but may be slow in this case.
    pub fn set_geometry_settings(&self, max_world_size: c_float) -> Result<()> {
        unsafe { FMOD_System_SetGeometrySettings(self.inner, max_world_size).to_result() }
    }

    /// Retrieves the maximum world size for the geometry engine.
    ///
    /// FMOD uses an efficient spatial partitioning system to store polygons for ray casting purposes.
    /// The maximum size of the world should (`max_world_size`) be set to allow processing within a known range.
    /// Outside of this range, objects and polygons will not be processed as efficiently.
    /// Excessive world size settings can also cause loss of precision and efficiency.
    pub fn get_geometry_settings(&self) -> Result<c_float> {
        let mut max_world_size = 0.0;
        unsafe {
            FMOD_System_GetGeometrySettings(self.inner, &mut max_world_size).to_result()?;
        }
        Ok(max_world_size)
    }

    /// Creates a geometry object from a block of memory which contains pre-saved geometry data.
    ///
    /// This function avoids the need to manually create and add geometry for faster start time.
    pub fn load_geometry(&self, data: &[u8]) -> Result<Geometry> {
        let mut geometry = std::ptr::null_mut();
        unsafe {
            FMOD_System_LoadGeometry(
                self.inner,
                data.as_ptr().cast(),
                data.len() as c_int,
                &mut geometry,
            )
            .to_result()?;
        }
        Ok(geometry.into())
    }

    /// Calculates geometry occlusion between a listener and a sound source.
    ///
    /// If single sided polygons have been created, it is important to get the source and listener positions around the right way,
    /// as the occlusion from point A to point B may not be the same as the occlusion from point B to point A.
    pub fn get_geometry_occlusion(
        &self,
        listener: Vector,
        source: Vector,
    ) -> Result<(c_float, c_float)> {
        let mut direct = 0.0;
        let mut reverb = 0.0;
        unsafe {
            FMOD_System_GetGeometryOcclusion(
                self.inner,
                std::ptr::from_ref(&listener).cast(),
                std::ptr::from_ref(&source).cast(),
                &mut direct,
                &mut reverb,
            )
            .to_result()?;
        }
        Ok((direct, reverb))
    }

    /// Mutual exclusion function to lock the FMOD DSP engine (which runs asynchronously in another thread), so that it will not execute.
    ///
    /// If the FMOD DSP engine is already executing, this function will block until it has completed.
    ///
    /// The function may be used to synchronize DSP network operations carried out by the user.
    ///
    /// An example of using this function may be for when the user wants to construct a DSP sub-network, without the DSP engine executing in the background while the sub-network is still under construction.
    ///
    /// Once the user no longer needs the DSP engine locked, it must be unlocked with [`System::unlock_dsp`].
    ///
    /// Note that the DSP engine should not be locked for a significant amount of time, otherwise inconsistency in the audio output may result. (audio skipping / stuttering).
    pub fn lock_dsp(&self) -> Result<()> {
        unsafe { FMOD_System_LockDSP(self.inner).to_result() }
    }

    // TODO add guard and investigate safety
    /// Mutual exclusion function to unlock the FMOD DSP engine (which runs asynchronously in another thread) and let it continue executing.
    ///
    /// The DSP engine must be locked with [`System::lock_dsp`] before this function is called.
    pub fn unlock_dsp(&self) -> Result<()> {
        unsafe { FMOD_System_UnlockDSP(self.inner).to_result() }
    }

    // TODO callbacks and userdata
}
