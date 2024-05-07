// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int};
use std::mem::MaybeUninit;

use crate::studio::System;
use crate::{Attributes3D, Vector};

impl System {
    /// Sets the 3D attributes of the listener.
    pub fn set_listener_attributes(
        &self,
        listener: c_int,
        attributes: Attributes3D,
        attenuation_position: Option<Vector>,
    ) -> Result<()> {
        // we need to do this conversion seperately, for lifetime reasons
        let attenuation_position = attenuation_position.map(Into::into);
        unsafe {
            FMOD_Studio_System_SetListenerAttributes(
                self.inner,
                listener,
                &attributes.into(),
                attenuation_position
                    .as_ref()
                    .map_or(std::ptr::null(), std::ptr::from_ref),
            )
            .to_result()
        }
    }

    /// Retrieves listener 3D attributes.
    pub fn get_listener_attributes(&self, listener: c_int) -> Result<(Attributes3D, Vector)> {
        let mut attributes = MaybeUninit::uninit();
        let mut attenuation_position = MaybeUninit::uninit();

        unsafe {
            FMOD_Studio_System_GetListenerAttributes(
                self.inner,
                listener,
                attributes.as_mut_ptr(),
                attenuation_position.as_mut_ptr(),
            )
            .to_result()?;

            // TODO: check safety
            Ok((
                attributes.assume_init().into(),
                attenuation_position.assume_init().into(),
            ))
        }
    }

    /// Sets the listener weighting.
    ///
    /// Listener weighting is a factor which determines how much the listener influences the mix.
    /// It is taken into account for 3D panning, doppler, and the automatic distance event parameter. A listener with a weight of 0 has no effect on the mix.
    ///
    /// Listener weighting can be used to fade in and out multiple listeners.
    /// For example to do a crossfade, an additional listener can be created with a weighting of 0 that ramps up to 1 while the old listener weight is ramped down to 0.
    /// After the crossfade is finished the number of listeners can be reduced to 1 again.
    ///
    /// The sum of all the listener weights should add up to at least 1. It is a user error to set all listener weights to 0.
    pub fn set_listener_weight(&self, listener: c_int, weight: c_float) -> Result<()> {
        unsafe { FMOD_Studio_System_SetListenerWeight(self.inner, listener, weight).to_result() }
    }

    /// Retrieves listener weighting.
    pub fn get_listener_weight(&self, listener: c_int) -> Result<c_float> {
        let mut weight = 0.0;
        unsafe {
            FMOD_Studio_System_GetListenerWeight(self.inner, listener, &mut weight).to_result()?;
        }
        Ok(weight)
    }

    /// Sets the number of listeners in the 3D sound scene.
    ///
    /// If the number of listeners is set to more than 1 then FMOD uses a 'closest sound to the listener' method to determine what should be heard.
    pub fn set_listener_count(&self, amount: c_int) -> Result<()> {
        unsafe { FMOD_Studio_System_SetNumListeners(self.inner, amount).to_result() }
    }

    /// Sets the number of listeners in the 3D sound scene.
    ///
    /// If the number of listeners is set to more than 1 then FMOD uses a 'closest sound to the listener' method to determine what should be heard.
    pub fn get_listener_count(&self) -> Result<c_int> {
        let mut amount = 0;
        unsafe {
            FMOD_Studio_System_GetNumListeners(self.inner, &mut amount).to_result()?;
        }
        Ok(amount)
    }
}
