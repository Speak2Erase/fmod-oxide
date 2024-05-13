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
use fmod::c;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::studio::SystemBuilder::new()?
    };

    let system = builder.build(
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
        c!("fmod/api/studio/examples/media/SFX.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    let event_description = system.get_event(c!("event:/Character/Player Footsteps"))?;

    // Find the parameter once and then set by handle
    // Or we can just find by name every time but by handle is more efficient if we are setting lots of parameters
    let parameter_description =
        event_description.get_parameter_description_by_name(c!("Surface"))?;
    let surface_id = parameter_description.id;

    let event_instance = event_description.create_instance()?;

    let mut surface_parameter_value = 1.0;
    event_instance.set_parameter_by_id(surface_id, surface_parameter_value, false)?;

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
                ' ' => {
                    event_instance.start()?;
                }
                '1' => {
                    surface_parameter_value = parameter_description
                        .minimum
                        .max(surface_parameter_value - 1.0);
                    event_instance.set_parameter_by_id(
                        surface_id,
                        surface_parameter_value,
                        false,
                    )?;
                }
                '2' => {
                    surface_parameter_value = parameter_description
                        .maximum
                        .min(surface_parameter_value + 1.0);
                    event_instance.set_parameter_by_id(
                        surface_id,
                        surface_parameter_value,
                        false,
                    )?;
                }
                'q' => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        system.update()?;

        let (user_value, final_value) = event_instance.get_parameter_by_id(surface_id)?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Event Parameter Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;
        writeln!(
            stdout,
            "Surface Parameter = (user: {user_value}, final: {final_value})"
        )?;
        stdout.write_all(b"\n")?;

        stdout.write_all(b"Surface Parameter:\n")?;
        stdout.write_all(b"Press SPACE to play event\n")?;
        stdout.write_all(b"Press 1 to decrease value\n")?;
        stdout.write_all(b"Press 2 to increase value\n")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press Q to quit\n")?;

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
