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

use crate::Shareable;

pub trait UserdataTypes {
    // Studio
    type StudioSystem: Shareable;
    type Bank: Shareable;
    type Bus: Shareable;
    type CommandReplay: Shareable;
    type EventDescription: Shareable;
    type EventInstance: Shareable;
    type VCA: Shareable;
}

impl UserdataTypes for () {
    type StudioSystem = ();
    type Bank = ();
    type Bus = ();
    type CommandReplay = ();
    type EventDescription = ();
    type EventInstance = ();
    type VCA = ();
}
