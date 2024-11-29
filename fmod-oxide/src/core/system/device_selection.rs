// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::{ffi::c_int, mem::MaybeUninit};

use crate::{get_string, Guid, OutputType, SpeakerMode, System};

#[cfg(doc)]
use crate::SystemBuilder;

impl System {
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
        unsafe { FMOD_System_SetOutput(self.inner.as_ptr(), output_type.into()).to_result() }
    }

    /// Retrieves the type of output interface used to run the mixer.
    pub fn get_output_type(&self) -> Result<OutputType> {
        let mut output_type = 0;
        unsafe {
            FMOD_System_GetOutput(self.inner.as_ptr(), &mut output_type).to_result()?;
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
            FMOD_System_GetNumDrivers(self.inner.as_ptr(), &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves identification information about a sound device specified by its index, and specific to the selected output mode.
    pub fn get_driver_info(
        &self,
        id: c_int,
    ) -> Result<(Utf8CString, Guid, c_int, SpeakerMode, c_int)> {
        unsafe {
            let mut guid = MaybeUninit::zeroed();
            let mut system_rate = 0;
            let mut speaker_mode = 0;
            let mut speaker_mode_channels = 0;

            let name = get_string(|name| {
                FMOD_System_GetDriverInfo(
                    self.inner.as_ptr(),
                    id,
                    name.as_mut_ptr().cast(),
                    name.len() as c_int,
                    guid.as_mut_ptr(),
                    &mut system_rate,
                    &mut speaker_mode,
                    &mut speaker_mode_channels,
                )
            })?;

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
        unsafe { FMOD_System_SetDriver(self.inner.as_ptr(), driver).to_result() }
    }

    /// Retrieves the output driver for the selected output type.
    pub fn get_driver(&self) -> Result<c_int> {
        let mut driver = 0;
        unsafe {
            FMOD_System_GetDriver(self.inner.as_ptr(), &mut driver).to_result()?;
        }
        Ok(driver)
    }
}
