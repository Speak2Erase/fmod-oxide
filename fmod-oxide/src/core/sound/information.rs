// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_int, c_uint},
    mem::MaybeUninit,
};

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};

use crate::{get_string, Sound, SoundFormat, SoundType, Tag, TimeUnit};

impl Sound {
    /// Retrieves the name of a sound.
    ///
    /// If `FMOD_LOWMEM` has been specified in `System::createSound`, this function will return "(null)".
    pub fn get_name(&self) -> Result<Utf8CString> {
        get_string(|name| unsafe {
            FMOD_Sound_GetName(
                self.inner.as_ptr(),
                name.as_mut_ptr().cast(),
                name.len() as c_int,
            )
        })
    }

    /// Returns format information about the sound.
    pub fn get_format(&self) -> Result<(SoundType, SoundFormat, c_int, c_int)> {
        let mut kind = 0;
        let mut format = 0;
        let mut channels = 0;
        let mut bits = 0;
        unsafe {
            FMOD_Sound_GetFormat(
                self.inner.as_ptr(),
                &mut kind,
                &mut format,
                &mut channels,
                &mut bits,
            )
            .to_result()?;
        }
        let kind = kind.try_into()?;
        let format = format.try_into()?;
        Ok((kind, format, channels, bits))
    }

    /// Retrieves the length using the specified time unit.
    ///
    /// A length of `0xFFFFFFFF` means it is of unlimited length, such as an internet radio stream or MOD/S3M/XM/IT file which may loop forever.
    ///
    /// Note: Using a VBR (Variable Bit Rate) source that does not have metadata containing its accurate length (such as un-tagged MP3 or MOD/S3M/XM/IT) may return inaccurate length values.
    /// For these formats, use `FMOD_ACCURATETIME` when creating the sound.
    /// This will cause a slight delay and memory increase, as FMOD will scan the whole during creation to find the correct length.
    /// This flag also creates a seek table to enable sample accurate seeking.
    pub fn get_length(&self, unit: TimeUnit) -> Result<c_uint> {
        let mut length = 0;
        unsafe {
            FMOD_Sound_GetLength(self.inner.as_ptr(), &mut length, unit.into()).to_result()?;
        }
        Ok(length)
    }

    /// Retrieves the number of metadata tags.
    ///
    /// 'Tags' are metadata stored within a sound file. These can be things like a song's name, composer etc.
    ///
    /// The second tuple field could be periodically checked to see if new tags are available in certain circumstances.
    /// This might be the case with internet based streams (i.e. shoutcast or icecast) where the name of the song or other attributes might change.
    pub fn get_tag_count(&self) -> Result<(c_int, c_int)> {
        let mut tags = 0;
        let mut updated = 0;
        unsafe {
            FMOD_Sound_GetNumTags(self.inner.as_ptr(), &mut tags, &mut updated).to_result()?;
        }
        Ok((tags, updated))
    }

    /// Retrieves a metadata tag.
    ///
    /// 'Tags' are metadata stored within a sound file. These can be things like a song's name, composer etc.
    ///
    /// The number of tags available can be found with `Sound::getNumTags`.
    ///
    /// The way to display or retrieve tags can be done in 3 different ways:
    /// - All tags can be continuously retrieved by looping from 0 to the numtags value in `Sound::getNumTags` - 1. Updated tags will refresh automatically, and the 'updated' member of the `FMOD_TAG` structure will be set to true if a tag has been updated, due to something like a netstream changing the song name for example.
    /// - Tags can be retrieved by specifying -1 as the index and only updating tags that are returned. If all tags are retrieved and this function is called the function will return an error of `FMOD_ERR_TAGNOTFOUND`.
    /// - Specific tags can be retrieved by specifying a name parameter. The index can be 0 based or -1 in the same fashion as described previously.
    ///
    /// Note with netstreams an important consideration must be made between songs, a tag may occur that changes the playback rate of the song. It is up to the user to catch this and reset the playback rate with `Channel::setFrequency`.
    /// A sample rate change will be signalled with a tag of type `FMOD_TAGTYPE_FMOD`.
    ///```rs
    /// while let Ok(tag) = sound->getTag(None, -1)
    /// {
    ///   if matches!(tag.type, TagType::Fmod) {
    ///     // When a song changes, the sample rate may also change, so compensate here.
    ///     if tag.name == "Sample Rate Change" && channel {
    ///       let TagDataType::Float(frequency) = tag.data else {
    ///         break
    ///       };
    ///       result = channel.set_frequency(frequency)?;
    ///     }
    ///   }
    /// }
    ///```
    pub fn get_tag(&self, name: Option<&Utf8CStr>, index: c_int) -> Result<Tag> {
        let mut tag = MaybeUninit::uninit();
        unsafe {
            FMOD_Sound_GetTag(
                self.inner.as_ptr(),
                name.map_or(std::ptr::null(), Utf8CStr::as_ptr),
                index,
                tag.as_mut_ptr(),
            )
            .to_result()?;

            let tag = tag.assume_init();
            let tag = Tag::from_ffi(tag);
            Ok(tag)
        }
    }
}
