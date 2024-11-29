// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_uint},
    mem::MaybeUninit,
};

use fmod_sys::*;

use crate::studio::EventInstance;
use crate::Attributes3D;

impl EventInstance {
    /// Sets the 3D attributes.
    ///
    /// An event's 3D attributes specify its position, velocity and orientation.
    /// The 3D attributes are used to calculate 3D panning, doppler and the values of automatic distance and angle parameters.
    pub fn set_3d_attributes(&self, attributes: Attributes3D) -> Result<()> {
        let mut attributes = attributes.into();
        unsafe {
            // FIXME is this supposed to take an &mut
            FMOD_Studio_EventInstance_Set3DAttributes(self.inner.as_ptr(), &mut attributes)
                .to_result()
        }
    }

    /// Retrieves the 3D attributes.
    pub fn get_3d_attributes(&self) -> Result<Attributes3D> {
        let mut attributes = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventInstance_Get3DAttributes(self.inner.as_ptr(), attributes.as_mut_ptr())
                .to_result()?;

            let attributes = attributes.assume_init().into();
            Ok(attributes)
        }
    }

    /// Sets the listener mask.
    ///
    /// The listener mask controls which listeners are considered when calculating 3D panning and the values of listener relative automatic parameters.
    ///
    /// To create the mask you must perform bitwise OR and shift operations, the basic form is 1 << `listener_index` or'd together with other required listener indices.
    /// For example to create a mask for listener index `0` and `2` the calculation would be `mask = (1 << 0) | (1 << 2)`, to include all listeners use the default mask of `0xFFFFFFFF`.
    pub fn set_listener_mask(&self, mask: c_uint) -> Result<()> {
        unsafe { FMOD_Studio_EventInstance_SetListenerMask(self.inner.as_ptr(), mask).to_result() }
    }

    /// Retrieves the listener mask.
    pub fn get_listener_mask(&self) -> Result<c_uint> {
        let mut mask = 0;
        unsafe {
            FMOD_Studio_EventInstance_GetListenerMask(self.inner.as_ptr(), &mut mask)
                .to_result()?;
        }
        Ok(mask)
    }

    /// Retrieves the minimum and maximum distances for 3D attenuation.
    pub fn get_min_max_distance(&self) -> Result<(c_float, c_float)> {
        let mut min = 0.0;
        let mut max = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetMinMaxDistance(self.inner.as_ptr(), &mut min, &mut max)
                .to_result()?;
        }
        Ok((min, max))
    }
}
