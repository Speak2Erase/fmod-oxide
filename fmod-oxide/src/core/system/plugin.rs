// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::ffi::{c_int, c_uint};

use crate::{get_string, Dsp, PluginType, System};

impl System {
    /// Specify a base search path for plugins so they can be placed somewhere else than the directory of the main executable.
    pub fn set_plugin_path(&self, path: &Utf8CStr) -> Result<()> {
        unsafe { FMOD_System_SetPluginPath(self.inner, path.as_ptr()).to_result() }
    }

    /// Loads an FMOD (DSP, Output or Codec) plugin from file.
    ///
    /// Once loaded DSP plugins can be used via `System::createDSPByPlugin`, output plugins can be use via `System::setOutputByPlugin` and codec plugins will be used automatically.
    ///
    /// When opening a file each codec tests whether it can support the file format in priority order.
    ///
    /// The format of the plugin is dependant on the operating system:
    ///  - Windows / UWP / Xbox One: .dll
    ///  - Linux / Android: .so
    ///  - Macintosh: .dylib
    ///  - PS4: .prx
    ///
    /// # Safety
    ///
    /// THIS CALLS INTO NON-RUST CODE! There is no guarantee that the plugin is safe to load, use, or unload.
    pub unsafe fn load_plugin(&self, filename: &Utf8CStr, priority: c_uint) -> Result<c_uint> {
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
        let mut version = 0;

        let name = get_string(|name| unsafe {
            FMOD_System_GetPluginInfo(
                self.inner,
                handle,
                &mut plugin_type,
                name.as_mut_ptr().cast(),
                name.len() as c_int,
                &mut version,
            )
        })?;

        let plugin_type = plugin_type.try_into()?;
        Ok((plugin_type, name, version))
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

    /// Create a DSP object given a plugin handle.
    ///
    /// A DSP object is a module that can be inserted into the mixing graph to allow sound filtering or sound generation.
    /// See the DSP architecture guide for more information.
    ///
    /// A handle can come from a newly loaded plugin with `System::loadPlugin` or an existing plugin with `System::getPluginHandle`.
    ///
    /// DSPs must be attached to the DSP graph before they become active, either via `ChannelControl::addDSP` or `DSP::addInput`.
    pub fn create_dsp_by_plugin(&self, handle: c_uint) -> Result<Dsp> {
        let mut dsp = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateDSPByPlugin(self.inner, handle, &mut dsp).to_result()?;
        }
        Ok(dsp.into())
    }

    /// Retrieve the description structure for a pre-existing DSP plugin.
    // this is safe, but dereferencing the pointer is not
    pub fn get_dsp_info_by_plugin(&self, handle: c_uint) -> Result<*const FMOD_DSP_DESCRIPTION> {
        let mut dsp_description = std::ptr::null();
        unsafe {
            FMOD_System_GetDSPInfoByPlugin(self.inner, handle, &mut dsp_description).to_result()?;
        }
        Ok(dsp_description)
    }

    /// Register a Codec plugin description structure for later use.
    ///
    /// To create an instances of this plugin use `System::createSound` and `System::createStream`.
    ///
    /// When opening a file each Codec tests whether it can support the file format in priority order.
    /// The priority for each `FMOD_SOUND_TYPE` are as follows:
    /// - `FMOD_SOUND_TYPE_FSB` : 250
    /// - `FMOD_SOUND_TYPE_XMA` : 250
    /// - `FMOD_SOUND_TYPE_AT9` : 250
    /// - `FMOD_SOUND_TYPE_VORBIS` : 250
    /// - `FMOD_SOUND_TYPE_OPUS` : 250
    /// - `FMOD_SOUND_TYPE_FADPCM` : 250
    /// - `FMOD_SOUND_TYPE_WAV` : 600
    /// - `FMOD_SOUND_TYPE_OGGVORBIS` : 800
    /// - `FMOD_SOUND_TYPE_AIFF` : 1000
    /// - `FMOD_SOUND_TYPE_FLAC` : 1100
    /// - `FMOD_SOUND_TYPE_MOD` : 1200
    /// - `FMOD_SOUND_TYPE_S3M` : 1300
    /// - `FMOD_SOUND_TYPE_XM` : 1400
    /// - `FMOD_SOUND_TYPE_IT` : 1500
    /// - `FMOD_SOUND_TYPE_MIDI` : 1600
    /// - `FMOD_SOUND_TYPE_DLS` : 1700
    /// - `FMOD_SOUND_TYPE_ASF` : 1900
    /// - `FMOD_SOUND_TYPE_AUDIOQUEUE` : 2200
    /// - `FMOD_SOUND_TYPE_MEDIACODEC` : 2250
    /// - `FMOD_SOUND_TYPE_MPEG` : 2400
    /// - `FMOD_SOUND_TYPE_PLAYLIST` : 2450
    /// - `FMOD_SOUND_TYPE_RAW` : 2500
    /// - `FMOD_SOUND_TYPE_USER` : 2600
    /// - `FMOD_SOUND_TYPE_MEDIA_FOUNDATION` : 2600
    ///
    /// XMA, AT9, Vorbis, Opus and FADPCM are supported through the FSB format, and therefore have the same priority as FSB.
    ///
    /// Media Foundation is supported through the User codec, and therefore has the same priority as User.
    ///
    /// Raw and User are only accesible if `FMOD_OPENRAW` or `FMOD_OPENUSER` is specified as the mode in `System::createSound`.
    ///
    /// # Safety
    ///
    /// This function provides no gaurdrails or safe API for registering a codec.
    /// It can call into non-rust external code.
    /// Codec descriptions are intended to be retrieved from a plugin's C API, so it's not feasible to provide a safe API for this function.
    pub unsafe fn register_codec(
        &self,
        description: *mut FMOD_CODEC_DESCRIPTION,
        priority: c_uint,
    ) -> Result<c_uint> {
        let mut handle = 0;
        unsafe {
            FMOD_System_RegisterCodec(self.inner, description, &mut handle, priority)
                .to_result()?;
        }
        Ok(handle)
    }

    /// Register a DSP plugin description structure for later use.
    ///
    /// To create an instances of this plugin use `System::createDSPByPlugin`.
    ///
    /// # Safety
    ///
    /// This function provides no gaurdrails or safe API for registering a plugin.
    /// It can call into non-rust external code.
    /// Dsp descriptions are intended to be retrieved from a plugin's C API, so it's not feasible to provide a safe API for this function.
    pub unsafe fn register_plugin(
        &self,
        dsp_description: *mut FMOD_DSP_DESCRIPTION,
    ) -> Result<c_uint> {
        let mut handle = 0;
        unsafe {
            FMOD_System_RegisterDSP(self.inner, dsp_description, &mut handle).to_result()?;
        }
        Ok(handle)
    }

    /// Register an Output plugin description structure for later use.
    ///
    /// To select this plugin for output use `System::setOutputByPlugin`.
    ///
    /// # Safety
    ///
    /// This function provides no gaurdrails or safe API for registering an output.
    /// It can call into non-rust external code.
    /// Output descriptions are intended to be retrieved from a plugin's C API, so it's not feasible to provide a safe API for this function.
    pub unsafe fn register_output(
        &self,
        description: *mut FMOD_OUTPUT_DESCRIPTION,
    ) -> Result<c_uint> {
        let mut handle = 0;
        unsafe {
            FMOD_System_RegisterOutput(self.inner, description, &mut handle).to_result()?;
        }
        Ok(handle)
    }
}
