// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int, c_uint};

use crate::{Mode, Sound, TimeUnit, Vector};

impl Sound {
    /// Sets the angles and attenuation levels of a 3D cone shape, for simulated occlusion which is based on direction.
    ///
    /// When `ChannelControl::set3DConeOrientation` is used and a 3D 'cone' is set up,
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
            FMOD_Sound_Set3DConeSettings(self.inner, inside_angle, outside_angle, outside_volume)
                .to_result()
        }
    }

    /// Retrieves the inside and outside angles of the 3D projection cone and the outside volume.
    pub fn get_3d_cone_settings(&self) -> Result<(c_float, c_float, c_float)> {
        let mut inside_angle = 0.0;
        let mut outside_angle = 0.0;
        let mut outside_volume = 0.0;
        unsafe {
            FMOD_Sound_Get3DConeSettings(
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
            FMOD_Sound_Set3DCustomRolloff(
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
            FMOD_Sound_Get3DCustomRolloff(self.inner, &mut points, &mut num_points).to_result()?;

            let points = std::slice::from_raw_parts(points.cast(), num_points as usize).to_vec();

            Ok(points)
        }
    }

    /// Sets the minimum and maximum audible distance for a 3D sound.
    ///
    /// The distances are meant to simulate the 'size' of a sound. Reducing the min distance will mean the sound appears smaller in the world, and in some modes makes the volume attenuate faster as the listener moves away from the sound.
    /// Increasing the min distance simulates a larger sound in the world, and in some modes makes the volume attenuate slower as the listener moves away from the sound.
    ///
    /// max will affect attenuation differently based on roll-off mode set in the mode parameter of `System::createSound`, `System::createStream`, `Sound::setMode` or `ChannelControl::setMode`.
    ///
    /// For these modes the volume will attenuate to 0 volume (silence), when the distance from the sound is equal to or further than the max distance:
    /// - `FMOD_3D_LINEARROLLOFF`
    /// - `FMOD_3D_LINEARSQUAREROLLOFF`
    ///
    /// For these modes the volume will stop attenuating at the point of the max distance, without affecting the rate of attenuation:
    /// - `FMOD_3D_INVERSEROLLOFF`
    /// - `FMOD_3D_INVERSETAPEREDROLLOFF`
    ///
    /// For this mode the max distance is ignored:
    /// - `FMOD_3D_CUSTOMROLLOFF`
    pub fn set_3d_min_max_distance(&self, min: c_float, max: c_float) -> Result<()> {
        unsafe { FMOD_Sound_Set3DMinMaxDistance(self.inner, min, max).to_result() }
    }

    /// Retrieve the minimum and maximum audible distance for a 3D sound.
    pub fn get_3d_min_max_distance(&self) -> Result<(c_float, c_float)> {
        let mut min = 0.0;
        let mut max = 0.0;
        unsafe {
            FMOD_Sound_Get3DMinMaxDistance(self.inner, &mut min, &mut max).to_result()?;
        }
        Ok((min, max))
    }

    /// Sets a sound's default playback attributes.
    ///
    /// When the Sound is played it will use these values without having to specify them later on a per Channel basis.
    pub fn set_defaults(&self, frequency: c_float, priority: c_int) -> Result<()> {
        unsafe { FMOD_Sound_SetDefaults(self.inner, frequency, priority).to_result() }
    }

    /// Retrieves a sound's default playback attributes.
    pub fn get_defaults(&self) -> Result<(c_float, c_int)> {
        let mut frequency = 0.0;
        let mut priority = 0;
        unsafe {
            FMOD_Sound_GetDefaults(self.inner, &mut frequency, &mut priority).to_result()?;
        }
        Ok((frequency, priority))
    }

    /// Sets or alters the mode of a sound.
    ///
    /// When calling this function, note that it will only take effect when the sound is played again with `System::playSound`.
    /// This is the default for when the sound next plays, not a mode that will suddenly change all currently playing instances of this sound.
    ///
    /// Flags supported:
    /// - [`Mode::LOOP_OFF`]
    /// - [`Mode::LOOP_NORMAL`]
    /// - [`Mode::LOOP_BIDI`]
    /// - [`Mode::HEADRELATIVE_3D`]
    /// - [`Mode::WORLDRELATIVE_3D`]
    /// - [`Mode::D2`]
    /// - [`Mode::D3`]
    /// - [`Mode::INVERSE_ROLLOFF_3D`]
    /// - [`Mode::LINEAR_ROLLOFF_3D`]
    /// - [`Mode::LINEAR_SQUARE_ROLLOFF_3D`]
    /// - [`Mode::INVERSE_TAPERED_ROLLOFF_3D`]
    /// - [`Mode::CUSTOM_ROLLOFF_3D`]
    /// - [`Mode::IGNORE_GEOMETRY_3D`]
    ///
    /// If [`Mode::IGNORE_GEOMETRY_3D`] is not specified, the flag will be cleared if it was specified previously.
    ///
    /// Changing mode on an already buffered stream may not produced desired output. See Streaming Issues.
    // FIXME this is pretty unsafe, add safe version
    pub fn set_mode(&self, mode: Mode) -> Result<()> {
        unsafe { FMOD_Sound_SetMode(self.inner, mode.bits()).to_result() }
    }

    /// Retrieves the mode of a sound.
    ///
    /// The mode will be dependent on the mode set by a call to `System::createSound`, `System::createStream` or [`Sound::set_mode`].
    pub fn get_mode(&self) -> Result<Mode> {
        let mut mode = 0;
        unsafe {
            FMOD_Sound_GetMode(self.inner, &mut mode).to_result()?;
        }
        Ok(Mode::from(mode))
    }

    /// Sets the sound to loop a specified number of times before stopping if the playback mode is set to looping.
    ///
    /// Changing loop count on an already buffered stream may not produced desired output. See Streaming Issues.
    pub fn set_loop_count(&self, loop_count: c_int) -> Result<()> {
        unsafe { FMOD_Sound_SetLoopCount(self.inner, loop_count).to_result() }
    }

    /// Retrieves the sound's loop count.
    ///
    /// Unlike the Channel loop count function, this function simply returns the value set with `Sound::setLoopCount`.
    /// It does not decrement as it plays (especially seeing as one sound can be played multiple times).
    pub fn get_loop_count(&self) -> Result<c_int> {
        let mut loop_count = 0;
        unsafe {
            FMOD_Sound_GetLoopCount(self.inner, &mut loop_count).to_result()?;
        }
        Ok(loop_count)
    }

    /// Sets the loop points within a sound.
    ///
    /// The values used for `loop_start` and loopend are inclusive, which means these positions will be played.
    ///
    /// If a `loop_end` is smaller or equal to loopstart an error will be returned.
    /// The same will happen for any values that are equal or greater than the length of the sound.
    ///
    /// Changing loop points on an already buffered stream may not produced desired output. See Streaming Issues.
    ///
    /// The Sound's mode must be set to [`Mode::LOOP_NORMAL`] or [`Mode::LOOP_BIDI`] for loop points to affect playback.
    pub fn set_loop_points(
        &self,
        loop_start: c_uint,
        start_type: TimeUnit,
        loop_end: c_uint,
        end_type: TimeUnit,
    ) -> Result<()> {
        unsafe {
            FMOD_Sound_SetLoopPoints(
                self.inner,
                loop_start,
                start_type.into(),
                loop_end,
                end_type.into(),
            )
            .to_result()
        }
    }

    /// Retrieves the loop points for a sound.
    ///
    /// The values returned are inclusive, which means these positions will be played.
    pub fn get_loop_points(
        &self,
        start_type: TimeUnit,
        end_type: TimeUnit,
    ) -> Result<(c_uint, c_uint)> {
        let mut loop_start = 0;
        let mut loop_end = 0;
        unsafe {
            FMOD_Sound_GetLoopPoints(
                self.inner,
                &mut loop_start,
                start_type.into(),
                &mut loop_end,
                end_type.into(),
            )
            .to_result()?;
        }
        Ok((loop_start, loop_end))
    }
}
