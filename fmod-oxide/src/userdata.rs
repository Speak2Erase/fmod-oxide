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

use crate::studio::{Bank, CommandReplay, EventDescription, EventInstance, System as StudioSystem};

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
}

impl HasUserdata {
    fn is_valid(&self) -> bool {
        match self {
            HasUserdata::StudioSystem(s) => s.is_valid(),
            HasUserdata::Bank(b) => b.is_valid(),
            HasUserdata::EventDescription(e) => e.is_valid(),
            HasUserdata::EventInstance(e) => e.is_valid(),
            HasUserdata::CommandReplay(c) => c.is_valid(),
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
    storage.slotmap.retain(|_, v| v.owner.is_valid());
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
