// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CStr;
use std::mem::MaybeUninit;

use crate::Guid;

use crate::studio::{AdvancedSettings, Bus, EventDescription, SoundInfo, System, Vca};

impl System {
    /// Retrieves a loaded [`Bus`].
    ///
    /// This function allows you to retrieve a handle for any bus in the global mixer.
    ///
    /// `path_or_id` may be a path, such as `bus:/SFX/Ambience`, or an ID string, such as `{d9982c58-a056-4e6c-b8e3-883854b4bffb}`.
    ///
    /// Note that path lookups will only succeed if the strings bank has been loaded.
    pub fn get_bus(&self, path_or_id: &Utf8CStr) -> Result<Bus> {
        let mut bus = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetBus(self.inner.as_ptr(), path_or_id.as_ptr(), &mut bus)
                .to_result()?;
        }
        Ok(bus.into())
    }

    /// Retrieves a loaded [`Bus`].
    ///
    /// This function allows you to retrieve a handle for any bus in the global mixer.
    pub fn get_bus_by_id(&self, id: Guid) -> Result<Bus> {
        let mut bus = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetBusByID(self.inner.as_ptr(), &id.into(), &mut bus).to_result()?;
        }
        Ok(bus.into())
    }

    /// Retrieves an [`EventDescription`].
    ///
    /// This function allows you to retrieve a handle to any loaded event description.
    ///
    /// `path+or_id` may be a path, such as `event:/UI/Cancel` or `snapshot:/IngamePause`, or an ID string, such as `{2a3e48e6-94fc-4363-9468-33d2dd4d7b00}`.
    ///
    /// Note that path lookups will only succeed if the strings bank has been loaded.
    pub fn get_event(&self, path_or_id: &Utf8CStr) -> Result<EventDescription> {
        let mut event = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetEvent(self.inner.as_ptr(), path_or_id.as_ptr(), &mut event)
                .to_result()?;
            Ok(EventDescription::from(event))
        }
    }

    /// Retrieves an [`EventDescription`].
    ///
    /// This function allows you to retrieve a handle to any loaded event description.
    pub fn get_event_by_id(&self, id: Guid) -> Result<EventDescription> {
        let mut event = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetEventByID(self.inner.as_ptr(), &id.into(), &mut event)
                .to_result()?;
            Ok(EventDescription::from(event))
        }
    }

    /// Retrieves a loaded VCA.
    ///
    /// This function allows you to retrieve a handle for any VCA in the global mixer.
    ///
    /// `path_or_id` may be a path, such as `vca:/MyVCA`, or an ID string, such as `{d9982c58-a056-4e6c-b8e3-883854b4bffb`}.
    ///
    /// Note that path lookups will only succeed if the strings bank has been loaded.
    pub fn get_vca(&self, path_or_id: &Utf8CStr) -> Result<Vca> {
        let mut vca = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetVCA(self.inner.as_ptr(), path_or_id.as_ptr(), &mut vca)
                .to_result()?;
        }
        Ok(vca.into())
    }

    /// Retrieves a loaded VCA.
    ///
    /// This function allows you to retrieve a handle for any VCA in the global mixer.
    pub fn get_vca_by_id(&self, id: Guid) -> Result<Vca> {
        let mut vca = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetVCAByID(self.inner.as_ptr(), &id.into(), &mut vca).to_result()?;
        }
        Ok(vca.into())
    }

    /// Retrieves advanced settings.
    pub fn get_advanced_settings(&self) -> Result<AdvancedSettings> {
        let mut advanced_settings = MaybeUninit::zeroed();

        unsafe {
            FMOD_Studio_System_GetAdvancedSettings(
                self.inner.as_ptr(),
                advanced_settings.as_mut_ptr(),
            )
            .to_result()?;

            let advanced_settings = AdvancedSettings::from_ffi(advanced_settings.assume_init());

            Ok(advanced_settings)
        }
    }

    /// Retrieves information for loading a sound from the audio table.
    ///
    /// The [`SoundInfo`] structure contains information to be passed to [`crate::System::create_sound`] (which will create a parent sound),
    /// along with a subsound index to be passed to [`crate::Sound::get_sub_sound`] once the parent sound is loaded.
    ///
    /// The user is expected to call [`crate::System::create_sound`] with the given information.
    /// It is up to the user to combine in any desired loading flags, such as [`FMOD_CREATESTREAM`], [`FMOD_CREATECOMPRESSEDSAMPLE`] or [`FMOD_NONBLOCKING`] with the flags in [`FMOD_STUDIO_SOUND_INFO::mode`].
    ///
    /// When the banks have been loaded via [`System::load_bank_memory`], the mode will be returned as [`FMOD_OPENMEMORY_POINT`].
    /// This won't work with the default [`FMOD_CREATESAMPLE`] mode.
    /// For memory banks, you should add in the [`FMOD_CREATECOMPRESSEDSAMPLE`] or [`FMOD_CREATESTREAM`] flag, or remove [`FMOD_OPENMEMORY_POINT`] and add [`FMOD_OPENMEMORY`] to decompress the sample into a new allocation.
    ///
    /// # Safety
    ///
    /// The returned [`SoundInfo`] structure has an unbounded lifetime as it is hard to represent. You MUST constrain its lifetime as quickly as possible.
    pub unsafe fn get_sound_info<'a>(&self, key: &Utf8CStr) -> Result<SoundInfo<'a>> {
        let mut sound_info = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetSoundInfo(
                self.inner.as_ptr(),
                key.as_ptr(),
                sound_info.as_mut_ptr(),
            )
            .to_result()?;

            let sound_info = SoundInfo::from_ffi(sound_info.assume_init());
            Ok(sound_info)
        }
    }
}
