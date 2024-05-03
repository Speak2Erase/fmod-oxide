// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_int},
    mem::MaybeUninit,
};

use fmod_sys::*;

use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Geometry {
    pub(crate) inner: *mut FMOD_GEOMETRY,
}

unsafe impl Send for Geometry {}
unsafe impl Sync for Geometry {}

impl From<*mut FMOD_GEOMETRY> for Geometry {
    fn from(value: *mut FMOD_GEOMETRY) -> Self {
        Geometry { inner: value }
    }
}

impl From<Geometry> for *mut FMOD_GEOMETRY {
    fn from(value: Geometry) -> Self {
        value.inner
    }
}

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
                self.inner,
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
                self.inner,
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
            FMOD_Geometry_GetPolygonNumVertices(self.inner, index, &mut count).to_result()?;
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
                self.inner,
                index,
                vertex_index,
                std::ptr::from_ref(&vertex).cast(),
            )
            .to_result()
        }
    }

    /// Retrieves the position of a vertex.
    ///
    /// Vertices are relative to the position of the object. See [`Geometry::set_osition`].
    pub fn get_polygon_vertex(&self, index: c_int, vertex_index: c_int) -> Result<Vector> {
        let mut vertex = MaybeUninit::uninit();
        unsafe {
            FMOD_Geometry_GetPolygonVertex(self.inner, index, vertex_index, vertex.as_mut_ptr())
                .to_result()?;
            let vertex = vertex.assume_init().into();
            Ok(vertex)
        }
    }

    /// Sets the 3D position of the object.
    ///
    /// Position is in world space.
    pub fn set_position(&self, position: Vector) -> Result<()> {
        unsafe {
            FMOD_Geometry_SetPosition(self.inner, std::ptr::from_ref(&position).cast()).to_result()
        }
    }

    /// Retrieves the 3D position of the object.
    ///
    /// Position is in world space.
    pub fn get_position(&self) -> Result<Vector> {
        let mut position = MaybeUninit::uninit();
        unsafe {
            FMOD_Geometry_GetPosition(self.inner, position.as_mut_ptr()).to_result()?;
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
                self.inner,
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
            FMOD_Geometry_GetRotation(self.inner, forward.as_mut_ptr(), up.as_mut_ptr())
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
        unsafe { FMOD_Geometry_SetScale(self.inner, std::ptr::from_ref(&scale).cast()).to_result() }
    }

    /// Retrieves the 3D scale of the object.
    pub fn get_scale(&self) -> Result<Vector> {
        let mut scale = MaybeUninit::uninit();
        unsafe {
            FMOD_Geometry_GetScale(self.inner, scale.as_mut_ptr()).to_result()?;
            let scale = scale.assume_init().into();
            Ok(scale)
        }
    }

    /// Adds a polygon.
    ///
    /// All vertices must lay in the same plane otherwise behavior may be unpredictable.
    /// The polygon is assumed to be convex. A non convex polygon will produce unpredictable behavior.
    /// Polygons with zero area will be ignored.
    ///
    /// Polygons cannot be added if already at the maximum number of polygons or if the addition of their verticies would result in exceeding the maximum number of vertices.
    ///
    /// Vertices of an object are in object space, not world space, and so are relative to the position, or center of the object.
    /// See [`Geometry::setP_psition`].
    pub fn add_polygon(
        &self,
        direct_occlusion: c_float,
        reverb_occlusion: c_float,
        double_sided: bool,
        vertices: &[Vector],
    ) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_Geometry_AddPolygon(
                self.inner,
                direct_occlusion,
                reverb_occlusion,
                double_sided.into(),
                vertices.len() as c_int,
                vertices.as_ptr().cast(),
                &mut index,
            )
            .to_result()?;
        }
        Ok(index)
    }

    /// Sets whether an object is processed by the geometry engine.
    pub fn set_active(&self, active: bool) -> Result<()> {
        unsafe { FMOD_Geometry_SetActive(self.inner, active.into()).to_result() }
    }

    /// Retrieves whether an object is processed by the geometry engine.
    pub fn get_active(&self) -> Result<bool> {
        let mut active = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Geometry_GetActive(self.inner, &mut active).to_result()?;
        }
        Ok(active.into())
    }

    /// Retrieves the maximum number of polygons and vertices allocatable for this object.
    ///
    /// The maximum number was set with [`crate::System::create_geometry`].
    pub fn get_max_polygons(&self) -> Result<(c_int, c_int)> {
        let mut max_polygons = 0;
        let mut max_vertices = 0;
        unsafe {
            FMOD_Geometry_GetMaxPolygons(self.inner, &mut max_polygons, &mut max_vertices)
                .to_result()?;
        }
        Ok((max_polygons, max_vertices))
    }

    /// Retrieves the number of polygons in this object.
    pub fn get_polygon_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Geometry_GetNumPolygons(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    // TODO userdata

    /// Frees a geometry object and releases its memory.
    pub fn release(&self) -> Result<()> {
        unsafe { FMOD_Geometry_Release(self.inner).to_result() }
    }

    /// Saves the geometry object as a serialized binary block to a [`Vec`].
    ///
    /// The data can be saved to a file if required and loaded later with [`crate::System::load_geometry`].
    pub fn save(&self) -> Result<Vec<u8>> {
        let mut data_size = 0;
        unsafe {
            FMOD_Geometry_Save(self.inner, std::ptr::null_mut(), &mut data_size).to_result()?;
        }

        let mut data = vec![0; data_size as usize];
        unsafe {
            FMOD_Geometry_Save(self.inner, data.as_mut_ptr().cast(), &mut data_size).to_result()?;
        }

        Ok(data)
    }
}
