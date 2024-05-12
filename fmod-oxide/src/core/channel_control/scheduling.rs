// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_ulonglong;

use fmod_sys::*;

use crate::ChannelControl;

impl ChannelControl {
    /// Retrieves the DSP clock values at this point in time.
    ///
    /// To perform sample accurate scheduling in conjunction with ChannelControl::setDelay and ChannelControl::addFadePoint query the parentclock value.
    pub fn get_dsp_clock(&self) -> Result<(c_ulonglong, c_ulonglong)> {
        let mut dsp_clock = 0;
        let mut parent_clock = 0;
        unsafe {
            FMOD_ChannelControl_GetDSPClock(self.inner, &mut dsp_clock, &mut parent_clock)
                .to_result()?;
        }
        Ok((dsp_clock, parent_clock))
    }

    /// Sets a sample accurate start (and/or stop) time relative to the parent ChannelGroup DSP clock.
    ///
    /// To perform sample accurate scheduling use ChannelControl::getDSPClock to query the parent clock value.
    /// If a parent ChannelGroup changes its pitch, the start and stop times will still be correct as the parent clock rate is adjusted by that pitch.
    pub fn set_delay(
        &self,
        start: c_ulonglong,
        end: c_ulonglong,
        stop_channels: bool,
    ) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetDelay(self.inner, start, end, stop_channels).to_result() }
    }

    /// Retrieves a sample accurate start (and/or stop) time relative to the parent ChannelGroup DSP clock.
    pub fn get_delay(&self) -> Result<(c_ulonglong, c_ulonglong, bool)> {
        let mut dsp_start = 0;
        let mut dsp_end = 0;
        let mut stop_channels = false;
        unsafe {
            FMOD_ChannelControl_GetDelay(
                self.inner,
                &mut dsp_start,
                &mut dsp_end,
                &mut stop_channels,
            )
            .to_result()?;
        }
        Ok((dsp_start, dsp_end, stop_channels))
    }

    /// Adds a sample accurate fade point at a time relative to the parent ChannelGroup DSP clock.
    ///
    /// Fade points are scaled against other volume settings and in-between each fade point the volume will be linearly ramped.
    ///
    /// To perform sample accurate fading use ChannelControl::getDSPClock to query the parent clock value.
    /// If a parent ChannelGroup changes its pitch, the fade points will still be correct as the parent clock rate is adjusted by that pitch.
    ///
    /// ```rs
    /// // Example. Ramp from full volume to half volume over the next 4096 samples
    /// let (_, parent) = target.get_dsp_clock();
    /// target.add_fade_point(parent, 1.0);
    /// target.add_fade_point(parent + 4096, 0.5);
    /// ```
    pub fn add_fade_point(&self, dsp_clock: c_ulonglong, volume: f32) -> Result<()> {
        unsafe { FMOD_ChannelControl_AddFadePoint(self.inner, dsp_clock, volume).to_result() }
    }

    /// Adds a volume ramp at the specified time in the future using fade points.
    ///
    /// This is a convenience function that creates a scheduled 64 sample fade point ramp from the current volume level to volume arriving at `dsp_clock` time.
    ///
    /// Can be use in conjunction with ChannelControl::SetDelay.
    ///
    /// All fade points after `dsp_clock` will be removed.
    pub fn set_fade_point_ramp(&self, dsp_clock: c_ulonglong, volume: f32) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetFadePointRamp(self.inner, dsp_clock, volume).to_result() }
    }

    /// Removes all fade points between the two specified clock values (inclusive).
    pub fn remove_fade_points(
        &self,
        dsp_clock_start: c_ulonglong,
        dsp_clock_end: c_ulonglong,
    ) -> Result<()> {
        unsafe {
            FMOD_ChannelControl_RemoveFadePoints(self.inner, dsp_clock_start, dsp_clock_end)
                .to_result()
        }
    }

    /// Retrieves information about stored fade points.
    pub fn get_fade_points(&self) -> Result<(Vec<c_ulonglong>, Vec<f32>)> {
        let mut num_points = 0;
        unsafe {
            FMOD_ChannelControl_GetFadePoints(
                self.inner,
                &mut num_points,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
            .to_result()?;
        }

        let mut dsp_clocks = vec![0; num_points as usize];
        let mut volumes = vec![0.0; num_points as usize];
        unsafe {
            FMOD_ChannelControl_GetFadePoints(
                self.inner,
                &mut num_points,
                dsp_clocks.as_mut_ptr(),
                volumes.as_mut_ptr(),
            )
            .to_result()?;
        }
        Ok((dsp_clocks, volumes))
    }
}
