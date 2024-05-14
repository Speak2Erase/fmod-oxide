// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_int, c_uint},
    mem::MaybeUninit,
};

use fmod_sys::*;
use lanyard::Utf8CString;

use crate::{get_string, DriverState, Guid, Sound, SpeakerMode, System};

impl System {
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

    /// Retrieves identification information about an audio device specified by its index, and specific to the output mode.
    pub fn get_record_driver_info(
        &self,
        id: c_int,
    ) -> Result<(Utf8CString, Guid, c_int, SpeakerMode, c_int, DriverState)> {
        let mut guid = MaybeUninit::zeroed();
        let mut system_rate = 0;
        let mut speaker_mode = 0;
        let mut speaker_mode_channels = 0;
        let mut state = 0;
        let name = get_string(|name| unsafe {
            FMOD_System_GetRecordDriverInfo(
                self.inner,
                id,
                name.as_mut_ptr().cast(),
                name.len() as c_int,
                guid.as_mut_ptr(),
                &mut system_rate,
                &mut speaker_mode,
                &mut speaker_mode_channels,
                &mut state,
            )
        })?;
        unsafe {
            let guid = guid.assume_init().into();
            let speaker_mode = speaker_mode.try_into()?;
            let state = state.into();
            Ok((
                name,
                guid,
                system_rate,
                speaker_mode,
                speaker_mode_channels,
                state,
            ))
        }
    }

    /// Retrieves the current recording position of the record buffer in PCM samples.
    ///
    /// Will return [`FMOD_RESULT::FMOD_ERR_RECORD_DISCONNECTED`] if the driver is unplugged.
    ///
    /// The position will return to 0 when [`System::record_stop`] is called or when a non-looping recording reaches the end.
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
    /// Raw PCM data can be accessed with Sound::lock, Sound::unlock and [`System::get_record_position`].
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
}
