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
        c!("fmod/api/studio/examples/media/SFX.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    // Get the Looping Ambience event
    let looping_ambience_description = system.get_event(c!("event:/Ambience/Country"))?;
    let looping_ambiance_instance = looping_ambience_description.create_instance()?;

    // Get the 4 Second Surge event
    let cancel_description = system.get_event(c!("event:/Ui/Cancel"))?;
    let cancel_instance = cancel_description.create_instance()?;

    // Get the Single Explosion event
    let explosion_description = system.get_event(c!("event:/Weapons/Explosion"))?;
    // Start loading explosion sample data and keep it in memory
    explosion_description.load_sample_data()?;

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
                '1' => {
                    let explosion_instance = explosion_description.create_instance()?;

                    explosion_instance.start()?;

                    // Release will clean up the instance when it completes
                    explosion_instance.release()?;
                }
                '2' => {
                    looping_ambiance_instance.start()?;
                }
                '3' => {
                    looping_ambiance_instance.stop(fmod::studio::StopMode::Immediate)?;
                }
                '4' => {
                    // Calling start on an instance will cause it to restart if it's already playing
                    cancel_instance.start()?;
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
        stdout.write_all(b"Simple Event Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press 1 to fire and forget the explosion\n")?;
        stdout.write_all(b"Press 2 to start the looping ambiance\n")?;
        stdout.write_all(b"Press 3 to stop the looping ambiance\n")?;
        stdout.write_all(b"Press 4 to start/restart the cancel sound\n")?;
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
