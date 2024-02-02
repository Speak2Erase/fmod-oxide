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
use fmod::studio::{EventCallbackKind, EventCallbackMask};
use fmod_sys::{FMOD_Sound_GetLength, FMOD_Sound_GetName};
use std::{io::Write, sync::Mutex};

#[derive(Default)]
struct CallbackInfo {
    entries: Mutex<Vec<String>>,
}

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
        c"fmod/api/studio/examples/media/Master.bank",
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    system.load_bank_file(
        c"fmod/api/studio/examples/media/Master.strings.bank",
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    let result = system.load_bank_file(
        c"fmod/api/studio/examples/media/Music.bank",
        fmod::studio::LoadBankFlags::NORMAL,
    );
    if let Err(e) = result {
        eprintln!("{e}");
        // Music bank is not exported by default, you will have to export from the tool first
        eprintln!("Please export music.bank from the Studio tool to run this example.");
        return Ok(());
    }

    let event_description = system.get_event(c"event:/Music/Level 01")?;
    let event_instance = event_description.create_instance()?;

    event_instance.set_user_data(Some(CallbackInfo::default()))?;
    event_instance.set_callback(
        marker_callback,
        EventCallbackMask::TIMELINE_MARKER
            | EventCallbackMask::TIMELINE_BEAT
            | EventCallbackMask::SOUND_PLAYED
            | EventCallbackMask::SOUND_STOPPED,
    )?;
    event_instance.start()?;

    let callback_info = event_instance.get_user_data::<CallbackInfo>()?.unwrap();

    let parameter_description =
        event_description.get_parameter_description_by_name(c"Progression")?;
    let progresssion_id = parameter_description.id;

    let mut progression = 0.0;
    event_instance.set_parameter_by_id(progresssion_id, progression, false)?;

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
                    if progression == 0.0 {
                        progression = 1.0;
                    } else {
                        progression = 0.0;
                    }
                    event_instance.set_parameter_by_id(progresssion_id, progression, false)?;
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

        let position = event_instance.get_timeline_position()?;

        stdout.write_all(b"==================================================\n")?;
        stdout.write_all(b"Music Callback Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;
        writeln!(stdout, "Timeline = {position}")?;
        stdout.write_all(b"\n")?;

        let entries = callback_info.entries.lock().unwrap();
        for entry in entries.iter() {
            writeln!(stdout, "  {entry}")?;
        }
        drop(entries);

        stdout.write_all(b"\n")?;
        writeln!(
            stdout,
            "Press SPACE to toggle progression (currently {progression})"
        )?;
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

fn marker_callback(
    kind: EventCallbackKind,
    instance: fmod::studio::EventInstance,
) -> fmod::Result<()> {
    let callback_info = instance.get_user_data::<CallbackInfo>()?.unwrap();
    let mut entries = callback_info.entries.lock().unwrap();

    match kind {
        EventCallbackKind::TimelineMarker(props) => {
            let name = props.name.to_string_lossy();
            entries.push(format!("Named marker '{name}'"));
        }
        EventCallbackKind::TimelineBeat(props) => {
            entries.push(format!(
                "beat {}, bar {} (tempo {} {:.1}:{:.1})",
                props.beat,
                props.bar,
                props.tempo,
                props.time_signature_upper,
                props.time_signature_lower,
            ));
        }
        EventCallbackKind::SoundPlayed(sound) | EventCallbackKind::SoundStopped(sound) => {
            // TODO
            unsafe {
                let mut name_buf = [0u8; 256];
                FMOD_Sound_GetName(sound.into(), &mut name_buf as *mut u8 as *mut i8, 256)
                    .to_result()?;
                let name =
                    std::ffi::CStr::from_bytes_with_nul_unchecked(&name_buf).to_string_lossy();

                let mut length = 0;
                FMOD_Sound_GetLength(sound.into(), &mut length, fmod_sys::FMOD_TIMEUNIT_MS)
                    .to_result()?;

                let status_text = if matches!(kind, EventCallbackKind::SoundPlayed(_)) {
                    "Started"
                } else {
                    "Stopped"
                };

                entries.push(format!("Sound '{name}' (length {length:.3}) {status_text}",));
            }
        }
        _ => {}
    }

    Ok(())
}
