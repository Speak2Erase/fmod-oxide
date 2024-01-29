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

use fmod_rs::studio::LoadBankFlags;
use fmod_sys::FMOD_Studio_Bank_GetUserData;

fn main() -> fmod_rs::Result<()> {
    // # Safety: we are only calling this from the main fn and the main thread.
    // No other thread or api call will overlap this.
    let system = unsafe { fmod_rs::studio::System::new()? };

    let main_bank = system.load_bank_file(
        c"fmod/api/studio/examples/media/Master.bank",
        LoadBankFlags::NORMAL,
    )?;
    let strings_bank = system.load_bank_file(
        c"fmod/api/studio/examples/media/Master.strings.bank",
        LoadBankFlags::NORMAL,
    )?;
    let vehicles_bank = system.load_bank_file(
        c"fmod/api/studio/examples/media/Vehicles.bank",
        LoadBankFlags::NORMAL,
    );

    let event_desc = system.get_event(c"event:/Vehicles/Car Engine")?;

    unsafe {
        let mut userdata = std::ptr::null_mut();
        FMOD_Studio_Bank_GetUserData(main_bank.into(), &mut userdata).to_result()?;
        println!("{userdata:p}")
    }

    println!("releasing system");

    unsafe {
        // # Safety we're done processingg and about to return from the main fn.
        // No other API calls can happen after this.
        system.release()?;
    }

    Ok(())
}
