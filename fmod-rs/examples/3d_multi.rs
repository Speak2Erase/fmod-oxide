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
        c!("fmod/api/studio/examples/media/Vehicles.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    let event_description = system.get_event(c!("event:/Vehicles/Ride-on Mower"))?;
    let event_instance = event_description.create_instance()?;

    event_instance.set_parameter_by_name(c!("RPM"), 650.0, false)?;
    event_instance.start()?;

    // Position two listeners
    system.set_listener_count(2)?;

    let mut active_listener = 0usize;
    let mut listener_distance = 8.0;
    let mut listener_weight = [1.0; 2];
    let mut listener_attributes = [fmod::Attributes3D::default(); 2];

    listener_attributes[0].forward.z = 1.0;
    listener_attributes[0].up.y = 1.0;
    listener_attributes[0].position.x = -listener_distance;

    listener_attributes[1].forward.z = 1.0;
    listener_attributes[1].up.y = 1.0;
    listener_attributes[1].position.x = listener_distance;

    system.set_listener_attributes(0, listener_attributes[0], None)?;
    system.set_listener_weight(0, listener_weight[0])?;

    system.set_listener_attributes(1, listener_attributes[1], None)?;
    system.set_listener_weight(1, listener_weight[1])?;

    let mut car_attributes = fmod::Attributes3D::default();
    car_attributes.forward.z = 1.0;
    car_attributes.up.y = 1.0;
    car_attributes.position.x = 0.0;
    car_attributes.position.z = 2.0;

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
                    car_attributes.position.z += 1.0;
                    event_instance.set_3d_attributes(car_attributes)?;
                }
                'a' => {
                    car_attributes.position.x -= 1.0;
                    event_instance.set_3d_attributes(car_attributes)?;
                }
                's' => {
                    car_attributes.position.z -= 1.0;
                    event_instance.set_3d_attributes(car_attributes)?;
                }
                'd' => {
                    car_attributes.position.x += 1.0;
                    event_instance.set_3d_attributes(car_attributes)?;
                }
                '1' => {
                    active_listener += 1;
                    if active_listener > 2 {
                        active_listener = 0;
                    }
                }
                '2' => {
                    active_listener = active_listener.checked_sub(1).unwrap_or(2);
                }
                '3' => {
                    listener_distance = (listener_distance - 1.0).max(0.0);
                }
                '4' => {
                    listener_distance = (listener_distance + 1.0).max(0.0);
                }
                'q' => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

        for i in 0..2 {
            // 0 = left, 1 = right, 2 = both
            let target = (active_listener == i || active_listener == 2) as i32 as f32;
            let distance = target - listener_weight[i];
            // very rough estimate of 50ms per update, not properly timed
            let step = 50.0 / 1000.0;

            if (-step..step).contains(&distance) {
                listener_weight[i] = target;
            } else if distance > 0.0 {
                listener_weight[i] += step;
            } else {
                listener_weight[i] -= step;
            }
        }

        listener_attributes[0].position.x = -listener_distance;
        listener_attributes[1].position.x = listener_distance;

        system.set_listener_attributes(0, listener_attributes[0], None)?;
        system.set_listener_weight(0, listener_weight[0])?;
        system.set_listener_attributes(1, listener_attributes[1], None)?;
        system.set_listener_weight(1, listener_weight[1])?;

        system.update()?;

        execute!(stdout, Clear(ClearType::All))?;

        execute!(stdout, MoveTo(0, 0))?;
        crossterm::terminal::disable_raw_mode()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Event 3D Multi-Listener Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;

        if let Some((row, col)) = get_character_position(fmod::Vector::default()) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(b"^")?;
        }

        if let Some((row, col)) = get_character_position(car_attributes.position) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(b"o")?;
        }

        if let Some((row, col)) = get_character_position(fmod::Vector {
            x: -listener_distance,
            y: 0.0,
            z: 0.0,
        }) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(symbol_for_weight(listener_weight[0]))?;
        }

        if let Some((row, col)) = get_character_position(fmod::Vector {
            x: listener_distance,
            y: 0.0,
            z: 0.0,
        }) {
            execute!(stdout, MoveTo(col, row))?;
            stdout.write_all(symbol_for_weight(listener_weight[1]))?;
        }

        execute!(stdout, MoveTo(0, 20))?;
        writeln!(stdout, "Left listener: {:.0}", listener_weight[0] * 100.)?;
        writeln!(stdout, "Right listener: {:.0}", listener_weight[1] * 100.)?;
        stdout.write_all(b"Use the arrow keys (W, A, S, D) to control the event position\n")?;
        stdout.write_all(b"Use 1 and 2 to toggle left/right/both listeners\n")?;
        stdout.write_all(b"Use 3 and 4 to move listeners closer or further apart\n")?;
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

fn symbol_for_weight(weight: f32) -> &'static [u8] {
    if weight > 0.95 {
        b"X"
    } else if weight > 0.05 {
        b"x"
    } else {
        b"."
    }
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
