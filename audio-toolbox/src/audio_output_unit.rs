use {AudioComponent, AudioComponentDescription, AudioComponentInstance, AudioDevice,
     AudioTimeStampRef, AudioUnitElement, AudioUnitParameter, AudioUnitProperty,
     AudioUnitRef, AudioUnitScope, Result};
use AudioUnitScope::{Global, Input};
use audio_toolbox_sys as ffi;
use call;
use std::{mem, ops};
use util::component_instance_dispose;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioOutputUnitSubType {
    Generic,
    VoiceProcessing,
    HAL,
    Default,
    System,
}

ffi_type_heap! {
    type CType = ffi::ComponentInstanceRecord;
    fn drop = component_instance_dispose;
    #[derive(Debug)]
    pub struct AudioOutputUnit;
    pub struct AudioOutputUnitRef;
}

impl AudioOutputUnit {
    // Properties
    pub const CURRENT_DEVICE: AudioUnitProperty =
        ffi::kAudioOutputUnitProperty_CurrentDevice;
    pub const IS_RUNNING: AudioUnitProperty = ffi::kAudioOutputUnitProperty_IsRunning;
    pub const CHANNEL_MAP: AudioUnitProperty = ffi::kAudioOutputUnitProperty_ChannelMap;
    pub const ENABLE_IO: AudioUnitProperty = ffi::kAudioOutputUnitProperty_EnableIO;
    pub const START_TIME: AudioUnitProperty = ffi::kAudioOutputUnitProperty_StartTime;
    pub const SET_INPUT_CALLBACK: AudioUnitProperty =
        ffi::kAudioOutputUnitProperty_SetInputCallback;
    pub const HAS_IO: AudioUnitProperty = ffi::kAudioOutputUnitProperty_HasIO;
    pub const START_TIMESTAMPS_AT_ZERO: AudioUnitProperty =
        ffi::kAudioOutputUnitProperty_StartTimestampsAtZero;

    // Parameters
    pub const VOLUME: AudioUnitParameter = ffi::kHALOutputParam_Volume;

    pub fn new_instance<F>(f: F) -> Result<Option<AudioOutputUnit>>
    where
        F: Fn(&AudioComponent) -> bool,
    {
        let desc = AudioComponentDescription::new(
            ffi::kAudioUnitType_Output,
            0,
            ffi::kAudioUnitManufacturer_Apple,
        );
        match AudioComponent::iter(desc.as_ref()).find(|c| f(c)) {
            Some(c) => c.new().map(|ci| Some(ci.into())),
            None => Ok(None),
        }
    }
}

impl ::std::convert::From<AudioComponentInstance> for AudioOutputUnit {
    fn from(ci: AudioComponentInstance) -> Self {
        let ptr = ci.as_ptr();
        mem::forget(ci);
        unsafe { AudioOutputUnit::from_ptr(ptr) }
    }
}

impl AudioOutputUnitRef {
    pub fn start(&self) -> Result<()> {
        unsafe {
            call::cvt_r(ffi::AudioOutputUnitStart(self.as_ptr()))?;
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        unsafe {
            call::cvt_r(ffi::AudioOutputUnitStop(self.as_ptr()))?;
        }
        Ok(())
    }

    pub fn current_device(&self) -> Result<AudioDevice> {
        self.get_property(AudioOutputUnit::CURRENT_DEVICE, Global, 0)
    }

    pub fn is_running(&self) -> Result<bool> {
        Ok(self.get_property::<u32>(AudioOutputUnit::IS_RUNNING, Global, 0)? != 0)
    }

    pub fn channel_map(
        &self,
        scope: AudioUnitScope,
        element: AudioUnitElement,
    ) -> Result<Vec<i32>> {
        self.get_property_array(AudioOutputUnit::CHANNEL_MAP, scope, element)
    }

    pub fn enable_io(&self, scope: AudioUnitScope) -> Result<bool> {
        let element = if scope == Input { 1 } else { 0 };
        Ok(
            try!(self.get_property::<u32>(AudioOutputUnit::ENABLE_IO, scope, element))
                != 0,
        )
    }

    // pub fn input_callback(&self) -> AURenderCallbackStruct {
    //     self.get_property(AudioOutputUnit::SET_INPUT_CALLBACK, Global, 0)
    // }

    pub fn has_io(&self, scope: AudioUnitScope) -> Result<bool> {
        let element = if scope == Input { 1 } else { 0 };
        Ok(try!(self.get_property::<u32>(AudioOutputUnit::HAS_IO, scope, element)) != 0)
    }

    pub fn start_timestamps_at_zero(&self) -> Result<bool> {
        Ok(try!(self.get_property::<u32>(
            AudioOutputUnit::START_TIMESTAMPS_AT_ZERO,
            Global,
            0,
        )) != 0)
    }

    pub fn set_current_device(&mut self, device: &AudioDevice) -> Result<()> {
        self.set_property(AudioOutputUnit::CURRENT_DEVICE, Global, 0, device)
    }

    pub fn set_channel_map(
        &mut self,
        scope: AudioUnitScope,
        element: AudioUnitElement,
        data: &[i32],
    ) -> Result<()> {
        self.set_property_array(AudioOutputUnit::CHANNEL_MAP, scope, element, data)
    }

    pub fn set_enable_io(&mut self, scope: AudioUnitScope, enable: bool) -> Result<()> {
        let element = if scope == Input { 1 } else { 0 };
        let data = if enable { 1u32 } else { 0u32 };
        self.set_property(AudioOutputUnit::ENABLE_IO, scope, element, &data)
    }

    pub fn set_start_time(&mut self, start_time: &AudioTimeStampRef) -> Result<()> {
        let timestamp = unsafe { *start_time.as_ptr() };
        let data = ffi::AudioOutputUnitStartAtTimeParams {
            mTimestamp: timestamp,
            mFlags: 0,
        };
        self.set_property(AudioOutputUnit::START_TIME, Global, 0, &data)
    }

    pub fn set_start_timestamps_at_zero(&mut self, enable: bool) -> Result<()> {
        let data: u32 = if enable { 1 } else { 0 };
        self.set_property(AudioOutputUnit::START_TIMESTAMPS_AT_ZERO, Global, 0, &data)
    }

    pub fn volume(&self) -> Result<f32> {
        self.get_parameter(AudioOutputUnit::VOLUME, Global, 0)
    }

    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        self.set_parameter(AudioOutputUnit::VOLUME, Global, 0, volume)
    }
}

impl ops::Deref for AudioOutputUnitRef {
    type Target = AudioUnitRef;

    fn deref(&self) -> &Self::Target {
        unsafe { AudioUnitRef::from_ptr(self.as_ptr()) }
    }
}

impl ops::DerefMut for AudioOutputUnitRef {
    fn deref_mut(&mut self) -> &mut AudioUnitRef {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}
