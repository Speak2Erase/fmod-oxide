// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_int, c_uint, c_void},
    mem::MaybeUninit,
};

use fmod_sys::*;
use lanyard::Utf8CString;

use crate::{Guid, OutputType, Speaker, SpeakerMode, TimeUnit};

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
}
