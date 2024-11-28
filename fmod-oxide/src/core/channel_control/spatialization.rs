// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::c_float, mem::MaybeUninit};

use fmod_sys::*;

use crate::{ChannelControl, Vector};

impl ChannelControl {
    /// Sets the 3D position and velocity used to apply panning, attenuation and doppler.
    ///
    /// The [`Mode::D3`] flag must be set on this object otherwise [`FMOD_RESULT::FMOD_ERR_NEEDS3D`] is returned.
    ///
    /// Vectors must be provided in the correct handedness.
    ///
    /// For a stereo 3D sound, you can set the spread of the left/right parts in speaker space by using [`ChannelControl::set_3d_spread`].
    pub fn set_3d_attributes(
        &self,
        position: Option<Vector>,
        velocity: Option<Vector>,
    ) -> Result<()> {
        // vector is layout compatible with FMOD_VECTOR
        let position = position
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        let velocity = velocity
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        unsafe { FMOD_ChannelControl_Set3DAttributes(self.inner, position, velocity).to_result() }
    }

    /// Retrieves the 3D position and velocity used to apply panning, attenuation and doppler.
    pub fn get_3d_attributes(&self) -> Result<(Vector, Vector)> {
        let mut position = MaybeUninit::zeroed();
        let mut velocity = MaybeUninit::zeroed();
        unsafe {
            FMOD_ChannelControl_Get3DAttributes(
                self.inner,
                position.as_mut_ptr(),
                velocity.as_mut_ptr(),
            )
            .to_result()?;

            let position = position.assume_init().into();
            let velocity = velocity.assume_init().into();

            Ok((position, velocity))
        }
    }

    /// Sets the orientation of a 3D cone shape, used for simulated occlusion.
    ///
    /// The [`Mode::D3`] flag must be set on this object otherwise [`FMOD_RESULT::FMOD_ERR_NEEDS3D`] is returned.
    ///
    /// This function has no effect unless [`ChannelControl::set_3d_cone_settings`] has been used to change the cone inside/outside angles from the default.
    ///
    /// Vectors must be provided in the correct handedness.
    pub fn set_3d_cone_orientation(&self, mut orientation: Vector) -> Result<()> {
        // this probably doesn't need to be mutable? more safe to be mutable just in case
        let orientation = std::ptr::from_mut(&mut orientation).cast();
        unsafe { FMOD_ChannelControl_Set3DConeOrientation(self.inner, orientation).to_result() }
    }

    /// Retrieves the orientation of a 3D cone shape, used for simulated occlusion.
    pub fn get_3d_cone_orientation(&self) -> Result<Vector> {
        let mut orientation = MaybeUninit::zeroed();
        unsafe {
            FMOD_ChannelControl_Get3DConeOrientation(self.inner, orientation.as_mut_ptr())
                .to_result()?;

            let orientation = orientation.assume_init().into();

            Ok(orientation)
        }
    }

    /// Sets the angles and attenuation levels of a 3D cone shape, for simulated occlusion which is based on direction.
    ///
    /// The [`Mode::D3`] flag must be set on this object otherwise [`FMOD_RESULT::FMOD_ERR_NEEDS3D`] is returned.
    ///
    /// When [`ChannelControl::set_3d_cone_orientation`] is used and a 3D 'cone' is set up,
    /// attenuation will automatically occur for a sound based on the relative angle of the direction the cone is facing,
    /// vs the angle between the sound and the listener.
    /// - If the relative angle is within the `inside_angle`, the sound will not have any attenuation applied.
    /// - If the relative angle is between the `inside_angle` and `outside_angle`,
    ///   linear volume attenuation (between 1 and `outside_volume`) is applied between the two angles until it reaches the `outside_angle`.
    /// - If the relative angle is outside of the `outside_angle` the volume does not attenuate any further.
    pub fn set_3d_cone_settings(
        &self,
        inside_angle: c_float,
        outside_angle: c_float,
        outside_volume: c_float,
    ) -> Result<()> {
        unsafe {
            FMOD_ChannelControl_Set3DConeSettings(
                self.inner,
                inside_angle,
                outside_angle,
                outside_volume,
            )
            .to_result()
        }
    }

