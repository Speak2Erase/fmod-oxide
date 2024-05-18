// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int, c_uint};

use crate::{ChannelControl, Speaker, SpeakerMode, System, TimeUnit};

pub trait RolloffCallback {
    fn rolloff(channel_control: ChannelControl, distance: c_float) -> c_float;
}
unsafe extern "C" fn rolloff_callback_impl<C: RolloffCallback>(
    channel_control: *mut FMOD_CHANNELCONTROL,
    distance: c_float,
) -> c_float {
    let channel_control = channel_control.into();
    C::rolloff(channel_control, distance)
}

impl System {
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
    /// Does not affect streams created with [`SoundMoude::OpenUser`], as the buffer size is specified in [`System::create_sound`].
    ///
    /// Does not affect latency of playback. All streams are pre-buffered (unless opened with [`SoundMode::OpenOnly`]), so they will always start immediately.
    ///
    /// Seek and Play operations can sometimes cause a reflush of this buffer.
    ///
    /// If [`TimeUnit::RawBytes`] is used, the memory allocated is two times the size passed in, because fmod allocates a double buffer.
    ///
    /// If [`TimeUnit::MS`], [`TimeUnit::PCM`] or [`TimeUnit::PCMBytes`] is used, and the stream is infinite (such as a shoutcast netstream),
    /// or VBR, then FMOD cannot calculate an accurate compression ratio to work with when the file is opened.
    /// This means it will then base the buffersize on [`TimeUnit::PCMBytes`], or in other words the number of PCM bytes,
    /// but this will be incorrect for some compressed formats. Use [`TimeUnit::RawBytes`] for these type (infinite / undetermined length) of streams for more accurate read sizes.
    ///
    /// To determine the actual memory usage of a stream, including sound buffer and other overhead, use [`crate::memory::memory_get_stats`] before and after creating a sound.
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
    /// This allows you to customize the position of the speakers for the current [`SpeakerMode`] by giving X (left to right) and Y (front to back) coordinates.
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
    /// Calling [`SystemBuilder::software_format`] will override any customization made with this function.
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
    pub fn get_3d_settings(&self) -> Result<(c_float, c_float, c_float)> {
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

    /// Sets a callback to allow custom calculation of distance attenuation.
    ///
    /// This function overrides FMOD_3D_INVERSEROLLOFF, FMOD_3D_LINEARROLLOFF, FMOD_3D_LINEARSQUAREROLLOFF, FMOD_3D_INVERSETAPEREDROLLOFF and FMOD_3D_CUSTOMROLLOFF.
    pub fn set_3d_rolloff_callback<C: RolloffCallback>(&self) -> Result<()> {
        unsafe {
            FMOD_System_Set3DRolloffCallback(self.inner, Some(rolloff_callback_impl::<C>))
                .to_result()
        }
    }

    /// Unset the 3d rolloff callback, returning control of distance attenuation to FMOD.
    pub fn unset_3d_rolloff_callback(&self) -> Result<()> {
        unsafe { FMOD_System_Set3DRolloffCallback(self.inner, None).to_result() }
    }
}
