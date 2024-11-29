// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::{
    ffi::{c_float, c_int},
    ops::Deref,
    os::raw::c_void,
};

use crate::{Channel, ChannelControl, ChannelGroup};

#[derive(Debug, Clone, Copy)]
pub enum ChannelControlType {
    Channel(Channel),
    ChannelGroup(ChannelGroup),
}

#[allow(unused_variables)]
pub trait ChannelControlCallback {
    fn end(channel_control: ChannelControlType) -> Result<()> {
        Ok(())
    }

    fn virtual_voice(channel_control: ChannelControlType, is_virtual: bool) -> Result<()> {
        Ok(())
    }

    fn sync_point(channel_control: ChannelControlType, sync_point: c_int) -> Result<()> {
        Ok(())
    }

    // FIXME: is this &mut safe?
    fn occlusion(
        channel_control: ChannelControlType,
        direct: &mut c_float,
        reverb: &mut c_float,
    ) -> Result<()> {
        Ok(())
    }
}

impl Deref for ChannelControlType {
    type Target = ChannelControl;

    fn deref(&self) -> &Self::Target {
        match self {
            ChannelControlType::Channel(channel) => channel,
            ChannelControlType::ChannelGroup(channel_group) => channel_group,
        }
    }
}

unsafe extern "C" fn callback_impl<C: ChannelControlCallback>(
    channel_control: *mut FMOD_CHANNELCONTROL,
    control_type: FMOD_CHANNELCONTROL_TYPE,
    callback_type: FMOD_CHANNELCONTROL_CALLBACK_TYPE,
    commanddata1: *mut c_void,
    commanddata2: *mut c_void,
) -> FMOD_RESULT {
    let channel_control = match control_type {
        FMOD_CHANNELCONTROL_CHANNEL => {
            let channel = Channel::from(channel_control.cast::<FMOD_CHANNEL>());
            ChannelControlType::Channel(channel)
        }
        FMOD_CHANNELCONTROL_CHANNELGROUP => {
            let channel_group = ChannelGroup::from(channel_control.cast::<FMOD_CHANNELGROUP>());
            ChannelControlType::ChannelGroup(channel_group)
        }
        _ => return FMOD_RESULT::FMOD_ERR_INVALID_PARAM, // this should never happen
    };

    match callback_type {
        FMOD_CHANNELCONTROL_CALLBACK_END => C::end(channel_control).into(),
        FMOD_CHANNELCONTROL_CALLBACK_VIRTUALVOICE => {
            let is_virtual = unsafe { *commanddata1.cast::<i32>() } != 0;
            C::virtual_voice(channel_control, is_virtual).into()
        }
        FMOD_CHANNELCONTROL_CALLBACK_SYNCPOINT => {
            let sync_point = unsafe { *commanddata1.cast::<c_int>() };
            C::sync_point(channel_control, sync_point).into()
        }
        FMOD_CHANNELCONTROL_CALLBACK_OCCLUSION => {
            let direct = unsafe { &mut *commanddata1.cast::<c_float>() };
            let reverb = unsafe { &mut *commanddata2.cast::<c_float>() };
            C::occlusion(channel_control, &mut *direct, &mut *reverb).into()
        }
        _ => {
            eprintln!("warning: unknown callback type {callback_type}");
            FMOD_RESULT::FMOD_OK
        }
    }
}

impl ChannelControl {
    pub fn set_callback<C: ChannelControlCallback>(&self) -> Result<()> {
        unsafe {
            FMOD_ChannelControl_SetCallback(self.inner.as_ptr(), Some(callback_impl::<C>))
                .to_result()
        }
    }
}
