// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::mem::MaybeUninit;

use crate::{Geometry, Vector};

impl Geometry {
    /// Sets the 3D position of the object.
    ///
    /// Position is in world space.
    pub fn set_position(&self, position: Vector) -> Result<()> {
        unsafe {
            FMOD_Geometry_SetPosition(self.inner.as_ptr(), std::ptr::from_ref(&position).cast())
                .to_result()
        }
    }

    /// Retrieves the 3D position of the object.
    ///
    /// Position is in world space.
    pub fn get_position(&self) -> Result<Vector> {
        let mut position = MaybeUninit::uninit();
        unsafe {
            FMOD_Geometry_GetPosition(self.inner.as_ptr(), position.as_mut_ptr()).to_result()?;
            let position = position.assume_init().into();
            Ok(position)
        }
    }

    /// Sets the 3D orientation of the object.
    ///
    /// See remarks in [`crate::System::set_3d_listener_attributes`] for more description on forward and up vectors.
    pub fn set_rotation(&self, forward: Vector, up: Vector) -> Result<()> {
        unsafe {
            FMOD_Geometry_SetRotation(
                self.inner.as_ptr(),
                std::ptr::from_ref(&forward).cast(),
                std::ptr::from_ref(&up).cast(),
            )
            .to_result()
        }
    }

    /// Retrieves the 3D orientation of the object.
    pub fn get_rotation(&self) -> Result<(Vector, Vector)> {
        let mut forward = MaybeUninit::uninit();
        let mut up = MaybeUninit::uninit();
        unsafe {
            FMOD_Geometry_GetRotation(self.inner.as_ptr(), forward.as_mut_ptr(), up.as_mut_ptr())
                .to_result()?;
            let forward = forward.assume_init().into();
            let up = up.assume_init().into();
            Ok((forward, up))
        }
    }

    /// Sets the 3D scale of the object.
    ///
    /// An object can be scaled/warped in all 3 dimensions separately using this function without having to modify polygon data.
    pub fn set_scale(&self, scale: Vector) -> Result<()> {
        unsafe {
            FMOD_Geometry_SetScale(self.inner.as_ptr(), std::ptr::from_ref(&scale).cast())
                .to_result()
        }
    }

    /// Retrieves the 3D scale of the object.
    pub fn get_scale(&self) -> Result<Vector> {
        let mut scale = MaybeUninit::uninit();
        unsafe {
            FMOD_Geometry_GetScale(self.inner.as_ptr(), scale.as_mut_ptr()).to_result()?;
            let scale = scale.assume_init().into();
            Ok(scale)
        }
    }
}
