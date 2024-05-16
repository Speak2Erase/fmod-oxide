// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_void},
    mem::MaybeUninit,
};

use fmod_sys::*;

use crate::{ReverbProperties, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Reverb3D {
    pub(crate) inner: *mut FMOD_REVERB3D,
}

unsafe impl Send for Reverb3D {}
unsafe impl Sync for Reverb3D {}

impl From<*mut FMOD_REVERB3D> for Reverb3D {
    fn from(value: *mut FMOD_REVERB3D) -> Self {
        Reverb3D { inner: value }
    }
}

impl From<Reverb3D> for *mut FMOD_REVERB3D {
    fn from(value: Reverb3D) -> Self {
        value.inner
    }
}

impl Reverb3D {
    /// Sets the 3D attributes of a reverb sphere.
    ///
    /// See the 3D Reverb guide for more information.
    ///
    /// When the position of the listener is less than `max_distance` away from the position of one or more reverb objects,
    /// the listener's 3D reverb properties are a weighted combination of those reverb objects.
    /// Otherwise, the reverb dsp will use the global reverb settings.
    pub fn set_3d_attributes(
        &self,
        position: Option<Vector>,
        min_distance: c_float,
        max_distance: c_float,
    ) -> Result<()> {
        let position = position
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        unsafe {
            FMOD_Reverb3D_Set3DAttributes(self.inner, position, min_distance, max_distance)
                .to_result()
        }
    }

    /// Retrieves the 3D attributes of a reverb sphere.
    ///
    /// See the 3D Reverb guide for more information.
    pub fn get_3d_attributes(&self) -> Result<(Vector, c_float, c_float)> {
        let mut position = MaybeUninit::uninit();
        let mut min_distance = 0.0;
        let mut max_distance = 0.0;
        unsafe {
            FMOD_Reverb3D_Get3DAttributes(
                self.inner,
                position.as_mut_ptr(),
                &mut min_distance,
                &mut max_distance,
            )
            .to_result()?;
            let position = position.assume_init().into();
            Ok((position, min_distance, max_distance))
        }
    }

    /// Sets the environmental properties of a reverb sphere.
    ///
    /// Reverb presets are available, see the associated constants of [`ReverbProperties`].
    pub fn set_properties(&self, properties: ReverbProperties) -> Result<()> {
        unsafe {
            FMOD_Reverb3D_SetProperties(self.inner, std::ptr::from_ref(&properties).cast())
                .to_result()
        }
    }

    /// Retrieves the environmental properties of a reverb sphere.
    ///
    /// See the 3D Reverb guide for more information.
    pub fn get_properties(&self) -> Result<ReverbProperties> {
        let mut properties = MaybeUninit::uninit();
        unsafe {
            FMOD_Reverb3D_GetProperties(self.inner, properties.as_mut_ptr()).to_result()?;
            let properties = properties.assume_init().into();
            Ok(properties)
        }
    }

    /// Sets the active state.
    ///
    /// See the 3D Reverb guide for more information.
    pub fn set_active(&self, active: bool) -> Result<()> {
        unsafe { FMOD_Reverb3D_SetActive(self.inner, active.into()).to_result() }
    }

    /// Retrieves the active state.
    ///
    /// See the 3D Reverb guide for more information.
    pub fn get_active(&self) -> Result<bool> {
        let mut active = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Reverb3D_GetActive(self.inner, &mut active).to_result()?;
        }
        Ok(active.into())
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_raw_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Reverb3D_SetUserData(self.inner, userdata).to_result() }
    }

    pub fn get_raw_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Reverb3D_GetUserData(self.inner, &mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    /// Releases the memory for a reverb object and makes it inactive.
    ///
    /// If you release all [`Reverb3D`] objects and have not added a new [`Reverb3D`] object,
    /// [`crate::System::set_reverb_properties`] should be called to reset the reverb properties.
    pub fn release(&self) -> Result<()> {
        // release userdata
        #[cfg(feature = "userdata-abstraction")]
        let userdata = self.get_raw_userdata()?;

        unsafe {
            FMOD_Reverb3D_Release(self.inner).to_result()?;
        }

        // release/remove userdata if it is not null
        #[cfg(feature = "userdata-abstraction")]
        if !userdata.is_null() {
            crate::userdata::remove_userdata(userdata.into());
            self.set_raw_userdata(std::ptr::null_mut())?;
        }

        Ok(())
    }
}

#[cfg(feature = "userdata-abstraction")]
impl Reverb3D {
    pub fn set_userdata(&self, userdata: crate::userdata::Userdata) -> Result<()> {
        use crate::userdata::{insert_userdata, set_userdata};

        let pointer = self.get_raw_userdata()?;
        if pointer.is_null() {
            let key = insert_userdata(userdata, *self);
            self.set_raw_userdata(key.into())?;
        } else {
            set_userdata(pointer.into(), userdata);
        }

        Ok(())
    }

    pub fn get_userdata(&self) -> Result<Option<crate::userdata::Userdata>> {
        use crate::userdata::get_userdata;

        let pointer = self.get_raw_userdata()?;
        Ok(get_userdata(pointer.into()))
    }
}
