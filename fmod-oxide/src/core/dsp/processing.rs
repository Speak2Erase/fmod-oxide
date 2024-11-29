// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_float;

use crate::Dsp;

impl Dsp {
    /// Sets the processing active state.
    ///
    /// If active is false, processing of this unit and its inputs are stopped.
    ///
    /// When created a [`Dsp`] is inactive. If `ChannelControl::addDSP` is used it will automatically be activated, otherwise it must be set to active manually.
    pub fn set_active(&self, active: bool) -> Result<()> {
        unsafe { FMOD_DSP_SetActive(self.inner.as_ptr(), active.into()).to_result() }
    }

    /// Retrieves the processing active state.
    ///
    /// If active is False, processing of this unit and its inputs are stopped.
    ///
    /// When created a [`Dsp`] is inactive.
    /// If `ChannelControl::addDSP` is used it will automatically be activated, otherwise it must be set to active manually.
    pub fn get_active(&self) -> Result<bool> {
        let mut active = FMOD_BOOL::FALSE;
        unsafe { FMOD_DSP_GetActive(self.inner.as_ptr(), &mut active).to_result()? };
        Ok(active.into())
    }

    /// Sets the processing bypass state.
    ///
    /// If `bypass` is true, processing of this unit is skipped but it continues to process its inputs.
    pub fn set_bypass(&self, bypass: bool) -> Result<()> {
        unsafe { FMOD_DSP_SetBypass(self.inner.as_ptr(), bypass.into()).to_result() }
    }

    /// Retrieves the processing bypass state.
    ///
    /// If `bypass` is true, processing of this unit is skipped but it continues to process its inputs.
    pub fn get_bypass(&self) -> Result<bool> {
        let mut bypass = FMOD_BOOL::FALSE;
        unsafe { FMOD_DSP_GetBypass(self.inner.as_ptr(), &mut bypass).to_result()? };
        Ok(bypass.into())
    }

    /// Sets the scale of the wet and dry signal components.
    ///
    /// The dry signal path is silent by default, because dsp effects transform the input and pass the newly processed result to the output.
    pub fn set_wet_dry_mix(&self, pre_wet: c_float, post_wet: c_float, dry: c_float) -> Result<()> {
        unsafe { FMOD_DSP_SetWetDryMix(self.inner.as_ptr(), pre_wet, post_wet, dry).to_result() }
    }

    /// Retrieves the scale of the wet and dry signal components.
    pub fn get_wet_dry_mix(&self) -> Result<(c_float, c_float, c_float)> {
        let mut pre_wet = 0.0;
        let mut post_wet = 0.0;
        let mut dry = 0.0;
        unsafe {
            FMOD_DSP_GetWetDryMix(self.inner.as_ptr(), &mut pre_wet, &mut post_wet, &mut dry)
                .to_result()?;
        }
        Ok((pre_wet, post_wet, dry))
    }

    /// Retrieves the idle state.
    ///
    /// A [`Dsp`] is considered idle when it stops receiving input signal and all internal processing of stored input has been exhausted.
    ///
    /// Each [`Dsp`] type has the potential to have differing idle behaviour based on the type of effect.
    /// A reverb or echo may take a longer time to go idle after it stops receiving a valid signal, compared to an effect with a shorter tail length like an EQ filter.
    pub fn get_idle(&self) -> Result<bool> {
        let mut idle = FMOD_BOOL::FALSE;
        unsafe { FMOD_DSP_GetIdle(self.inner.as_ptr(), &mut idle).to_result()? };
        Ok(idle.into())
    }
}
