// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod::InitFlags;

fn main() -> fmod::Result<()> {
    let builder = unsafe { fmod::studio::SystemBuilder::new()? };

    let system = builder.build(0, fmod::studio::InitFlags::NORMAL, InitFlags::NORMAL)?;

    system.load_bank_file(
        fmod::c!("fmod/api/studio/examples/media/Master.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    let strings = system.load_bank_file(
        fmod::c!("fmod/api/studio/examples/media/Master.strings.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    for i in 0..strings.string_count()? {
        let (guid, string) = strings.get_string_info(i)?;
        println!("{guid}: {:?}", string.as_str());
    }

    let core_system = system.get_core_system()?;

    let (name, guid, foo, speaker, bar) = core_system.get_driver_info(0)?;
    println!(
        "Driver {:?}: {guid}, {foo}, {speaker:?}, {bar}",
        name.as_str(),
    );

    unsafe {
        system.release()?;
    }

    Ok(())
}