    /// Retrieves the angles and attenuation levels of a 3D cone shape, for simulated occlusion which is based on direction.
    ///
    /// When [`ChannelControl::set_3d_cone_orientation`] is used and a 3D 'cone' is set up,
    /// attenuation will automatically occur for a sound based on the relative angle of the direction the cone is facing,
    /// vs the angle between the sound and the listener.
    /// - If the relative angle is within the `inside_angle`, the sound will not have any attenuation applied.
    /// - If the relative angle is between the `inside_angle` and `outside_angle`,
    ///     linear volume attenuation (between 1 and `outside_volume`) is applied between the two angles until it reaches the `outside_angle`.
    /// - If the relative angle is outside of the `outside_angle` the volume does not attenuate any further.
    pub fn get_3d_cone_settings(&self) -> Result<(c_float, c_float, c_float)> {
        let mut inside_angle = 0.0;
        let mut outside_angle = 0.0;
        let mut outside_volume = 0.0;
        unsafe {
            FMOD_ChannelControl_Get3DConeSettings(
                self.inner,
                &mut inside_angle,
                &mut outside_angle,
                &mut outside_volume,
            )
            .to_result()?;
        }
        Ok((inside_angle, outside_angle, outside_volume))
    }

    /// Sets a custom roll-off shape for 3D distance attenuation.
    ///
    /// Must be used in conjunction with [`Mode::CUSTOM_ROLLOFF`] flag to be activated.
    ///
    /// If [`Mode::CUSTOM_ROLLOFF`] is set and the roll-off shape is not set, FMOD will revert to [`Mode::INVERSE_ROLLOFF`] roll-off mode.
    ///
    /// When a custom roll-off is specified a [`Channel`] or [`ChannelGroup`]'s 3D 'minimum' and 'maximum' distances are ignored.
    ///
    /// The distance in-between point values is linearly interpolated until the final point where the last value is held.
    ///
    /// If the points are not sorted by distance, an error will result.
    ///
    /// # Safety
    ///
    /// This function does not duplicate the memory for the points internally.
    /// The memory you pass to FMOD must remain valid while in use.
    pub unsafe fn set_3d_custom_rolloff(&self, points: &mut [Vector]) -> Result<()> {
        // probably doesn't need to be mutable, but more safe to be mutable just in case
        unsafe {
            FMOD_ChannelControl_Set3DCustomRolloff(
                self.inner,
                points.as_mut_ptr().cast(),
                points.len() as i32,
            )
            .to_result()
        }
    }

    /// Retrieves the current custom roll-off shape for 3D distance attenuation.
    pub fn get_3d_custom_rolloff(&self) -> Result<Vec<Vector>> {
        let mut points = std::ptr::null_mut();
        let mut num_points = 0;
        unsafe {
            FMOD_ChannelControl_Get3DCustomRolloff(self.inner, &mut points, &mut num_points)
                .to_result()?;

            let points = std::slice::from_raw_parts(points.cast(), num_points as usize).to_vec();

            Ok(points)
        }
    }

    /// Sets an override value for the 3D distance filter.
    ///
    /// If distance filtering is enabled, by default the 3D engine will automatically attenuate frequencies using a lowpass and a highpass filter, based on 3D distance.
    /// This function allows the distance filter effect to be set manually, or to be set back to 'automatic' mode.
    ///
    /// The `FMOD_3D` flag must be set on this object otherwise `FMOD_ERR_NEEDS3D` is returned.
    ///
    /// The System must be initialized with `FMOD_INIT_CHANNEL_DISTANCEFILTER` for this feature to work.
    ///
    /// #### NOTE: Currently only supported for [`Channel`], not [`ChannelGroup`].
    pub fn set_3d_distance_filter(
        &self,
        custom: bool,
        custom_level: c_float,
        center_freq: c_float,
    ) -> Result<()> {
        unsafe {
            FMOD_ChannelControl_Set3DDistanceFilter(self.inner, custom, custom_level, center_freq)
                .to_result()
        }
    }

