// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_float;

use fmod_sys::*;

use crate::{ChannelControl, Mode};

#[cfg(doc)]
use crate::{Channel, ChannelGroup};

impl ChannelControl {
    /// Retrieves the playing state.
    ///
    /// A [`Channel`] is considered playing after `System::playSound` or `System::playDSP`, even if it is paused.
    ///
    /// A [`ChannelGroup`] is considered playing if it has any playing [`Channel`]s.
    pub fn is_playing(&self) -> Result<bool> {
        // because we use c exports of the c++ api, we get to use bool! no fmod_bool here :3
        let mut playing = false;
        unsafe {
            FMOD_ChannelControl_IsPlaying(self.inner.as_ptr(), &mut playing).to_result()?;
        }
        Ok(playing)
    }

    /// Stops the Channel (or all [`Channel`]s in nested [`ChannelGroup`]s) from playing.
    ///
    /// This will free up internal resources for reuse by the virtual voice system.
    ///
    /// [`Channel`]s are stopped automatically when their playback position reaches the length of the Sound being played.
    /// This is not the case however if the [`Channel`] is playing a DSP or the Sound is looping,
    /// in which case the [`Channel`] will continue playing until stop is called. Once stopped,
    /// the [`Channel`] handle will become invalid and can be discarded and any API calls made with it will return [`FMOD_RESULT::FMOD_ERR_INVALID_HANDLE`].
    pub fn stop(&self) -> Result<()> {
        unsafe { FMOD_ChannelControl_Stop(self.inner.as_ptr()).to_result() }
    }

    /// Sets the paused state.
    ///
    /// Pause halts playback which effectively freezes `Channel::getPosition` and `ChannelControl::getDSPClock` values.
    ///
    /// An individual pause state is kept for each object,
    /// pausing a parent `ChannelGroup` will effectively pause this object however when queried the individual pause state is returned.
    pub fn set_paused(&self, paused: bool) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetPaused(self.inner.as_ptr(), paused).to_result() }
    }

    /// Retrieves the paused state.
    ///
    /// An individual pause state is kept for each object,
    /// a parent [`ChannelGroup`] being paused will effectively pause this object however when queried the individual pause state is returned.
    pub fn get_paused(&self) -> Result<bool> {
        let mut paused = false;
        unsafe {
            FMOD_ChannelControl_GetPaused(self.inner.as_ptr(), &mut paused).to_result()?;
        }
        Ok(paused)
    }

    /// Sets the playback mode that controls how this object behaves.
    ///
    /// Modes supported:
    ///
    /// - [`Mode::LOOP_OFF`]
    /// - [`Mode::LOOP_NORMAL`]
    /// - [`Mode::LOOP_BIDI`]
    /// - [`Mode::D2`]
    /// - [`Mode::D3`]
    /// - [`Mode::HEADRELATIVE_3D`]
    /// - [`Mode::WORLDRELATIVE_3D`]
    /// - [`Mode::INVERSE_ROLLOFF_3D`]
    /// - [`Mode::LINEAR_ROLLOFF_3D`]
    /// - [`Mode::LINEAR_SQUARE_ROLLOFF_3D`]
    /// - [`Mode::INVERSE_TAPERED_ROLLOFF_3D`]
    /// - [`Mode::CUSTOM_ROLLOFF_3D`]
    /// - [`Mode::IGNORE_GEOMETRY_3D`]
    /// - [`Mode::VIRTUAL_PLAYFROM_START`]
    ///
    /// When changing the loop mode, sounds created with
    /// `System::createStream` or [`Mode::CREATE_STREAM`] may have already been pre-buffered and executed their loop logic ahead of time before this call was even made.
    /// This is dependent on the size of the sound versus the size of the stream decode buffer (see `FMOD_CREATESOUNDEXINFO`).
    /// If this happens, you may need to reflush the stream buffer by calling `Channel::setPosition`.
    /// Note this will usually only happen if you have sounds or loop points that are smaller than the stream decode buffer size.
    ///
    /// When changing the loop mode of sounds created with with `System::createSound` or `FMOD_CREATESAMPLE`,
    /// if the sound was set up as [`Mode::LOOP_OFF`], then set to [`Mode::LOOP_NORMAL`] with this function, the sound may click when playing the end of the sound.
    /// This is because the sound needs to be prepared for looping using `Sound::setMode`,
    /// by modifying the content of the PCM data (i.e. data past the end of the actual sample data) to allow the interpolators to read ahead without clicking.
    /// If you use `ChannelControl::setMode` it will not do this (because different Channels may have different loop modes for the same sound)
    /// and may click if you try to set it to looping on an unprepared sound.
    /// If you want to change the loop mode at runtime it may be better to load the sound as looping first (or use `Sound::setMode`),
    /// to let it prepare the data as if it was looping so that it does not click whenever `ChannelControl::setMode` is used to turn looping on.
    ///
    /// If [`Mode::IGNORE_GEOMETRY_3D`] or [`Mode::VIRTUAL_PLAYFROM_START`] is not specified, the flag will be cleared if it was specified previously.
    pub fn set_mode(&self, mode: Mode) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetMode(self.inner.as_ptr(), mode.into()).to_result() }
    }

    /// Retrieves the playback mode bits that control how this object behaves.
    pub fn get_mode(&self) -> Result<Mode> {
        let mut mode = 0;
        unsafe {
            FMOD_ChannelControl_GetMode(self.inner.as_ptr(), &mut mode).to_result()?;
        }
        Ok(mode.into())
    }

    /// Sets the relative pitch / playback rate.
    ///
    /// Scales playback frequency of [`Channel`] object or if issued on a [`ChannelGroup`] it scales the frequencies of all [`Channel`]s contained in the [`ChannelGroup`].
    ///
    /// An individual pitch value is kept for each object,
    /// changing the pitch of a parent [`ChannelGroup`] will effectively alter the pitch of this object however when queried the individual pitch value is returned.
    pub fn set_pitch(&self, pitch: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetPitch(self.inner.as_ptr(), pitch).to_result() }
    }

    /// Retrieves the relative pitch / playback rate.
    ///
    /// An individual pitch value is kept for each object, a parent [`ChannelGroup`] pitch will effectively scale the pitch of this object however when queried the individual pitch value is returned.
    pub fn get_pitch(&self) -> Result<c_float> {
        let mut pitch = 0.0;
        unsafe {
            FMOD_ChannelControl_GetPitch(self.inner.as_ptr(), &mut pitch).to_result()?;
        }
        Ok(pitch)
    }
}
