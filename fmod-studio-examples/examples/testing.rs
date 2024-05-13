// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod::InitFlags;

fn main() -> fmod::Result<()> {
    let builder = unsafe { fmod::SystemBuilder::new()? };

    let system = builder.build(0, InitFlags::NORMAL)?;

    let test = system.create_sound_group(fmod::c!("test"))?;
    test.set_raw_userdata(0xDEADCAFE_usize as _)?;

    println!("{:?}", test.get_raw_userdata()?);

    test.release()?;

    println!("{:?}", test.get_raw_userdata()?);

    unsafe {
        system.release()?;
    }

    Ok(())
}
