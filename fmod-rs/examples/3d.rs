// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crossterm::{
    cursor::*,
    event::{Event, KeyCode, KeyEvent},
    execute,
    terminal::*,
};
use lanyard::c;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::studio::SystemBuilder::new()?
    };

    // The example Studio project is authored for 5.1 sound, so set up the system output mode to match
    let system = builder
        .software_format(0, fmod_sys::FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_5POINT1, 0)?
        .build(
            1024,
            fmod::studio::InitFlags::NORMAL,
            fmod::InitFlags::NORMAL,
        )?;

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
    let event_instance = event_description.create_instance()?;

    event_instance.set_parameter_by_name(c!("RPM"), 650.0, false)?;
    event_instance.start()?;

    // Position the listener at the origin
    let mut attributes = fmod::Attributes3D::default();
    attributes.forward.z = 1.0;
    attributes.up.y = 1.0;

    system.set_listener_attributes(0, attributes, None)?;

    // Position the event 2 units in front of the listener
    attributes.position.z = 2.0;
    event_instance.set_3d_attributes(attributes)?;

    // use alternate screen
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    'main_loop: loop {
        while crossterm::event::poll(std::time::Duration::from_micros(1000))? {
            let event = crossterm::event::read()?;

            let Event::Key(KeyEvent {
                code: KeyCode::Char(character),
                ..
            }) = event
            else {
                continue;
            };

            match character {
                'w' => {
                    attributes.position.z += 1.0;
                    event_instance.set_3d_attributes(attributes)?;
                }
                'a' => {
                    attributes.position.x -= 1.0;
                    event_instance.set_3d_attributes(attributes)?;
                }
                's' => {
                    attributes.position.z -= 1.0;
                    event_instance.set_3d_attributes(attributes)?;
                }
                'd' => {
                    attributes.position.x += 1.0;
                    event_instance.set_3d_attributes(attributes)?;
                }
                'q' => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Event 3D Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;

        if let Some((row, col)) = get_character_position(fmod::Vector::default()) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(b"^")?;
        }

        if let Some((row, col)) = get_character_position(attributes.position) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(b"o")?;
        }

        execute!(stdout, MoveTo(0, 20))?;
        stdout.write_all(b"Use the arrow keys (W, A, S, D) to control the event position\n")?;
        stdout.write_all(b"Press Q to quit")?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // reset terminal
    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    unsafe {
        // Safety: we don't use any fmod api calls after this, so this is ok
        system.release()?;
    }

    Ok(())
}

fn get_character_position(postion: fmod::Vector) -> Option<(u16, u16)> {
    let row = (-postion.z) as i16 + 8;
    let col = postion.x as i16 + 25;

    if row.is_positive() && row < 16 && col.is_positive() && col < 50 {
        Some((row as u16 + 4, col as u16))
    } else {
        None
    }
}
