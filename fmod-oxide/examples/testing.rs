// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use fmod::studio::InitFlags as StudioInitFlags;
use fmod::InitFlags;

struct PrintOnDrop;

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() -> fmod::Result<()> {
    let mut builder = unsafe { fmod::studio::SystemBuilder::new()? };

    let system = builder.build(0, StudioInitFlags::NORMAL, InitFlags::NORMAL)?;

    system.set_userdata(Arc::new(PrintOnDrop))?;

    unsafe {
        system.release()?;
    }

    Ok(())
}
