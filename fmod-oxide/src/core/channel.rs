// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_int, c_uint},
    ops::Deref,
};

use fmod_sys::*;

use crate::{ChannelControl, ChannelGroup, TimeUnit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Channel {
    pub(crate) inner: *mut FMOD_CHANNEL,
}

unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}

impl From<*mut FMOD_CHANNEL> for Channel {
    fn from(value: *mut FMOD_CHANNEL) -> Self {
        Channel { inner: value }
    }
}

impl From<Channel> for *mut FMOD_CHANNEL {
    fn from(value: Channel) -> Self {
        value.inner
    }
}

impl Channel {
    /// Sets the frequency or playback rate.
    ///
    /// Default frequency is determined by the audio format of the Sound or DSP.
    ///
    /// Sounds opened as [`SoundMode::CreateSample`] (not [`SoundMode::CreateStream`] or [`SoundMode::CreateCompressedSample`]) can be played backwards by giving a negative frequency.
    pub fn set_frequency(&self, frequency: c_float) -> Result<()> {
        unsafe { FMOD_Channel_SetFrequency(self.inner, frequency).to_result() }
    }

    /// Retrieves the playback frequency or playback rate.
    pub fn get_frequency(&self) -> Result<c_float> {
        let mut frequency = 0.0;
        unsafe { FMOD_Channel_GetFrequency(self.inner, &mut frequency).to_result()? }
        Ok(frequency)
    }
    /// Sets the priority used for virtual voice ordering.
    ///
    /// Priority is used as a coarse grain control for the virtual voice system, lower priority [`Channel`]s will always be stolen before higher ones.
    /// For [`Channel`]s of equal priority, those with the quietest [`ChannelControl::get_audibility`] value will be stolen first.
    ///
    /// See the Virtual Voices guide for more information.
    pub fn set_priority(&self, priority: c_int) -> Result<()> {
        unsafe { FMOD_Channel_SetPriority(self.inner, priority).to_result() }
    }

    /// Retrieves the priority used for virtual voice ordering.
    ///
    /// Priority is used as a coarse grain control for the virtual voice system, lower priority [`Channel`]s will always be stolen before higher ones.
    /// For [`Channel`]s of equal priority, those with the quietest [`ChannelControl::get_audibility`] value will be stolen first.
    ///
    ///See the Virtual Voices guide for more information.
    pub fn get_priority(&self) -> Result<c_int> {
        let mut priority = 0;
        unsafe { FMOD_Channel_GetPriority(self.inner, &mut priority).to_result()? }
        Ok(priority)
    }

    /// Sets the current playback position.
    ///
    /// Certain [`TimeUnit`] types are always available: [`TimeUnit::PCM`], [`TimeUnit::PCMBytes`] and [`TimeUnit::MS`].
    /// The others are format specific such as [`TimeUnit::ModOrder`] / [`TimeUnit::ModRow`] / [`TimeUnit::ModPattern`] which is specific to files of type MOD / S3M / XM / IT.
    ///
    /// If playing a Sound created with [`System::create_stream`] or [`SoundMode::CreateStream`] changing the position may cause a slow reflush operation while the file seek and decode occurs.
    /// You can avoid this by creating the stream with [`SoundMode::Nonblocking`].
    /// This will cause the stream to go into FMOD_OPENSTATE_SETPOSITION state (see Sound::getOpenState) and Sound commands will return [`FMOD_RESULT::FMOD_ERR_NOTREADY`].
    /// [`Channel::get_position`] will also not update until this non-blocking set position operation has completed.
    ///
    /// Using a VBR source that does not have an associated seek table or seek information (such as MP3 or MOD/S3M/XM/IT) may cause inaccurate seeking if you specify [`TimeUnit::MS`] or [`TimeUnit::PCM`].
    /// If you want FMOD to create a PCM vs bytes seek table so that seeking is accurate, you will have to specify [`SoundMode::AccurrateTime`] when loading or opening the sound.
    /// This means there is a slight delay as FMOD scans the whole file when loading the sound to create this table.
    pub fn set_position(&self, position: c_uint, time_unit: TimeUnit) -> Result<()> {
        unsafe { FMOD_Channel_SetPosition(self.inner, position, time_unit.into()).to_result() }
    }

    /// Retrieves the current playback position.
    ///
    /// Certain [`TimeUnit`] types are always available: [`TimeUnit::PCM`], [`TimeUnit::PCMBytes`] and [`TimeUnit::MS`].
    /// The others are format specific such as [`TimeUnit::ModOrder`] / [`TimeUnit::ModRow`] / [`TimeUnit::ModPattern`] which is specific to files of type MOD / S3M / XM / IT.
    ///
    /// If [`TimeUnit::MS`] or [`TimeUnit::PCMBytes`] are used, the value is internally converted from [`TimeUnit::PCM`], so the retrieved value may not exactly match the set value.
    pub fn get_position(&self, time_unit: TimeUnit) -> Result<c_uint> {
        let mut position = 0;
        unsafe {
            FMOD_Channel_GetPosition(self.inner, &mut position, time_unit.into()).to_result()?;
        }
        Ok(position)
    }

