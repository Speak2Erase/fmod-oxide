// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::ffi::{c_int, c_uint};

use crate::{get_string, PluginType, System};

impl System {
    /// Specify a base search path for plugins so they can be placed somewhere else than the directory of the main executable.
    pub fn set_plugin_path(&self, path: &Utf8CStr) -> Result<()> {
        unsafe { FMOD_System_SetPluginPath(self.inner, path.as_ptr()).to_result() }
    }

    /// Loads an FMOD (DSP, Output or Codec) plugin from file.
    ///
    /// Once loaded DSP plugins can be used via System::createDSPByPlugin, output plugins can be use via System::setOutputByPlugin and codec plugins will be used automatically.
    ///
    /// When opening a file each codec tests whether it can support the file format in priority order.
    ///
    /// The format of the plugin is dependant on the operating system:
    ///  - Windows / UWP / Xbox One: .dll
    ///  - Linux / Android: .so
    ///  - Macintosh: .dylib
    ///  - PS4: .prx
    // FIXME do we mark this as unsafe? it is loading arbitrary code
    pub fn load_plugin(&self, filename: &Utf8CStr, priority: c_uint) -> Result<c_uint> {
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

    // TODO create dsp stuff
    // TODO register codec
    // TODO register dsp
    // TODO register output
}
