// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CStr;
use std::ffi::c_int;

use crate::{Channel, ChannelGroup, Dsp, DspType, Mode, Reverb3D, Sound, SoundGroup, System};

impl System {
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
        mode: Mode,
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
        mode: Mode,
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
}
