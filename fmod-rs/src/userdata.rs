// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Shareable;

pub trait UserdataTypes {
    // Studio
    type StudioSystem: Shareable + ?Sized;
    type Bank: Shareable + ?Sized;
    type CommandReplay: Shareable + ?Sized;
    type Event: Shareable + ?Sized;
}

impl UserdataTypes for () {
    type StudioSystem = ();
    type Bank = ();
    type CommandReplay = ();
    type Event = ();
}
