// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use fmod::studio::InitFlags as StudioInitFlags;
use fmod::{c, InitFlags};

struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("Dropping {}!", self.0);
    }
}

fn main() -> fmod::Result<()> {
    let builder = unsafe { fmod::studio::SystemBuilder::new()? };

    let system = builder.build(0, StudioInitFlags::NORMAL, InitFlags::NORMAL)?;

    system.load_bank_file(
        c!("fmod/api/studio/examples/media/Master.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    system.load_bank_file(
        c!("fmod/api/studio/examples/media/Master.strings.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    system.load_bank_file(
        c!("fmod/api/studio/examples/media/Vehicles.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    let event_description = system.get_event(c!("event:/Vehicles/Ride-on Mower"))?;
    event_description.set_userdata(Arc::new(PrintOnDrop("desc")))?;

    let instance = event_description.create_instance()?;
    instance.set_userdata(Arc::new(PrintOnDrop("instance")))?;
    instance.release()?;

    for _ in 0..3 {
        system.update()?;
        std::thread::sleep(std::time::Duration::from_millis(32));
        println!("update")
    }

    system.unload_all_banks()?;

    for _ in 0..3 {
        system.update()?;
        std::thread::sleep(std::time::Duration::from_millis(32));
        println!("update")
    }
    println!("release");

    unsafe {
        system.release()?;
    }

    Ok(())
}
