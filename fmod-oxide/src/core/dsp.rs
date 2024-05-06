// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_int, c_uint},
    mem::MaybeUninit,
};

use fmod_sys::*;

use crate::{
    ChannelMask, DspConnection, DspConnectionType, DspMeteringInfo, DspParameterDataType,
    DspParameterDescription, DspType, SpeakerMode, System,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // so we can transmute between types
pub struct Dsp {
    pub(crate) inner: *mut FMOD_DSP,
}

unsafe impl Send for Dsp {}
unsafe impl Sync for Dsp {}

impl From<*mut FMOD_DSP> for Dsp {
    fn from(value: *mut FMOD_DSP) -> Self {
        Dsp { inner: value }
    }
}

impl From<Dsp> for *mut FMOD_DSP {
    fn from(value: Dsp) -> Self {
        value.inner
    }
}

impl Dsp {
    /// Adds a [`Dsp`] unit as an input to this object.
    ///
    /// When a [`Dsp`] has multiple inputs the signals are automatically mixed together, sent to the unit's output(s).
    ///
    /// The returned [`DspConnection`] will remain valid until the units are disconnected.
    pub fn add_input(&self, input: Dsp, kind: DspConnectionType) -> Result<DspConnection> {
        let mut connection = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_AddInput(self.inner, input.inner, &mut connection, kind.into()).to_result()?;
        };
        Ok(connection.into())
    }

    /// Retrieves the [`Dsp`] unit at the specified index in the input list.
    ///
    /// This will flush the [`Dsp`] queue (which blocks against the mixer) to ensure the input list is correct, avoid this during time sensitive operations.
    ///
    /// The returned [`DspConnection`] will remain valid until the units are disconnected.
    pub fn get_input(&self, index: c_int) -> Result<(Dsp, DspConnection)> {
        let mut connection = std::ptr::null_mut();
        let mut dsp = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetInput(self.inner, index, &mut dsp, &mut connection).to_result()?;
        };
        Ok((dsp.into(), connection.into()))
    }

    /// Retrieves the [`Dsp`] unit at the specified index in the output list.
    ///
    /// This will flush the [`Dsp`] queue (which blocks against the mixer) to ensure the output list is correct, avoid this during time sensitive operations.
    ///
    /// The returned [`DspConnection`] will remain valid until the units are disconnected.
    pub fn get_output(&self, index: c_int) -> Result<(Dsp, DspConnection)> {
        let mut connection = std::ptr::null_mut();
        let mut dsp = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetOutput(self.inner, index, &mut dsp, &mut connection).to_result()?;
        };
        Ok((dsp.into(), connection.into()))
    }

    /// Retrieves the number of [`Dsp`] units in the input list.
    ///
    /// This will flush the [`Dsp`] queue (which blocks against the mixer) to ensure the input list is correct, avoid this during time sensitive operations.
    pub fn get_input_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_DSP_GetNumInputs(self.inner, &mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves the number of [`Dsp`] units in the output list.
    ///
    /// This will flush the [`Dsp`] queue (which blocks against the mixer) to ensure the output list is correct, avoid this during time sensitive operations.
    pub fn get_output_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_DSP_GetNumOutputs(self.inner, &mut count).to_result()? };
        Ok(count)
    }

    /// Disconnects all inputs and/or outputs.
    ///
    /// This is a convenience function that is faster than disconnecting all inputs and outputs individually.
    pub fn disconnect_all(&self, inputs: bool, outputs: bool) -> Result<()> {
        unsafe { FMOD_DSP_DisconnectAll(self.inner, inputs.into(), outputs.into()).to_result() }
    }

    /// Disconnect the specified input [`Dsp`].
    ///
    /// If target had only one output, after this operation that entire sub graph will no longer be connected to the [`Dsp`] network.
    ///
    /// After this operation `connection` is no longer valid.
    pub fn disconnect_from(
        &self,
        target: Option<Dsp>,
        connection: Option<DspConnection>,
    ) -> Result<()> {
        let target = target.map_or(std::ptr::null_mut(), Into::into);
        let connection = connection.map_or(std::ptr::null_mut(), Into::into);
        unsafe { FMOD_DSP_DisconnectFrom(self.inner, target, connection).to_result() }
    }

    /// Retrieve the index of the first data parameter of a particular data type.
    ///
    /// This function returns [`Ok`] if a parmeter of matching type is found and [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`] if no matches were found.
    ///
    /// The return code can be used to check whether the [`Dsp`] supports specific functionality through data parameters of certain types without the need to provide index.
    pub fn get_data_parameter_index(&self, data_type: DspParameterDataType) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_DSP_GetDataParameterIndex(self.inner, data_type.into(), &mut index).to_result()?;
        }
        Ok(index)
    }

    /// Retrieves the number of parameters exposed by this unit.
    ///
    /// Use this to enumerate all parameters of a [`Dsp`] unit with [`Dsp::get_parameter_info`].
    pub fn get_parameter_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_DSP_GetNumParameters(self.inner, &mut count).to_result()? };
        Ok(count)
    }

    /// Sets a boolean parameter by index.
    pub fn set_parameter_bool(&self, index: c_int, value: bool) -> Result<()> {
        unsafe { FMOD_DSP_SetParameterBool(self.inner, index, value.into()).to_result() }
    }

    /// Retrieves a boolean parameter by index.
    ///
    /// Note: FMOD also returns a string representation of the parameter value, but this is not currently exposed.
    /// You can just use [`ToString`] instead.
    // FIXME turn this into an enum sorta thing?
    pub fn get_parameter_bool(&self, index: c_int) -> Result<bool> {
        let mut value = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_DSP_GetParameterBool(self.inner, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
        }
        Ok(value.into())
    }

    /// Sets a binary data parameter by index.
    ///
    /// Certain data types are predefined by the system and can be specified via the FMOD_DSP_PARAMETER_DESC_DATA, see FMOD_DSP_PARAMETER_DATA_TYPE
    ///
    /// # Safety
    ///
    /// You must ensure that the data type passed in via `data` matches the data type expected by the [`Dsp`] unit.
    // FIXME: does FMOD copy the data?
    // FIXME: fmod has various predefined data types, should we expose them?
    pub unsafe fn set_parameter_data(&self, index: c_int, data: &[u8]) -> Result<()> {
        unsafe {
            FMOD_DSP_SetParameterData(
                self.inner,
                index,
                data.as_ptr() as *mut _,
                data.len() as c_uint,
            )
            .to_result()
        }
    }

    /// Retrieves a binary data parameter by index.
    ///
    /// Note: FMOD also returns a string representation of the parameter value, but this is not currently exposed.
    // FIXME is this safe???
    pub fn get_parameter_data(&self, index: c_int) -> Result<Vec<u8>> {
        let mut value = std::ptr::null_mut();
        let mut length = 0;
        unsafe {
            FMOD_DSP_GetParameterData(
                self.inner,
                index,
                &mut value,
                &mut length,
                std::ptr::null_mut(),
                0,
            )
            .to_result()?;

            let slice = std::slice::from_raw_parts_mut(value.cast(), length as usize);
            Ok(slice.to_vec())
        }
    }

    /// Sets a floating point parameter by index.
    pub fn set_parameter_float(&self, index: c_int, value: c_float) -> Result<()> {
        unsafe { FMOD_DSP_SetParameterFloat(self.inner, index, value).to_result() }
    }

    /// Retrieves a floating point parameter by index.
    ///
    /// Note: FMOD also returns a string representation of the parameter value, but this is not currently exposed.
    /// You can just use [`ToString`] instead.
    pub fn get_parameter_float(&self, index: c_int) -> Result<c_float> {
        let mut value = 0.0;
        unsafe {
            FMOD_DSP_GetParameterFloat(self.inner, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
        }
        Ok(value)
    }

    /// Sets an integer parameter by index.
    pub fn set_parameter_int(&self, index: c_int, value: c_int) -> Result<()> {
        unsafe { FMOD_DSP_SetParameterInt(self.inner, index, value).to_result() }
    }

    /// Retrieves an integer parameter by index.
    ///
    /// Note: FMOD also returns a string representation of the parameter value, but this is not currently exposed.
    /// You can just use [`ToString`] instead.
    pub fn get_parameter_int(&self, index: c_int) -> Result<c_int> {
        let mut value = 0;
        unsafe {
            FMOD_DSP_GetParameterInt(self.inner, index, &mut value, std::ptr::null_mut(), 0)
                .to_result()?;
        }
        Ok(value)
    }

    /// Retrieve information about a specified parameter.
    pub fn get_parameter_info(&self, index: c_int) -> Result<DspParameterDescription> {
        let mut desc = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetParameterInfo(self.inner, index, &mut desc).to_result()?;
            let desc = DspParameterDescription::from_ffi(*desc); // oh god this is *awful*
            Ok(desc)
        }
    }

    /// Sets the PCM input format this [`Dsp`] will receive when processing.
    ///
    /// Setting the number of channels on a unit will force either a down or up mix to that channel count before processing the [`Dsp`] read/process callback.
    pub fn set_channel_format(
        &self,
        channel_mask: ChannelMask,
        channel_count: c_int,
        source_speaker_mode: SpeakerMode,
    ) -> Result<()> {
        unsafe {
            FMOD_DSP_SetChannelFormat(
                self.inner,
                channel_mask.into(),
                channel_count,
                source_speaker_mode.into(),
            )
            .to_result()
        }
    }

    /// Retrieves the PCM input format this [`Dsp`] will receive when processing.
    pub fn get_channel_format(&self) -> Result<(ChannelMask, c_int, SpeakerMode)> {
        let mut channel_mask = 0;
        let mut channel_count = 0;
        let mut source_speaker_mode = 0;
        unsafe {
            FMOD_DSP_GetChannelFormat(
                self.inner,
                &mut channel_mask,
                &mut channel_count,
                &mut source_speaker_mode,
            )
            .to_result()?;
        }
        let source_speaker_mode = source_speaker_mode.try_into()?;
        Ok((channel_mask.into(), channel_count, source_speaker_mode))
    }

    /// Retrieves the output format this [`Dsp`] will produce when processing based on the input specified.
    pub fn get_output_channel_format(
        &self,
        in_mask: ChannelMask,
        in_channels: c_int,
        in_speaker_mode: SpeakerMode,
    ) -> Result<(ChannelMask, c_int, SpeakerMode)> {
        let mut out_mask = 0;
        let mut out_channels = 0;
        let mut out_speaker_mode = 0;
        unsafe {
            FMOD_DSP_GetOutputChannelFormat(
                self.inner,
                in_mask.into(),
                in_channels,
                in_speaker_mode.into(),
                &mut out_mask,
                &mut out_channels,
                &mut out_speaker_mode,
            )
            .to_result()?;
        }
        let out_speaker_mode = out_speaker_mode.try_into()?;
        Ok((out_mask.into(), out_channels, out_speaker_mode))
    }

    /// Retrieve the signal metering information.
    ///
    /// Requesting metering information when it hasn't been enabled will result in [`FMOD_RESULT::FMOD_ERR_BADCOMMAND`].
    ///
    /// FMOD_INIT_PROFILE_METER_ALL with System::init will automatically enable metering for all [`Dsp`] units.
    pub fn get_metering_info(&self) -> Result<(DspMeteringInfo, DspMeteringInfo)> {
        let mut input = MaybeUninit::zeroed();
        let mut output = MaybeUninit::zeroed();
        unsafe {
            FMOD_DSP_GetMeteringInfo(self.inner, input.as_mut_ptr(), output.as_mut_ptr())
                .to_result()?;
            let input = input.assume_init().into();
            let output = output.assume_init().into();
            Ok((input, output))
        }
    }

    /// Sets the input and output signal metering enabled states.
    ///
    /// Input metering is pre processing, while output metering is post processing.
    ///
    /// Enabled metering allows [`Dsp`]::getMeteringInfo to return metering information and allows FMOD profiling tools to visualize the levels.
    ///
    /// FMOD_INIT_PROFILE_METER_ALL with System::init will automatically turn on metering for all [`Dsp`] units inside the mixer graph.
    ///
    /// This function must have inputEnabled and outputEnabled set to true if being used by the FMOD Studio API,
    /// such as in the Unity or Unreal Engine integrations, in order to avoid conflict with FMOD Studio's live update feature.
    pub fn set_metering_enabled(&self, input_enabled: bool, output_enabled: bool) -> Result<()> {
        unsafe {
            FMOD_DSP_SetMeteringEnabled(self.inner, input_enabled.into(), output_enabled.into())
                .to_result()
        }
    }

    /// Retrieves the input and output signal metering enabled states.
    ///
    /// Input metering is pre processing, while output metering is post processing.
    ///
    /// Enabled metering allows [`Dsp`]::getMeteringInfo to return metering information and allows FMOD profiling tools to visualize the levels.
    ///
    /// FMOD_INIT_PROFILE_METER_ALL with System::init will automatically turn on metering for all [`Dsp`] units inside the mixer graph.
    pub fn get_metering_enabled(&self) -> Result<(bool, bool)> {
        let mut input_enabled = FMOD_BOOL::FALSE;
        let mut output_enabled = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_DSP_GetMeteringEnabled(self.inner, &mut input_enabled, &mut output_enabled)
                .to_result()?;
        }
        Ok((input_enabled.into(), output_enabled.into()))
    }

    /// Sets the processing active state.
    ///
    /// If active is false, processing of this unit and its inputs are stopped.
    ///
    /// When created a [`Dsp`] is inactive. If ChannelControl::addDSP is used it will automatically be activated, otherwise it must be set to active manually.
    pub fn set_active(&self, active: bool) -> Result<()> {
        unsafe { FMOD_DSP_SetActive(self.inner, active.into()).to_result() }
    }

    /// Retrieves the processing active state.
    ///
    /// If active is False, processing of this unit and its inputs are stopped.
    ///
    /// When created a [`Dsp`] is inactive.
    /// If ChannelControl::addDSP is used it will automatically be activated, otherwise it must be set to active manually.
    pub fn get_active(&self) -> Result<bool> {
        let mut active = FMOD_BOOL::FALSE;
        unsafe { FMOD_DSP_GetActive(self.inner, &mut active).to_result()? };
        Ok(active.into())
    }

    /// Sets the scale of the wet and dry signal components.
    ///
    /// The dry signal path is silent by default, because dsp effects transform the input and pass the newly processed result to the output.
    pub fn set_wet_dry_mix(&self, pre_wet: c_float, post_wet: c_float, dry: c_float) -> Result<()> {
        unsafe { FMOD_DSP_SetWetDryMix(self.inner, pre_wet, post_wet, dry).to_result() }
    }

    /// Retrieves the scale of the wet and dry signal components.
    pub fn get_wet_dry_mix(&self) -> Result<(c_float, c_float, c_float)> {
        let mut pre_wet = 0.0;
        let mut post_wet = 0.0;
        let mut dry = 0.0;
        unsafe {
            FMOD_DSP_GetWetDryMix(self.inner, &mut pre_wet, &mut post_wet, &mut dry).to_result()?;
        }
        Ok((pre_wet, post_wet, dry))
    }

    /// Retrieves the idle state.
    ///
    /// A [`Dsp`] is considered idle when it stops receiving input signal and all internal processing of stored input has been exhausted.
    ///
    /// Each [`Dsp`] type has the potential to have differing idle behaviour based on the type of effect.
    /// A reverb or echo may take a longer time to go idle after it stops receiving a valid signal, compared to an effect with a shorter tail length like an EQ filter.
    pub fn get_idle(&self) -> Result<bool> {
        let mut idle = FMOD_BOOL::FALSE;
        unsafe { FMOD_DSP_GetIdle(self.inner, &mut idle).to_result()? };
        Ok(idle.into())
    }

    // TODO show dialogue config

    /// Reset a DSPs internal state ready for new input signal.
    ///
    /// This will clear all internal state derived from input signal while retaining any set parameter values.
    /// The intended use of the function is to avoid audible artifacts if moving the [`Dsp`] from one part of the [`Dsp`] network to another.
    pub fn reset(&self) -> Result<()> {
        unsafe { FMOD_DSP_Reset(self.inner).to_result() }
    }

    /// Frees a [`Dsp`] object.
    ///
    /// If [`Dsp`] is not removed from the network with ChannelControl::removeDSP after being added with ChannelControl::addDSP,
    /// it will not release and will instead return [`FMOD_RESULT::FMOD_ERR_DSP_INUSE`].
    pub fn release(self) -> Result<()> {
        unsafe { FMOD_DSP_Release(self.inner).to_result() }
    }

    /// Retrieves the pre-defined type of a FMOD registered [`Dsp`] unit.
    pub fn get_type(&self) -> Result<DspType> {
        let mut dsp_type = 0;
        unsafe { FMOD_DSP_GetType(self.inner, &mut dsp_type).to_result()? };
        let dsp_type = dsp_type.try_into()?;
        Ok(dsp_type)
    }

    // TODO getinfo

    /// Retrieves statistics on the mixer thread CPU usage for this unit.
    ///
    /// [`crate::InitFlags::PROFILE_ENABLE`] with [`crate::SystemBuilder::new`] is required to call this function.
    pub fn get_cpu_usage(&self) -> Result<(c_uint, c_uint)> {
        let mut exclusive = 0;
        let mut inclusive = 0;
        unsafe {
            FMOD_DSP_GetCPUUsage(self.inner, &mut exclusive, &mut inclusive).to_result()?;
        }
        Ok((exclusive, inclusive))
    }

    // TODO userdata

    // TODO callback

    /// Retrieves the parent System object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_DSP_GetSystemObject(self.inner, &mut system).to_result()? };
        Ok(system.into())
    }
}
