// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::ffi::c_int;

use crate::System;

impl System {
    /// Set a proxy server to use for all subsequent internet connections.
    ///
    /// Specify the proxy in `host:port` format e.g. `www.fmod.com:8888` (defaults to port 80 if no port is specified).
    ///
    /// Basic authentication is supported using `user:password@host:port` format e.g. `bob:sekrit123@www.fmod.com:8888`
    pub fn set_network_proxy(&self, proxy: &Utf8CStr) -> Result<()> {
        unsafe { FMOD_System_SetNetworkProxy(self.inner, proxy.as_ptr()).to_result() }
    }

    /// Retrieves the URL of the proxy server used in internet streaming.
    pub fn get_network_proxy(&self) -> Result<Utf8CString> {
        let mut proxy = [0; 512];

        unsafe {
            FMOD_System_GetNetworkProxy(self.inner, proxy.as_mut_ptr(), 512).to_result()?;

            // FIXME is this right?
            let name = proxy
                .into_iter()
                .take_while(|&v| v != 0)
                .map(|v| v as u8)
                .collect();
            let name = Utf8CString::from_utf8_with_nul_unchecked(name);
            Ok(name)
        }
    }

    /// Set the timeout for network streams.
    pub fn set_network_timeout(&self, timeout: c_int) -> Result<()> {
        unsafe { FMOD_System_SetNetworkTimeout(self.inner, timeout).to_result() }
    }

    /// Retrieve the timeout value for network streams.
    pub fn get_network_timeout(&self) -> Result<c_int> {
        let mut timeout = 0;
        unsafe {
            FMOD_System_GetNetworkTimeout(self.inner, &mut timeout).to_result()?;
        }
        Ok(timeout)
    }
}
