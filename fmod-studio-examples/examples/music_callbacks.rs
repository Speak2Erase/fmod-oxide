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
use fmod::studio::{EventCallbackMask, EventInstanceCallback};
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

#[derive(Default)]
struct CallbackInfo {
    entries: Mutex<Vec<String>>,
}

struct Callback;

impl EventInstanceCallback for Callback {
    fn timeline_marker(
        event: fmod::studio::EventInstance,
        timeline_props: fmod::studio::TimelineMarkerProperties,
    ) -> fmod::Result<()> {
        let callback_info = event
            .get_userdata()?
            .unwrap()
            .downcast::<CallbackInfo>()
            .unwrap();
        let mut entries = callback_info.entries.lock().unwrap();

        let name = timeline_props.name.to_string();
        entries.push(format!("Named marker '{name}'"));

        Ok(())
    }

    fn timeline_beat(
        event: fmod::studio::EventInstance,
        timeline_beat: fmod::studio::TimelineBeatProperties,
    ) -> fmod::Result<()> {
        let callback_info = event
            .get_userdata()?
            .unwrap()
            .downcast::<CallbackInfo>()
            .unwrap();
        let mut entries = callback_info.entries.lock().unwrap();

        entries.push(format!(
            "beat {}, bar {} (tempo {} {:.1}:{:.1})",
            timeline_beat.beat,
            timeline_beat.bar,
            timeline_beat.tempo,
            timeline_beat.time_signature_upper,
            timeline_beat.time_signature_lower,
        ));

        Ok(())
    }

    fn sound_played(event: fmod::studio::EventInstance, sound: fmod::Sound) -> fmod::Result<()> {
        let callback_info = event
            .get_userdata()?
            .unwrap()
            .downcast::<CallbackInfo>()
            .unwrap();
        let mut entries = callback_info.entries.lock().unwrap();

        let name = sound.get_name()?;
        let length = sound.get_length(fmod::TimeUnit::MS)?;
        entries.push(format!("Sound '{name}' (length {length:.3}) started",));

        Ok(())
    }

    fn sound_stopped(event: fmod::studio::EventInstance, sound: fmod::Sound) -> fmod::Result<()> {
        let callback_info = event
            .get_userdata()?
            .unwrap()
            .downcast::<CallbackInfo>()
            .unwrap();
        let mut entries = callback_info.entries.lock().unwrap();

        let name = sound.get_name()?;
        let length = sound.get_length(fmod::TimeUnit::MS)?;

        entries.push(format!("Sound '{name}' (length {length:.3}) stopped",));

        Ok(())
    }
}

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
    let result = system.load_bank_file(
        c!("fmod/api/studio/examples/media/Music.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    );
    if let Err(e) = result {
        eprintln!("{e}");
        // Music bank is not exported by default, you will have to export from the tool first
        eprintln!("Please export music.bank from the Studio tool to run this example.");
        return Ok(());
    }

    let event_description = system.get_event(c!("event:/Music/Level 01"))?;
    let event_instance = event_description.create_instance()?;

    let callback_info = Arc::new(CallbackInfo::default());

    event_instance.set_userdata(callback_info.clone())?;
    event_instance.set_callback::<Callback>(
        EventCallbackMask::TIMELINE_MARKER
            | EventCallbackMask::TIMELINE_BEAT
            | EventCallbackMask::SOUND_PLAYED
            | EventCallbackMask::SOUND_STOPPED,
    )?;
    event_instance.start()?;

    let parameter_description =
        event_description.get_parameter_description_by_name(c!("Progression"))?;
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
