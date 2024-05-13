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
    let mut builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::studio::SystemBuilder::new()?
    };

    // The example Studio project is authored for 5.1 sound, so set up the system output mode to match
    builder
        .core_builder()
        .software_format(0, fmod::SpeakerMode::FivePointOne, 0)?;

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
        c!("fmod/api/studio/examples/media/Music.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    let spatializer_description = system.get_event(c!("event:/Music/Radio Station"))?;
    let spatializer_instance = spatializer_description.create_instance()?;
    spatializer_instance.start()?;

    let mut is_on_ground = false;
    let mut use_listener_attenuation_position = false;
    let mut t = 0.0f32;

    // use alternate screen
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;

    'main_loop: loop {
        let (_, radio_frequency) = spatializer_instance.get_parameter_by_name(c!("Freq"))?;
        let (_, spatializer) = spatializer_instance.get_parameter_by_name(c!("Spatializer"))?;

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
                    if radio_frequency == 3.0 {
                        spatializer_instance.set_parameter_by_name(c!("Freq"), 0.0, false)?;
                    } else {
                        spatializer_instance.set_parameter_by_name(
                            c!("Freq"),
                            radio_frequency + 1.5,
                            false,
                        )?;
                    }
                }
                '2' => {
                    if spatializer == 1.0 {
                        spatializer_instance.set_parameter_by_name(
                            c!("Spatializer"),
                            0.0,
                            false,
                        )?;
                    } else {
                        spatializer_instance.set_parameter_by_name(
                            c!("Spatializer"),
                            1.0,
                            false,
                        )?;
                    }
                }
                '3' => {
                    is_on_ground = !is_on_ground;
                }
                '4' => {
                    use_listener_attenuation_position = !use_listener_attenuation_position;
                }
                'q' => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        let mut vec = fmod::Attributes3D::default();
        vec.forward.z = 1.0;
        vec.up.y = 1.0;

        // Rotate sound in a circle
        vec.position.x = t.sin() * 3.0;
        vec.position.z = t.cos() * 3.0;
        t += 0.03;

        if is_on_ground {
            vec.position.y = 0.0; // At ground level
        } else {
            vec.position.y = 5.0; // Up high
        }

        spatializer_instance.set_3d_attributes(vec)?;

        let mut listener_vec = fmod::Attributes3D::default();
        listener_vec.forward.z = 1.0;
        listener_vec.up.y = 1.0;

        let mut attenuation_pos = vec.position;
        attenuation_pos.z -= 10.0;

        system.set_listener_attributes(
            0,
            listener_vec,
            use_listener_attenuation_position.then_some(attenuation_pos),
        )?;

        system.update()?;

        let radio_string = match radio_frequency {
            0.0 => "Rock",
            1.5 => "Lo-fi",
            _ => "Hip hop",
        };
        let spatial_string = if spatializer == 0.0 {
            "Standard 3D Spatializer"
        } else {
            "Object Spatializer"
        };

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Object Panning Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;

        writeln!(stdout, "Playing {radio_string} with the {spatial_string}.")?;
        writeln!(
            stdout,
            "Radio is {}.",
            if is_on_ground {
                "on the ground"
            } else {
                "up in the air"
            }
        )?;

        stdout.write_all(b"\n")?;
        stdout.write_all(b"Press 1 to switch stations.\n")?;
        stdout.write_all(b"Press 2 to switch spatializer.\n")?;
        stdout.write_all(b"Press 3 to elevate the event instance.\n")?;
        writeln!(
            stdout,
            "Press 4 to {} use of attenuation position.",
            if use_listener_attenuation_position {
                "disable"
            } else {
                "enable"
            }
        )?;
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
