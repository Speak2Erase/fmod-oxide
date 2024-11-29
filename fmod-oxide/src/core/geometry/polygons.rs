// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::{
    ffi::{c_float, c_int},
    mem::MaybeUninit,
};

use crate::{Geometry, Vector};

impl Geometry {
    /// Sets individual attributes for a polygon inside a geometry object.
    pub fn set_polygon_attributes(
        &self,
        index: c_int,
        direct_occlusion: c_float,
        reverb_occlusion: c_float,
        double_sided: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Geometry_SetPolygonAttributes(
                self.inner.as_ptr(),
                index,
                direct_occlusion,
                reverb_occlusion,
                double_sided.into(),
            )
            .to_result()
        }
    }

    /// Retrieves the attributes for a polygon.
    pub fn get_polygon_attributes(&self, index: c_int) -> Result<(c_float, c_float, bool)> {
        let mut direct = 0.0;
        let mut reverb = 0.0;
        let mut double_sided = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Geometry_GetPolygonAttributes(
                self.inner.as_ptr(),
                index,
                &mut direct,
                &mut reverb,
                &mut double_sided,
            )
            .to_result()?;
        }
        Ok((direct, reverb, double_sided.into()))
    }

    /// Gets the number of vertices in a polygon.
    pub fn get_polygon_vertex_count(&self, index: c_int) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Geometry_GetPolygonNumVertices(self.inner.as_ptr(), index, &mut count)
                .to_result()?;
        }
        Ok(count)
    }

    /// Alters the position of a polygon's vertex inside a geometry object.
    ///
    /// Vertices are relative to the position of the object. See [`Geometry::set_position`].
    ///
    /// There may be some significant overhead with this function as it may cause some reconfiguration of internal data structures used to speed up sound-ray testing.
    ///
    /// You may get better results if you want to modify your object by using [`Geometry::set_position`], [`Geometry::set_scale`] and [`Geometry::set_rotation`].
    pub fn set_polygon_vertex(
        &self,
        index: c_int,
        vertex_index: c_int,
        vertex: Vector,
    ) -> Result<()> {
        unsafe {
            FMOD_Geometry_SetPolygonVertex(
                self.inner.as_ptr(),
                index,
                vertex_index,
                std::ptr::from_ref(&vertex).cast(),
            )
            .to_result()
        }
    }

    /// Retrieves the position of a vertex.
    ///
    /// Vertices are relative to the position of the object. See [`Geometry::set_position`].
    pub fn get_polygon_vertex(&self, index: c_int, vertex_index: c_int) -> Result<Vector> {
        let mut vertex = MaybeUninit::uninit();
        unsafe {
            FMOD_Geometry_GetPolygonVertex(
                self.inner.as_ptr(),
                index,
                vertex_index,
                vertex.as_mut_ptr(),
            )
            .to_result()?;
            let vertex = vertex.assume_init().into();
            Ok(vertex)
        }
    }
}
