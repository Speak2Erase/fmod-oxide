// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lanyard::c;

#[derive(Debug, PartialEq)]
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("print on drop: {}", self.0);
    }
}

struct Userdata;

impl fmod::UserdataTypes for Userdata {
    type StudioSystem = ();
    type Bank = ();
    type CommandReplay = ();
    type Event = PrintOnDrop;
}

fn main() -> fmod::Result<()> {
    // # Safety: we are only calling this from the main fn and the main thread.
    // No other thread or api call will overlap this.
    let system = unsafe { fmod::studio::System::<Userdata>::with_userdata()? };

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
    let print_on_drop = PrintOnDrop("event desc userdata has been dropped :3").into();
    event_description.set_user_data(Some(print_on_drop))?;

    let instance = event_description.create_instance()?;

    let print_on_drop = PrintOnDrop("event instance userdata has been dropped :3").into();
    instance.set_user_data(Some(print_on_drop))?;

    println!("updating system");

    system.update()?;

    system.unload_all_banks()?;

    system.update()?;

    println!("releasing system");

    unsafe {
        // # Safety we're done processingg and about to return from the main fn.
        // No other API calls can happen after this.
        system.release()?;
    }

    println!("system released");

    Ok(())
}