    /// Sets the [`ChannelGroup`] this object outputs to.
    ///
    /// A [`ChannelGroup`] may contain many Channels.
    ///
    /// [`Channel`]s may only output to a single [`ChannelGroup`]. This operation will remove it from the previous group first.
    pub fn set_channel_group(&self, channel_group: ChannelGroup) -> Result<()> {
        unsafe { FMOD_Channel_SetChannelGroup(self.inner, channel_group.into()).to_result() }
    }

    /// Retrieves the [`ChannelGroup`] this object outputs to.
    pub fn get_channel_group(&self) -> Result<ChannelGroup> {
        let mut channel_group = std::ptr::null_mut();
        unsafe {
            FMOD_Channel_GetChannelGroup(self.inner, &mut channel_group).to_result()?;
        }
        Ok(channel_group.into())
    }

    /// Sets the number of times to loop before stopping.
    ///
    /// The 'mode' of the Sound or Channel must be [`SoundMode::LoopNormal`] or [`SoundMode::LoopBidi`] for this function to work.
    pub fn set_loop_count(&self, loop_count: c_int) -> Result<()> {
        unsafe { FMOD_Channel_SetLoopCount(self.inner, loop_count).to_result() }
    }

    /// Retrieves the number of times to loop before stopping.
    ///
    /// This is the current loop countdown value that will decrement as it plays until reaching 0.
    /// Reset with [`Channel::set_loop_count`].
    pub fn get_loop_count(&self) -> Result<c_int> {
        let mut loop_count = 0;
        unsafe { FMOD_Channel_GetLoopCount(self.inner, &mut loop_count).to_result()? }
        Ok(loop_count)
    }

    /// Sets the loop start and end points.
    ///
    /// Loop points may only be set on a Channel playing a Sound, not a Channel playing a DSP (See System::playDSP).
    ///
    /// Valid [`TimeUnit`] types are [`TimeUnit::PCM`], [`TimeUnit::MS`], [`TimeUnit::PCMBytes`]. Any other time units return [`FMOD_RESULT::FMOD_ERR_FORMAT`].
    /// If [`TimeUnit::MS`] or [`TimeUnit::PCMBytes`], the value is internally converted to [`TimeUnit::PCM`].
    ///
    /// The Channel's mode must be set to [`SoundMode::LoopNormal`] or [`SoundMode::LoopBidi`] for loop points to affect playback.
    pub fn set_loop_points(
        &self,
        start: c_uint,
        start_type: TimeUnit,
        end: c_uint,
        end_type: TimeUnit,
    ) -> Result<()> {
        unsafe {
            FMOD_Channel_SetLoopPoints(self.inner, start, start_type.into(), end, end_type.into())
                .to_result()
        }
    }

    /// Retrieves the loop start and end points.
    ///
    /// Valid [`TimeUnit`] types are [`TimeUnit::PCM`], [`TimeUnit::MS`], [`TimeUnit::PCMBytes`]. Any other time units return [`FMOD_RESULT::FMOD_ERR_FORMAT`].
    /// If [`TimeUnit::MS`] or [`TimeUnit::PCMBytes`] are used, the value is internally converted from [`TimeUnit::PCM`], so the retrieved value may not exactly match the set value.
    pub fn get_loop_points(
        &self,
        start_type: TimeUnit,
        end_type: TimeUnit,
    ) -> Result<(c_uint, c_uint)> {
        let mut start = 0;
        let mut end = 0;
        unsafe {
            FMOD_Channel_GetLoopPoints(
                self.inner,
                &mut start,
                start_type.into(),
                &mut end,
                end_type.into(),
            )
            .to_result()?;
        }
        Ok((start, end))
    }
}

impl Deref for Channel {
    type Target = ChannelControl;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        unsafe {
            // perform a debug check to ensure that the the cast results in the same pointer
            let control = FMOD_Channel_CastToControl(self.inner);
            assert_eq!(
                control as usize, self.inner as usize,
                "ChannelControl cast was not equivalent! THIS IS A MAJOR BUG. PLEASE REPORT THIS!"
            );
        }
        // channelcontrol has the same layout as channel, and if the assumption in channel_control.rs is correct,
        // this is cast is safe.
        unsafe { &*std::ptr::from_ref(self).cast::<ChannelControl>() }
    }
}
