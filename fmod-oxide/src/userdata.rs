// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(not(target_pointer_width = "64"))]
compile_error!("fmod-oxide's userdata system is currently only supported on 64-bit platforms.");

use std::{
    any::Any,
    sync::{Arc, RwLock},
};

use once_cell::sync::Lazy;
use slotmap::{HopSlotMap, Key, KeyData};

use crate::{
    studio::{Bank, CommandReplay, EventDescription, EventInstance, System as StudioSystem},
    ChannelControl, Dsp, DspConnection, Geometry, Reverb3D, Sound, SoundGroup, System,
};

#[derive(Default)]
struct UserdataStorage {
    // we use hopslotmap as we expect to iterate over all userdata often
    // denseslotmap might be better? it has vec iter times
    slotmap: HopSlotMap<UserdataKey, UserdataValue>,
}

slotmap::new_key_type! {
    pub struct UserdataKey;
}

struct UserdataValue {
    userdata: Userdata,
    owner: HasUserdata,
}

#[derive(PartialEq)]
pub(crate) enum HasUserdata {
    StudioSystem(StudioSystem),
    Bank(Bank),
    EventDescription(EventDescription),
    EventInstance(EventInstance),
    CommandReplay(CommandReplay),
    //
    Reverb3D(Reverb3D),
    SoundGroup(SoundGroup),
    ChannelControl(ChannelControl),
    Dsp(Dsp),
    Sound(Sound),
    Geometry(Geometry),
    DspConnection(DspConnection),
    System(System),
}

impl HasUserdata {
    fn get_raw_userdata(&self) -> fmod_sys::Result<*mut std::ffi::c_void> {
        match self {
            HasUserdata::StudioSystem(s) => s.get_raw_userdata(),
            HasUserdata::Bank(b) => b.get_raw_userdata(),
            HasUserdata::EventDescription(e) => e.get_raw_userdata(),
            HasUserdata::EventInstance(e) => e.get_raw_userdata(),
            HasUserdata::CommandReplay(c) => c.get_raw_userdata(),
            HasUserdata::Reverb3D(r) => r.get_raw_userdata(),
            HasUserdata::SoundGroup(s) => s.get_raw_userdata(),
            // may occasionally be called when the channel is lost, but this should return Err() in that case
            HasUserdata::ChannelControl(c) => c.get_raw_userdata(),
            HasUserdata::Dsp(d) => d.get_raw_userdata(),
            HasUserdata::Sound(s) => s.get_raw_userdata(),
            HasUserdata::Geometry(g) => g.get_raw_userdata(),
            HasUserdata::DspConnection(c) => c.get_raw_userdata(),
            HasUserdata::System(s) => s.get_raw_userdata(),
        }
    }

    fn is_valid(&self, key: UserdataKey) -> bool {
        match self {
            HasUserdata::StudioSystem(s) => s.is_valid(),
            HasUserdata::Bank(b) => b.is_valid(),
            HasUserdata::EventDescription(e) => e.is_valid(),
            HasUserdata::EventInstance(e) => e.is_valid(),
            HasUserdata::CommandReplay(c) => c.is_valid(),
            // System is ALWAYS valid
            HasUserdata::System(_) => true,
            // when released, get_raw_userdata will return null
            // this is a bit of a hack though (and not very safe)
            // this should almost never be called when released though, so it should be fine...?
            _ => {
                let userdata = self.get_raw_userdata();
                userdata.is_ok_and(|ptr| ptr == key.into())
            }
        }
    }
}

pub type Userdata = Arc<dyn Any + Send + Sync + 'static>;

static STORAGE: Lazy<RwLock<UserdataStorage>> = Lazy::new(Default::default);

pub(crate) fn insert_userdata(userdata: Userdata, owner: impl Into<HasUserdata>) -> UserdataKey {
    let mut storage = STORAGE.write().unwrap();
    storage.slotmap.insert(UserdataValue {
        userdata,
        owner: owner.into(),
    })
}

pub(crate) fn remove_userdata(key: UserdataKey) -> Option<Userdata> {
    let mut storage = STORAGE.write().unwrap();
    storage.slotmap.remove(key).map(|v| v.userdata)
}

pub(crate) fn get_userdata(key: UserdataKey) -> Option<Userdata> {
    let storage = STORAGE.read().unwrap();
    storage.slotmap.get(key).map(|v| v.userdata.clone())
}

pub(crate) fn set_userdata(key: UserdataKey, userdata: Userdata) {
    let mut storage = STORAGE.write().unwrap();
    match storage.slotmap.get_mut(key) {
        Some(v) => v.userdata = userdata,
        None => eprintln!("Warning: userdata key does not exist!"),
    }
}

pub(crate) fn cleanup_userdata() {
    let mut storage = STORAGE.write().unwrap();
    storage.slotmap.retain(|k, v| v.owner.is_valid(k));
}

pub(crate) fn clear_userdata() {
    let mut storage = STORAGE.write().unwrap();
    storage.slotmap.clear();
}

impl From<UserdataKey> for *mut std::ffi::c_void {
    fn from(key: UserdataKey) -> Self {
        key.data().as_ffi() as *mut std::ffi::c_void
    }
}

impl From<*mut std::ffi::c_void> for UserdataKey {
    fn from(ptr: *mut std::ffi::c_void) -> Self {
        UserdataKey::from(KeyData::from_ffi(ptr as u64))
    }
}

impl From<StudioSystem> for HasUserdata {
    fn from(s: StudioSystem) -> Self {
        HasUserdata::StudioSystem(s)
    }
}

impl From<Bank> for HasUserdata {
    fn from(b: Bank) -> Self {
        HasUserdata::Bank(b)
    }
}

impl From<EventDescription> for HasUserdata {
    fn from(e: EventDescription) -> Self {
        HasUserdata::EventDescription(e)
    }
}

impl From<EventInstance> for HasUserdata {
    fn from(e: EventInstance) -> Self {
        HasUserdata::EventInstance(e)
    }
}

impl From<CommandReplay> for HasUserdata {
    fn from(c: CommandReplay) -> Self {
        HasUserdata::CommandReplay(c)
    }
}

impl From<Reverb3D> for HasUserdata {
    fn from(r: Reverb3D) -> Self {
        HasUserdata::Reverb3D(r)
    }
}

impl From<SoundGroup> for HasUserdata {
    fn from(s: SoundGroup) -> Self {
        HasUserdata::SoundGroup(s)
    }
}

impl From<ChannelControl> for HasUserdata {
    fn from(c: ChannelControl) -> Self {
        HasUserdata::ChannelControl(c)
    }
}

impl From<Dsp> for HasUserdata {
    fn from(d: Dsp) -> Self {
        HasUserdata::Dsp(d)
    }
}

impl From<Sound> for HasUserdata {
    fn from(s: Sound) -> Self {
        HasUserdata::Sound(s)
    }
}

impl From<Geometry> for HasUserdata {
    fn from(g: Geometry) -> Self {
        HasUserdata::Geometry(g)
    }
}

impl From<DspConnection> for HasUserdata {
    fn from(c: DspConnection) -> Self {
        HasUserdata::DspConnection(c)
    }
}

impl From<System> for HasUserdata {
    fn from(s: System) -> Self {
        HasUserdata::System(s)
    }
}
