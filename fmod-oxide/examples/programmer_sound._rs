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

use fmod::Utf8CStr;
use lanyard::c;
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

pub struct ProgrammerSoundContext {
    core_system: fmod::System,
    studio_system: fmod::studio::System<Userdata>,
    dialogue_string: &'static Utf8CStr,
}

struct Userdata;

impl fmod::UserdataTypes for Userdata {
    type Bank = ();
    type CommandReplay = ();
    type Event = Mutex<ProgrammerSoundContext>;
    type StudioSystem = ();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = unsafe {
        // Safety: we call this before calling any other functions and only in main, so this is safe
        fmod::studio::SystemBuilder::with_userdata()?
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
        c!("fmod/api/studio/examples/media/SFX.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;

    let mut bank_index = 0;
    const BANKS: [&str; 3] = ["Dialogue_EN.bank", "Dialogue_JP.bank", "Dialogue_CN.bank"];

    let bank_path = "fmod/api/studio/examples/media/".to_string() + BANKS[bank_index] + "\0";
    let bank_path = Utf8CStr::from_str_with_nul(&bank_path).unwrap();

    let mut localized_bank =
        system.load_bank_file(bank_path, fmod::studio::LoadBankFlags::NORMAL)?;

    let event_description = system.get_event(c!("event:/Character/Dialogue"))?;
    let event_instance = event_description.create_instance()?;

    let mut dialogue_index = 0;
    const DIALOGUE: [&Utf8CStr; 3] = [c!("welcome"), c!("main menu"), c!("goodbye")];

    let programmer_sound_context = ProgrammerSoundContext {
        core_system: system.get_core_system()?,
        studio_system: system,
        dialogue_string: DIALOGUE[dialogue_index],
    };
    let programmer_sound_context = Arc::new(Mutex::new(programmer_sound_context));

    event_instance.set_user_data(Some(programmer_sound_context.clone()))?;
    event_instance.set_callback(
        Some(Arc::new(programmer_sound_callback)),
        fmod::studio::EventCallbackMask::CREATE_PROGRAMMER_SOUND
            | fmod::studio::EventCallbackMask::DESTROY_PROGRAMMER_SOUND,
    )?;

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
                    localized_bank.unload()?;

                    bank_index = if bank_index < 2 { bank_index + 1 } else { 0 };
                    let bank_path =
                        "fmod/api/studio/examples/media/".to_string() + BANKS[bank_index] + "\0";
                    let bank_path = Utf8CStr::from_str_with_nul(&bank_path).unwrap();

                    localized_bank =
                        system.load_bank_file(bank_path, fmod::studio::LoadBankFlags::NORMAL)?;
                }
                '2' => {
                    dialogue_index = if dialogue_index < 2 {
                        dialogue_index + 1
                    } else {
                        0
                    };
                    programmer_sound_context.lock().unwrap().dialogue_string =
                        DIALOGUE[dialogue_index];
                }
                ' ' => {
                    event_instance.start()?;
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
        stdout.write_all(b"Programmer Sound Example.\n")?;
        stdout.write_all(b"Adapted from the official FMOD example\n")?;
        stdout.write_all(b"==================================================")?;
        stdout.write_all(b"\n")?;

        stdout.write_all(b"Press 1 to change language\n")?;
        stdout.write_all(b"Press 2 to change dialogue\n")?;
        stdout.write_all(b"Press SPACE to play the event\n")?;
        stdout.write_all(b"\n")?;

        stdout.write_all(b"Language:\n")?;
        writeln!(
            stdout,
            " {} English",
            if bank_index == 0 { ">" } else { " " }
        )?;
        writeln!(
            stdout,
            " {} Japanese",
            if bank_index == 1 { ">" } else { " " }
        )?;
        writeln!(
            stdout,
            " {} Chinese",
            if bank_index == 2 { ">" } else { " " }
        )?;
        stdout.write_all(b"\n")?;

        stdout.write_all(b"Dialogue:\n")?;
        writeln!(
            stdout,
            " {} Welcome to the FMOD Studio tutorial",
            if dialogue_index == 0 { ">" } else { " " }
        )?;
        writeln!(
            stdout,
            " {} This is the main menu",
            if dialogue_index == 1 { ">" } else { " " }
        )?;
        writeln!(
            stdout,
            " {} Goodbye",
            if dialogue_index == 2 { ">" } else { " " }
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

fn programmer_sound_callback(
    kind: fmod::studio::EventCallbackKind<'_, Userdata>,
    event_instance: fmod::studio::EventInstance<Userdata>,
) -> fmod::Result<()> {
    match kind {
        fmod::studio::EventCallbackKind::CreateProgrammerSound(props) => {
            let context = event_instance.get_user_data()?.unwrap();
            let context = context.lock().unwrap();

            unsafe {
                let mut info = std::mem::MaybeUninit::uninit();
                fmod::ffi::FMOD_Studio_System_GetSoundInfo(
                    context.studio_system.into(),
                    context.dialogue_string.as_ptr(),
                    info.as_mut_ptr(),
                )
                .to_result()?;
                let mut info = info.assume_init();

                let mut sound = std::ptr::null_mut();
                fmod::ffi::FMOD_System_CreateSound(
                    context.core_system.into(),
                    info.name_or_data,
                    fmod::ffi::FMOD_LOOP_NORMAL
                        | fmod::ffi::FMOD_CREATECOMPRESSEDSAMPLE
                        | fmod::ffi::FMOD_NONBLOCKING
                        | info.mode,
                    &mut info.exinfo,
                    &mut sound,
                )
                .to_result()?;

                *props.sound = sound.into();
                *props.subsound_index = info.subsoundindex;
            };
        }
        fmod::studio::EventCallbackKind::DestroyProgrammerSound(props) => unsafe {
            let sound = (*props.sound).into();
            fmod::ffi::FMOD_Sound_Release(sound).to_result()?
        },
        _ => {}
    }

    Ok(())
}
