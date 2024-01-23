// Copyright (C) 2024 Lily Lyons
//
// This file is part of fmod-rs.
//
// fmod-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fmod-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with fmod-rs.  If not, see <http://www.gnu.org/licenses/>.

use fmod_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Bus {
    pub(crate) inner: *mut FMOD_STUDIO_BUS,
}

unsafe impl Send for Bus {}
unsafe impl Sync for Bus {}

impl From<*mut FMOD_STUDIO_BUS> for Bus {
    fn from(value: *mut FMOD_STUDIO_BUS) -> Self {
        Bus { inner: value }
    }
}

impl From<Bus> for *mut FMOD_STUDIO_BUS {
    fn from(value: Bus) -> Self {
        value.inner
    }
}
