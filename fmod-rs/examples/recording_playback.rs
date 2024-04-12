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

enum State {
    Selection,
    Record,
    Playback,
    Quit,
}

fn execute_selection(system: fmod::studio::System) -> Result<State, Box<dyn std::error::Error>> {
    loop {
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
                '1' => return Ok(State::Record),
                '2' => return Ok(State::Playback),
                'q' => {
                    return Ok(State::Quit);
                }
                _ => {}
            }
        }

        system.update()?;

        let mut stdout = std::io::stdout();

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Recording and playback example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example.\n")?;
        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Waiting to start recording\n")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press 1 to start recording\n")?;
        stdout.write_all(b"Press 2 to play back recording\n")?;
        stdout.write_all(b"Press Q to quit\n")?;
        stdout.write_all(b"\n")?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn execute_record(system: fmod::studio::System) -> Result<State, Box<dyn std::error::Error>> {
    let master_bank = system.load_bank_file(
        c!("fmod/api/studio/examples/media/Master.bank"),
        fmod::studio::LoadBankFlags::NONBLOCKING,
    )?;
    let strings_bank = system.load_bank_file(
        c!("fmod/api/studio/examples/media/Master.strings.bank"),
        fmod::studio::LoadBankFlags::NONBLOCKING,
    )?;
    let vehicles_bank = system.load_bank_file(
        c!("fmod/api/studio/examples/media/Vehicles.bank"),
        fmod::studio::LoadBankFlags::NONBLOCKING,
    )?;
    let sfx_bank = system.load_bank_file(
        c!("fmod/api/studio/examples/media/SFX.bank"),
        fmod::studio::LoadBankFlags::NONBLOCKING,
    )?;

    // Wait for banks to load
    system.flush_commands()?;

    // Start recording commands - it will also record which banks we have already loaded by now
    system.start_command_capture(
        c!("fmod/api/studio/examples/media/playback.cmd.txt"),
        fmod::studio::CommandCaptureFlags::NORMAL,
    )?;

    let explosion_id = system.lookup_id(c!("event:/Weapons/Explosion"))?;

    let engine_description = system.get_event(c!("event:/Vehicles/Ride-on Mower"))?;
    let engine_instance = engine_description.create_instance()?;

    engine_instance.set_parameter_by_name(c!("RPM"), 650.0, false)?;
    engine_instance.start()?;

    // Position the listener at the origin
    let mut attributes = fmod::Attributes3D::default();
    attributes.forward.z = 1.0;
    attributes.up.y = 1.0;
    system.set_listener_attributes(0, attributes, None)?;

    // Position the event 2 units in front of the listener
    attributes.position.z = 2.0;
    engine_instance.set_3d_attributes(attributes)?;

    let mut want_quit = false;

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
                ' ' => break 'main_loop,
                'q' => {
                    want_quit = true;
                    break 'main_loop;
                }
                '1' => {
                    // One-shot event
                    let event_description = system.get_event_by_id(explosion_id)?;
                    let event_instance = event_description.create_instance()?;

                    for i in 0..10 {
                        event_instance.set_volume(i as f32 / 10.0)?;
                    }

                    event_instance.start()?;
                    // Release will clean up the instance when it completes
                    event_instance.release()?;
                }
                'a' => {
                    attributes.position.x -= 1.0;
                    engine_instance.set_3d_attributes(attributes)?;
                }
                'd' => {
                    attributes.position.x += 1.0;
                    engine_instance.set_3d_attributes(attributes)?;
                }
                'w' => {
                    attributes.position.z += 1.0;
                    engine_instance.set_3d_attributes(attributes)?;
                }
                's' => {
                    attributes.position.z -= 1.0;
                    engine_instance.set_3d_attributes(attributes)?;
                }
                _ => {}
            }
        }

        system.update()?;

        let mut stdout = std::io::stdout();

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Recording and playback example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example.\n")?;
        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Recording\n")?;
        stdout.write_all(b"\n")?;

        if let Some((row, col)) = get_character_position(fmod::Vector::default()) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(b"^")?;
        }

        if let Some((row, col)) = get_character_position(attributes.position) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(b"o")?;
        }

        execute!(stdout, MoveTo(0, 20))?;
        stdout.write_all(b"Press SPACE to finish recording\n")?;
        stdout.write_all(b"Press 1 to play a one-shot\n")?;
        stdout.write_all(b"Use the arrow keys (A, D, W, S) to control the engine position\n")?;
        stdout.write_all(b"Press Q to quit\n")?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Unload all the banks
    master_bank.unload()?;
    strings_bank.unload()?;
    vehicles_bank.unload()?;
    sfx_bank.unload()?;

    // Finish recording
    system.flush_commands()?;
    system.stop_command_capture()?;

    Ok(if want_quit {
        State::Quit
    } else {
        State::Selection
    })
}

fn execute_playback(system: fmod::studio::System) -> Result<State, Box<dyn std::error::Error>> {
    let replay = system.load_command_replay(
        c!("fmod/api/studio/examples/media/playback.cmd.txt"),
        fmod::studio::CommandReplayFlags::NORMAL,
    )?;

    let command_count = replay.get_command_count()?;
    let total_time = replay.get_length()?;

    replay.start()?;
    system.update()?;

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
                'q' => {
                    break 'main_loop;
                }
                ' ' => {
                    let paused = replay.get_paused()?;
                    replay.set_paused(!paused)?;
                }
                _ => {}
            }
        }

        let state = replay.get_playback_state()?;
        if state == fmod::studio::PlaybackState::Stopped {
            break;
        }

        let (current_index, current_time) = replay.get_current_command()?;

        system.update()?;

        let mut stdout = std::io::stdout();

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Recording and playback example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example.\n")?;
        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Playing back commands:\n")?;
        writeln!(stdout, "Command = {current_index} / {command_count}")?;
        writeln!(stdout, "Time = {current_time} / {total_time}")?;
        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press SPACE to pause/unpause recording\n")?;
        stdout.write_all(b"Press Q to quit\n")?;

        crossterm::terminal::enable_raw_mode()?;

        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    replay.release()?;
    system.unload_all_banks()?;

    Ok(State::Selection)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::studio::SystemBuilder::new()?
    };

    // The example Studio project is authored for 5.1 sound, so set up the system output mode to match
    builder.core_builder().software_format(
        0,
        fmod_sys::FMOD_SPEAKERMODE_FMOD_SPEAKERMODE_5POINT1,
        0,
    )?;

    let system = builder.build(
        1024,
        fmod::studio::InitFlags::NORMAL,
        fmod::InitFlags::NORMAL,
    )?;

    // use alternate screen
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut state = State::Selection;
    loop {
        match state {
            State::Selection => state = execute_selection(system)?,
            State::Record => state = execute_record(system)?,
            State::Playback => state = execute_playback(system)?,
            State::Quit => break,
        };
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
