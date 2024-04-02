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
        "fmod/api/studio/examples/media/Master.bank",
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    system.load_bank_file(
        "fmod/api/studio/examples/media/Master.strings.bank",
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    system.load_bank_file(
        "fmod/api/studio/examples/media/Vehicles.bank",
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    let event_description = system.get_event("event:/Vehicles/Ride-on Mower")?;
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