    /// Retrieves the override values for the 3D distance filter.
    pub fn get_3d_distance_filter(&self) -> Result<(bool, c_float, c_float)> {
        let mut custom = false;
        let mut custom_level = 0.0;
        let mut center_freq = 0.0;
        unsafe {
            FMOD_ChannelControl_Get3DDistanceFilter(
                self.inner,
                &mut custom,
                &mut custom_level,
                &mut center_freq,
            )
            .to_result()?;
        }
        Ok((custom, custom_level, center_freq))
    }

    /// Sets the amount by which doppler is scaled.
    ///
    /// The `FMOD_3D` flag must be set on this object otherwise `FMOD_ERR_NEEDS3D` is returned.
    ///
    /// The doppler effect will disabled if `System::set3DNumListeners` is given a value greater than 1.
    ///
    /// #### NOTE: Currently only supported for [`Channel`], not [`ChannelGroup`].
    pub fn set_3d_doppler_level(&self, level: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_Set3DDopplerLevel(self.inner, level).to_result() }
    }

    /// Retrieves the amount by which doppler is scaled.
    pub fn get_3d_doppler_level(&self) -> Result<c_float> {
        let mut level = 0.0;
        unsafe { FMOD_ChannelControl_Get3DDopplerLevel(self.inner, &mut level).to_result()? }
        Ok(level)
    }

    /// Sets the blend between 3D panning and 2D panning.
    ///
    /// The `FMOD_3D` flag must be set on this object otherwise `FMOD_ERR_NEEDS3D` is returned.
    ///
    /// 2D functions include:
    ///
    /// - `ChannelControl::setPan`
    /// - `ChannelControl::setMixLevelsOutput`
    /// - `ChannelControl::setMixLevelsInput`
    /// - `ChannelControl::setMixMatrix`
    ///
    /// 3D functions include:
    ///
    /// - `ChannelControl::set3DAttributes`
    /// - `ChannelControl::set3DConeOrientation`
    /// - `ChannelControl::set3DCustomRolloff`
    pub fn set_3d_level(&self, level: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_Set3DLevel(self.inner, level).to_result() }
    }

    /// Retrieves the blend between 3D panning and 2D panning.
    ///
    /// The `FMOD_3D` flag must be set on this object otherwise `FMOD_ERR_NEEDS3D` is returned.
    ///
    /// 2D functions include:
    ///
    /// - `ChannelControl::setPan`
    /// - `ChannelControl::setMixLevelsOutput`
    /// - `ChannelControl::setMixLevelsInput`
    /// - `ChannelControl::setMixMatrix`
    ///
    /// 3D functions include:
    ///
    /// - `ChannelControl::set3DAttributes`
    /// - `ChannelControl::set3DConeOrientation`
    /// - `ChannelControl::set3DCustomRolloff`
    pub fn get_3d_level(&self) -> Result<c_float> {
        let mut level = 0.0;
        unsafe { FMOD_ChannelControl_Get3DLevel(self.inner, &mut level).to_result()? }
        Ok(level)
    }

    /// Sets the minimum and maximum distances used to calculate the 3D roll-off attenuation.
    ///
    /// When the listener is within the minimum distance of the sound source the 3D volume will be at its maximum. As the listener moves from the minimum distance to the maximum distance the sound will attenuate following the roll-off curve set. When outside the maximum distance the sound will no longer attenuate.
    ///
    /// Attenuation in 3D space is controlled by the roll-off mode, these are `FMOD_3D_INVERSEROLLOFF`, `FMOD_3D_LINEARROLLOFF`, `FMOD_3D_LINEARSQUAREROLLOFF`, `FMOD_3D_INVERSETAPEREDROLLOFF`, `FMOD_3D_CUSTOMROLLOFF`.
    ///
    /// Minimum distance is useful to give the impression that the sound is loud or soft in 3D space.
    /// A sound with a small 3D minimum distance in a typical (non custom) roll-off mode will make the sound appear small, and the sound will attenuate quickly.
    /// A sound with a large minimum distance will make the sound appear larger.
    ///
    /// The `FMOD_3D` flag must be set on this object otherwise `FMOD_ERR_NEEDS3D` is returned.
    ///
    /// To define the min and max distance per Sound instead of Channel or `ChannelGroup` use `Sound::set3DMinMaxDistance`.
    ///
    /// If `FMOD_3D_CUSTOMROLLOFF` has been set on this object these values are stored, but ignored in 3D processing.
    pub fn set_3d_min_max_distance(&self, min: c_float, max: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_Set3DMinMaxDistance(self.inner, min, max).to_result() }
    }

    /// Retrieves the minimum and maximum distances used to calculate the 3D roll-off attenuation.
    pub fn get_3d_min_max_distance(&self) -> Result<(c_float, c_float)> {
        let mut min = 0.0;
        let mut max = 0.0;
        unsafe {
            FMOD_ChannelControl_Get3DMinMaxDistance(self.inner, &mut min, &mut max).to_result()?;
        }
        Ok((min, max))
    }

    /// Sets the 3D attenuation factors for the direct and reverb paths.
    ///
    /// There is a reverb path/send when `ChannelControl::setReverbProperties` has been used, reverbocclusion controls its attenuation.
    ///
    /// If the System has been initialized with `FMOD_INIT_CHANNEL_DISTANCEFILTER` or
    /// `FMOD_INIT_CHANNEL_LOWPASS` the directocclusion is applied as frequency filtering rather than volume attenuation.
    pub fn set_3d_occlusion(&self, direct: c_float, reverb: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_Set3DOcclusion(self.inner, direct, reverb).to_result() }
    }

    /// Retrieves the 3D attenuation factors for the direct and reverb paths.
    pub fn get_3d_occlusion(&self) -> Result<(c_float, c_float)> {
        let mut direct = 0.0;
        let mut reverb = 0.0;
        unsafe {
            FMOD_ChannelControl_Get3DOcclusion(self.inner, &mut direct, &mut reverb).to_result()?;
        }
        Ok((direct, reverb))
    }

    /// Sets the spread of a 3D sound in speaker space.
    ///
    /// When the spread angle is 0 (default) a multi-channel signal will collapse to mono and be spatialized to a single point based on `ChannelControl::set3DAttributes` calculations.
    /// As the angle is increased, each channel within a multi-channel signal will be rotated away from that point.
    /// For 2, 4, 6, 8, and 12 channel signals, the spread is arranged from leftmost speaker to rightmost speaker intelligently,
    /// for example in 5.1 the leftmost speaker is rear left, followed by front left, center, front right then finally rear right as the rightmost speaker (LFE is not spread).
    /// For other channel counts the individual channels are spread evenly in the order of the signal.
    /// As the signal is spread the power will be preserved.
    ///
    /// For a stereo signal given different spread angles:
    /// - 0: Sound is collapsed to mono and spatialized to a single point.
    /// - 90: Left channel is rotated 45 degrees to the left compared with angle=0 and the right channel 45 degrees to the right.
    /// - 180: Left channel is rotated 90 degrees to the left compared with angle=0 and the right channel 90 degrees to the right.
    /// - 360: Left channel is rotated 180 degrees to the left and the right channel 180 degrees to the right. This means the sound is collapsed to mono and spatialized to a single point in the opposite direction compared with (angle=0).
    pub fn set_3d_spread(&self, angle: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_Set3DSpread(self.inner, angle).to_result() }
    }

    /// Retrieves the spread of a 3D sound in speaker space.
    pub fn get_3d_spread(&self) -> Result<c_float> {
        let mut angle = 0.0;
        unsafe { FMOD_ChannelControl_Get3DSpread(self.inner, &mut angle).to_result()? }
        Ok(angle)
    }
}
