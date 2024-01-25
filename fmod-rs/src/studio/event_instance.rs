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
pub struct EventInstance {
    pub(crate) inner: *mut FMOD_STUDIO_EVENTINSTANCE,
}

unsafe impl Send for EventInstance {}
unsafe impl Sync for EventInstance {}

impl From<*mut FMOD_STUDIO_EVENTINSTANCE> for EventInstance {
    fn from(value: *mut FMOD_STUDIO_EVENTINSTANCE) -> Self {
        EventInstance { inner: value }
    }
}

impl From<EventInstance> for *mut FMOD_STUDIO_EVENTINSTANCE {
    fn from(value: EventInstance) -> Self {
        value.inner
    }
}
