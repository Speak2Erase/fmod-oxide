// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::{ffi::c_int, mem::MaybeUninit};

use crate::{ChannelGroup, PortType, ReverbProperties, System, Vector};

#[cfg(doc)]
use crate::{Dsp, OutputType};

impl System {
    /// Sets the position, velocity and orientation of the specified 3D sound listener.
    ///
    /// The forward and up vectors must be perpendicular and be of unit length (magnitude of each vector should be 1).
    ///
    /// Vectors must be provided in the correct handedness.
    ///
    /// For velocity, remember to use units per second, and not units per frame.
    /// This is a common mistake and will make the doppler effect sound wrong if velocity is based on movement per frame rather than a fixed time period.
    /// If velocity per frame is calculated, it can be converted to velocity per second by dividing it by the time taken between frames as a fraction of a second.
    /// i.e.
    ///
    /// `velocity_units_per_second = (position_currentframe - position_lastframe) / time_taken_since_last_frame_in_seconds`.
    ///
    /// At 60fps the formula would look like `velocity_units_per_second = (position_current_frame - position_last_frame) / 0.0166667`.
    ///
    /// Users of the Studio API should call [`crate::studio::System::set_listener_attributes`] instead of this function.
    pub fn set_3d_listener_attributes(
        &self,
        listener: c_int,
        position: Option<Vector>,
        velocity: Option<Vector>,
        forward: Option<Vector>,
        up: Option<Vector>,
    ) -> Result<()> {
        // these casts are ok as Vector is layout equivalent with FMOD_VECTOR
        let position = position
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        let velocity = velocity
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        let forward = forward
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        let up = up
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        unsafe {
            FMOD_System_Set3DListenerAttributes(
                self.inner.as_ptr(),
                listener,
                position,
                velocity,
                forward,
                up,
            )
            .to_result()
        }
    }

    /// Retrieves the position, velocity and orientation of the specified 3D sound listener.
    ///
    /// Users of the Studio API should call [`crate::studio::System::get_listener_attributes`] instead of this function.
    pub fn get_3d_listener_attributes(
        &self,
        listener: c_int,
    ) -> Result<(Vector, Vector, Vector, Vector)> {
        let mut position = MaybeUninit::zeroed();
        let mut velocity = MaybeUninit::zeroed();
        let mut forward = MaybeUninit::zeroed();
        let mut up = MaybeUninit::zeroed();
        unsafe {
            FMOD_System_Get3DListenerAttributes(
                self.inner.as_ptr(),
                listener,
                position.as_mut_ptr(),
                velocity.as_mut_ptr(),
                forward.as_mut_ptr(),
                up.as_mut_ptr(),
            )
            .to_result()?;

            let position = position.assume_init();
            let velocity = velocity.assume_init();
            let forward = forward.assume_init();
            let up = up.assume_init();

            Ok((position.into(), velocity.into(), forward.into(), up.into()))
        }
    }

    /// Sets parameters for the global reverb environment.
    ///
    /// To assist in defining reverb properties there are several presets available,
    /// see the associated constants on [`ReverbProperties.`].
    ///
    /// When using each instance for the first time,
    /// FMOD will create an SFX reverb [`Dsp`] unit that takes up several hundred kilobytes of memory and some CPU.
    pub fn set_reverb_properties(
        &self,
        instance: c_int,
        properties: Option<ReverbProperties>,
    ) -> Result<()> {
        let properties = properties
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        unsafe {
            FMOD_System_SetReverbProperties(self.inner.as_ptr(), instance, properties).to_result()
        }
    }

    /// Retrieves the current reverb environment for the specified reverb instance.
    pub fn get_reverb_properties(&self, instance: c_int) -> Result<ReverbProperties> {
        let mut properties = MaybeUninit::zeroed();
        unsafe {
            FMOD_System_GetReverbProperties(self.inner.as_ptr(), instance, properties.as_mut_ptr())
                .to_result()?;
            let properties = properties.assume_init().into();
            Ok(properties)
        }
    }

    /// Connect the output of the specified [`ChannelGroup`] to an audio port on the output driver.
    ///
    /// Ports are additional outputs supported by some [`OutputType`] plugins and can include things like controller headsets or dedicated background music streams.
    /// See the Port Support section (where applicable) of each platform's getting started guide found in the platform details chapter.
    pub fn attach_channel_group_to_port(
        &self,
        kind: PortType,
        index: Option<FMOD_PORT_INDEX>,
        channel_group: ChannelGroup,
        pass_through: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_System_AttachChannelGroupToPort(
                self.inner.as_ptr(),
                kind.into(),
                index.unwrap_or(FMOD_PORT_INDEX_NONE as FMOD_PORT_INDEX),
                channel_group.into(),
                pass_through.into(),
            )
            .to_result()
        }
    }

    /// Disconnect the output of the specified [`ChannelGroup`] from an audio port on the output driver.
    ///
    /// Removing a [`ChannelGroup`] from a port will reroute the audio back to the main mix.
    pub fn detach_channel_group_from_port(&self, channel_group: ChannelGroup) -> Result<()> {
        unsafe {
            FMOD_System_DetachChannelGroupFromPort(self.inner.as_ptr(), channel_group.into())
                .to_result()
        }
    }
}
