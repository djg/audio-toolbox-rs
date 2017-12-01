use {AudioUnit, AudioUnitScope, AudioUnitElement, AudioUnitProperty};
use {AudioComponent, AudioComponentDescription, AudioComponentInstance};
use audio_toolbox_sys as ffi;
use core_audio::{AudioDevice, Result, AudioTimeStamp};
use std::ops;
use ffi_binding::Binding;

use AudioUnitScope::{Global, Input};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioOutputUnitSubType {
    Generic,
    VoiceProcessing,
    HAL,
    Default,
    System,
}

#[derive(Debug, PartialEq)]
pub struct AudioOutputUnit(AudioComponentInstance);

impl AudioOutputUnit {
    pub const CURRENT_DEVICE: AudioUnitProperty = ffi::kAudioOutputUnitProperty_CurrentDevice;
    pub const IS_RUNNING: AudioUnitProperty = ffi::kAudioOutputUnitProperty_IsRunning;
    pub const CHANNEL_MAP: AudioUnitProperty = ffi::kAudioOutputUnitProperty_ChannelMap;
    pub const ENABLE_IO: AudioUnitProperty = ffi::kAudioOutputUnitProperty_EnableIO;
    pub const START_TIME: AudioUnitProperty = ffi::kAudioOutputUnitProperty_StartTime;
    pub const SET_INPUT_CALLBACK: AudioUnitProperty = ffi::kAudioOutputUnitProperty_SetInputCallback;
    pub const HAS_IO: AudioUnitProperty = ffi::kAudioOutputUnitProperty_HasIO;
    pub const START_TIMESTAMPS_AT_ZERO: AudioUnitProperty = ffi::kAudioOutputUnitProperty_StartTimestampsAtZero;

    pub fn create<F>(f: F) -> Result<Option<AudioOutputUnit>>
        where F: Fn(&AudioComponent) -> bool {
        let desc = AudioComponentDescription::new(
            ffi::kAudioUnitType_Output, 0,
            ffi::kAudioUnitManufacturer_Apple);
        match AudioComponent::iter(&desc).find(|c| f(c)) {
            Some(c) => c.new().map(|ci| Some(AudioOutputUnit(ci))),
            None => Ok(None)
        }
    }
    
    pub fn start(&self) -> Result<()> {
        unsafe {
            try_call!(ffi::AudioOutputUnitStart(self.as_ffi()));
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        unsafe {
            try_call!(ffi::AudioOutputUnitStop(self.as_ffi()));
        }
        Ok(())
    }

    pub fn current_device(&self) -> Result<AudioDevice> {
        self.get_property(Self::CURRENT_DEVICE, Global, 0)
    }

    pub fn is_running(&self) -> Result<bool> {
        Ok(try!(self.get_property::<u32>(Self::IS_RUNNING, Global, 0)) != 0)
    }

    pub fn channel_map(&self, scope: AudioUnitScope, element: AudioUnitElement) -> Result<Vec<i32>> {
        self.get_property_array(Self::CHANNEL_MAP, scope, element)
    }

    pub fn enable_io(&self, scope: AudioUnitScope) -> Result<bool> {
        let element = if scope == Input { 1 } else { 0 };
        Ok(try!(self.get_property::<u32>(Self::ENABLE_IO, scope, element)) != 0)
    }

    // pub fn input_callback(&self) -> AURenderCallbackStruct {
    //     self.get_property(Self::SET_INPUT_CALLBACK, Global, 0)
    // }

    pub fn has_io(&self, scope: AudioUnitScope) -> Result<bool> {
        let element = if scope == Input { 1 } else { 0 };
        Ok(try!(self.get_property::<u32>(Self::HAS_IO, scope, element)) != 0)
    }

    pub fn start_timestamps_at_zero(&self) -> Result<bool> {
        Ok(try!(self.get_property::<u32>(Self::START_TIMESTAMPS_AT_ZERO, Global, 0)) != 0)
    }

    pub fn set_current_device(&mut self, device: &AudioDevice) -> Result<()> {
        self.set_property(Self::CURRENT_DEVICE, Global, 0, device)
    }

    pub fn set_channel_map(&mut self, scope: AudioUnitScope, element: AudioUnitElement, data: &[i32]) -> Result<()>
    {
        self.set_property_array(Self::CHANNEL_MAP, scope, element, data)
    }

    pub fn set_enable_io(&mut self, scope: AudioUnitScope, enable: bool) -> Result<()> {
        let element = if scope == Input { 1 } else { 0 };
        let data = if enable { 1u32 } else { 0u32 };
        self.set_property(Self::ENABLE_IO, scope, element, &data)
    }

    pub fn set_start_time(&mut self, start_time: &AudioTimeStamp) -> Result<()> {
        let data = ffi::AudioOutputUnitStartAtTimeParams {
            mTimestamp: unsafe { *start_time.as_ffi() },
            mFlags: 0
        };
        self.set_property(Self::START_TIME, Global, 0, &data)
    }

    pub fn set_start_timestamps_at_zero(&mut self, enable: bool) -> Result<()> {
        let data = if enable { 1u32 } else { 0u32 };
        self.set_property(Self::START_TIMESTAMPS_AT_ZERO, Global, 0, &data)
    }
}

impl Binding for AudioOutputUnit {
    type Ffi = ffi::AudioComponentInstance;

    fn as_ffi(&self) -> Self::Ffi {
        self.0.as_ffi()
    }

    unsafe fn from_ffi(_: Self::Ffi) -> Self {
        panic!("Not implemented")
    }
}

impl ops::Deref for AudioOutputUnit {
    type Target = AudioUnit;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl ops::DerefMut for AudioOutputUnit {
    fn deref_mut(&mut self) -> &mut AudioUnit {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}
