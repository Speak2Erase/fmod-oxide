// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int};

use crate::{Geometry, System, Vector};

impl System {
    /// [`Geometry`] creation function. This function will create a base geometry object which can then have polygons added to it.
    ///
    /// Polygons can be added to a geometry object using [`Geometry::add_polygon`]. For best efficiency, avoid overlapping of polygons and long thin polygons.
    ///
    /// A geometry object stores its polygons in a group to allow optimization for line testing, insertion and updating of geometry in real-time.
    /// Geometry objects also allow for efficient rotation, scaling and translation of groups of polygons.
    ///
    /// It is important to set the value of maxworldsize to an appropriate value using [`System::set_geometry_settings`].
    pub fn create_geometry(&self, max_polygons: c_int, max_vertices: c_int) -> Result<Geometry> {
        let mut geometry = std::ptr::null_mut();
        unsafe {
            FMOD_System_CreateGeometry(self.inner, max_polygons, max_vertices, &mut geometry)
                .to_result()?;
        }
        Ok(geometry.into())
    }

    /// Sets the maximum world size for the geometry engine for performance / precision reasons.
    ///
    /// FMOD uses an efficient spatial partitioning system to store polygons for ray casting purposes.
    /// The maximum size of the world should (`max_world_size`) be set to allow processing within a known range.
    /// Outside of this range, objects and polygons will not be processed as efficiently.
    /// Excessive world size settings can also cause loss of precision and efficiency.
    ///
    /// Setting `max_world_size` should be done first before creating any geometry.
    /// It can be done any time afterwards but may be slow in this case.
    pub fn set_geometry_settings(&self, max_world_size: c_float) -> Result<()> {
        unsafe { FMOD_System_SetGeometrySettings(self.inner, max_world_size).to_result() }
    }

    /// Retrieves the maximum world size for the geometry engine.
    ///
    /// FMOD uses an efficient spatial partitioning system to store polygons for ray casting purposes.
    /// The maximum size of the world should (`max_world_size`) be set to allow processing within a known range.
    /// Outside of this range, objects and polygons will not be processed as efficiently.
    /// Excessive world size settings can also cause loss of precision and efficiency.
    pub fn get_geometry_settings(&self) -> Result<c_float> {
        let mut max_world_size = 0.0;
        unsafe {
            FMOD_System_GetGeometrySettings(self.inner, &mut max_world_size).to_result()?;
        }
        Ok(max_world_size)
    }

    /// Creates a geometry object from a block of memory which contains pre-saved geometry data.
    ///
    /// This function avoids the need to manually create and add geometry for faster start time.
    pub fn load_geometry(&self, data: &[u8]) -> Result<Geometry> {
        let mut geometry = std::ptr::null_mut();
        unsafe {
            FMOD_System_LoadGeometry(
                self.inner,
                data.as_ptr().cast(),
                data.len() as c_int,
                &mut geometry,
            )
            .to_result()?;
        }
        Ok(geometry.into())
    }

    /// Calculates geometry occlusion between a listener and a sound source.
    ///
    /// If single sided polygons have been created, it is important to get the source and listener positions around the right way,
    /// as the occlusion from point A to point B may not be the same as the occlusion from point B to point A.
    pub fn get_geometry_occlusion(
        &self,
        listener: Vector,
        source: Vector,
    ) -> Result<(c_float, c_float)> {
        let mut direct = 0.0;
        let mut reverb = 0.0;
        unsafe {
            FMOD_System_GetGeometryOcclusion(
                self.inner,
                std::ptr::from_ref(&listener).cast(),
                std::ptr::from_ref(&source).cast(),
                &mut direct,
                &mut reverb,
            )
            .to_result()?;
        }
        Ok((direct, reverb))
    }
}
