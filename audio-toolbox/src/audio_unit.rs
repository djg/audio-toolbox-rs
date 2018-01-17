use {AudioBufferListRef, AudioTimeStampRef, Result};
use audio_toolbox_sys as ffi;
use call;
use foreign_types::ForeignTypeRef;
use panic;
use std::mem;
use std::os::raw::c_void;
use util::component_instance_dispose;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioUnitType {
    Output,
    MusicDevice,
    MusicEffect,
    FormatConverter,
    Effect,
    Mixer,
    Panner,
    Generator,
    OfflineEffect,
    MIDIProcessor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioUnitManufacturer {
    Apple,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioUnitSubType {
    GenericOutput,
    VoiceProcessingIO,

    HALOutput,
    DefaultOutput,
    SystemOutput,

    DLSSynth,
    Sampler,
    MIDISynth,

    AUConverter,
    Varispeed,
    DeferredRenderer,
    Splitter,
    MultiSplitter,
    Merger,
    NewTimePitch,
    AUiPodTimeOther,
    RoundTripAAC,

    TimePitch,

    PeakLimiter,
    DynamicsProcessor,
    LowPassFilter,
    HighPassFilter,
    BandPassFilter,
    HighShelfFilter,
    LowShelfFilter,
    ParametricEQ,
    Distortion,
    Delay,
    SampleDelay,
    NBandEQ,

    GraphicEQ,
    MultiBandCompressor,
    MatrixReverb,
    Pitch,
    AUFilter,
    NetSend,
    RogerBeep,

    MultiChannelMixer,
    MatrixMixer,
    SpatialMixer,

    StereoMixer,
    _3DMixer,

    SphericalHeadPanner,
    VectorPanner,
    SoundFieldPanner,
    HRTFPanner,

    NetReceive,
    ScheduledSoundPlayer,
    AudioFilePlayer,
}

bitflags! {
    pub struct AudioUnitRenderActionFlags: ffi::AudioUnitRenderActionFlags {
        const PRE_RENDER = ffi::kAudioUnitRenderAction_PreRender;
        const POST_RENDER = ffi::kAudioUnitRenderAction_PostRender;
        const OUTPUT_IS_SILENCE = ffi::kAudioUnitRenderAction_OutputIsSilence;
        const PREFLIGHT = ffi::kAudioOfflineUnitRenderAction_Preflight;
        const RENDER = ffi::kAudioOfflineUnitRenderAction_Render;
        const COMPLETE = ffi::kAudioOfflineUnitRenderAction_Complete;
        const POST_RENDER_ERROR = ffi::kAudioUnitRenderAction_PostRenderError;
        const DO_NOT_CHECK_RENDER_ARGS = ffi::kAudioUnitRenderAction_DoNotCheckRenderArgs;
    }
}

impl ::std::convert::From<ffi::AudioUnitRenderActionFlags>
    for AudioUnitRenderActionFlags {
    fn from(ffi: ffi::AudioUnitRenderActionFlags) -> Self {
        Self::from_bits_truncate(ffi)
    }
}

impl ::std::convert::Into<ffi::AudioUnitRenderActionFlags>
    for AudioUnitRenderActionFlags {
    fn into(self) -> ffi::AudioUnitRenderActionFlags { self.bits() }
}

pub type AudioUnitProperty = ffi::AudioUnitPropertyID;
pub type AudioUnitElement = u32;
pub type AudioUnitParameter = ffi::AudioUnitParameterID;

pub enum AudioUnitParameterEventData {
    Ramp {
        start_buffer_offset: i32,
        duration_in_frames: u32,
        start_value: f32,
        end_value: f32,
    },
    Immediate {
        buffer_offset: u32,
        value: f32,
    },
}

pub struct AudioUnitParameterEvent {
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub parameter: AudioUnitParameter,
    pub event: AudioUnitParameterEventData,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioUnitScope {
    Global,
    Input,
    Output,
    Group,
    Part,
    Note,
    Layer,
    LayerItem,
}

impl ::std::convert::From<ffi::AudioUnitScope> for AudioUnitScope {
    /// Panics is `ffi` is an invalid value
    fn from(ffi: ffi::AudioUnitScope) -> Self {
        use AudioUnitScope::*;
        match ffi {
            ffi::kAudioUnitScope_Global => Global,
            ffi::kAudioUnitScope_Input => Input,
            ffi::kAudioUnitScope_Output => Output,
            ffi::kAudioUnitScope_Group => Group,
            ffi::kAudioUnitScope_Part => Part,
            ffi::kAudioUnitScope_Note => Note,
            ffi::kAudioUnitScope_Layer => Layer,
            ffi::kAudioUnitScope_LayerItem => LayerItem,
            _ => panic!("Unknown AudioUnitScope"),
        }
    }
}

impl ::std::convert::Into<ffi::AudioUnitScope> for AudioUnitScope {
    fn into(self) -> ffi::AudioUnitScope {
        use AudioUnitScope::*;
        match self {
            Global => ffi::kAudioUnitScope_Global,
            Input => ffi::kAudioUnitScope_Input,
            Output => ffi::kAudioUnitScope_Output,
            Group => ffi::kAudioUnitScope_Group,
            Part => ffi::kAudioUnitScope_Part,
            Note => ffi::kAudioUnitScope_Note,
            Layer => ffi::kAudioUnitScope_Layer,
            LayerItem => ffi::kAudioUnitScope_LayerItem,
        }
    }
}


pub type AudioUnitPropertyListenerCB = FnMut(&AudioUnitRef,
                                             AudioUnitProperty,
                                             AudioUnitScope,
                                             AudioUnitElement)
                                             
                                           + Send
                                           + 'static;

pub type AudioUnitRenderCB = FnMut(&AudioUnitRenderActionFlags,
                                   &AudioTimeStampRef,
                                   u32,
                                   u32,
                                   &AudioBufferListRef)
                                 + Send;

struct CallbackThunk<T: ?Sized> {
    cb: Box<T>,
}
pub struct CallbackHandle<T: ?Sized> {
    thunk: *mut CallbackThunk<T>,
}

pub type AudioUnitRenderHandle = CallbackHandle<AudioUnitRenderCB>;
pub type AudioUnitPropertyListenerHandle = CallbackHandle<AudioUnitPropertyListenerCB>;

foreign_type! {
    type CType = ffi::ComponentInstanceRecord;
    fn drop = component_instance_dispose;
    pub struct AudioUnit;
    pub struct AudioUnitRef;
}

impl AudioUnitRef {
    pub fn initialize(&self) -> Result<()> {
        unsafe { call::cvt_r(ffi::AudioUnitInitialize(self.as_ptr()))? }
        Ok(())
    }

    pub fn uninitialize(&self) -> Result<()> {
        unsafe { call::cvt_r(ffi::AudioUnitUninitialize(self.as_ptr()))? }
        Ok(())
    }

    pub fn get_property_info(&self,
                             id: AudioUnitProperty,
                             scope: AudioUnitScope,
                             element: AudioUnitElement)
                             -> Result<(u32, bool)>
    {
        let mut data_size: u32 = 0;
        let mut writable: ffi::Boolean = 0;
        unsafe {
            call::cvt_r(ffi::AudioUnitGetPropertyInfo(self.as_ptr(),
                                                      id,
                                                      scope.into(),
                                                      element,
                                                      &mut data_size as *mut _,
                                                      &mut writable as *mut _))?;
        }
        Ok((data_size, writable != 0))
    }

    pub fn get_property<T>(&self,
                           id: AudioUnitProperty,
                           scope: AudioUnitScope,
                           element: AudioUnitElement)
                           -> Result<T>
        where T: Sized
    {
        let mut data = unsafe { mem::uninitialized() };
        let mut data_size = mem::size_of::<T>() as u32;
        unsafe {
            call::cvt_r(ffi::AudioUnitGetProperty(self.as_ptr(),
                                                  id,
                                                  scope.into(),
                                                  element,
                                                  &mut data as *mut _ as *mut _,
                                                  &mut data_size))?;
        }
        Ok(data)
    }

    pub fn get_property_array<T>(&self,
                                 id: AudioUnitProperty,
                                 scope: AudioUnitScope,
                                 element: AudioUnitElement)
                                 -> Result<Vec<T>>
    {
        let (mut data_size, _) = try!(self.get_property_info(id, scope, element));
        let mut data = Vec::<T>::with_capacity(data_size as _);
        unsafe {
            call::cvt_r(ffi::AudioUnitGetProperty(self.as_ptr(),
                                                  id,
                                                  scope.into(),
                                                  element,
                                                  data.as_mut_ptr() as *mut _,
                                                  &mut data_size))?;
        }
        Ok(data)
    }


    pub fn set_property<T>(&mut self,
                           id: AudioUnitProperty,
                           scope: AudioUnitScope,
                           element: AudioUnitElement,
                           data: &T)
                           -> Result<()>
    {
        unsafe {
            call::cvt_r(ffi::AudioUnitSetProperty(self.as_ptr(),
                                                  id,
                                                  scope.into(),
                                                  element,
                                                  &data as *const _ as *const _,
                                                  mem::size_of::<T>() as u32))?;
        }
        Ok(())
    }

    pub fn set_property_array<T>(&mut self,
                                 id: AudioUnitProperty,
                                 scope: AudioUnitScope,
                                 element: AudioUnitElement,
                                 data: &[T])
                                 -> Result<()>
    {
        unsafe {
            call::cvt_r(ffi::AudioUnitSetProperty(self.as_ptr(),
                                                  id,
                                                  scope.into(),
                                                  element,
                                                  data.as_ptr() as *const _,
                                                  data.len() as u32))?;
        }
        Ok(())
    }

    pub fn add_property_listener<CB>(&self,
                                     id: AudioUnitProperty,
                                     cb: CB)
                                     -> Result<AudioUnitPropertyListenerHandle>
        where CB: FnMut(&AudioUnitRef,
                        AudioUnitProperty,
                        AudioUnitScope,
                        AudioUnitElement)
                      + Send
                      + 'static
    {
        let cb = Box::new(cb) as Box<AudioUnitPropertyListenerCB>;
        let thunk = Box::into_raw(Box::new(CallbackThunk { cb }));
        let cb: ffi::AudioUnitPropertyListenerProc = audio_unit_property_listener;
        unsafe {
            call::cvt_r(ffi::AudioUnitAddPropertyListener(self.as_ptr(),
                                                          id,
                                                          Some(cb),
                                                          thunk as *mut _))?;
        }
        Ok(CallbackHandle { thunk })
    }

    pub fn remove_property_listener_with_user_data(
        &self,
        id: AudioUnitProperty,
        handle: AudioUnitPropertyListenerHandle,
) -> Result<()>{
        let cb: ffi::AudioUnitPropertyListenerProc = audio_unit_property_listener;
        unsafe {
            call::cvt_r(ffi::AudioUnitRemovePropertyListenerWithUserData(self.as_ptr(),
                                                                         id,
                                                                         Some(cb),
                                                                         handle.thunk
                                                                         as *mut _))?;
            drop(Box::from_raw(handle.thunk));
        }
        Ok(())
    }

    pub fn add_render_notify<CB>(&self,
                                 cb: CB)
                                 -> Result<AudioUnitRenderHandle>
        where CB: FnMut(&AudioUnitRenderActionFlags,
                        &AudioTimeStampRef,
                        u32,
                        u32,
                        &AudioBufferListRef)
                      + Send
                      + 'static
    {
        let cb = Box::new(cb) as Box<AudioUnitRenderCB>;
        let thunk = Box::into_raw(Box::new(CallbackThunk { cb }));
        let cb: ffi::AURenderCallback = audio_unit_render_cb;
        unsafe {
            call::cvt_r(ffi::AudioUnitAddRenderNotify(self.as_ptr(),
                                                      Some(cb),
                                                      thunk as *mut _))?;
        }
        Ok(CallbackHandle { thunk: thunk })
    }

    pub unsafe fn remove_render_notify(&self,
                                       handle: AudioUnitRenderHandle)
                                       -> Result<()>
    {
        let cb: ffi::AURenderCallback = audio_unit_render_cb;
        call::cvt_r(ffi::AudioUnitRemoveRenderNotify(self.as_ptr(),
                                                     Some(cb),
                                                     handle.thunk as *mut _))?;
        drop(Box::from_raw(handle.thunk));
        Ok(())
    }

    pub fn get_parameter(&self,
                         id: AudioUnitParameter,
                         scope: AudioUnitScope,
                         element: AudioUnitElement)
                         -> Result<f32>
    {
        let mut data: f32 = 0.0;
        unsafe {
            call::cvt_r(ffi::AudioUnitGetParameter(self.as_ptr(),
                                                   id,
                                                   scope.into(),
                                                   element,
                                                   &mut data as *mut _))?;
        }
        Ok(data)
    }

    pub fn set_parameter(&self,
                         id: AudioUnitParameter,
                         scope: AudioUnitScope,
                         element: AudioUnitElement,
                         data: f32)
                         -> Result<()>
    {
        unsafe {
            call::cvt_r(ffi::AudioUnitSetParameter(self.as_ptr(),
                                                   id,
                                                   scope.into(),
                                                   element,
                                                   data,
                                                   0))?;
        }
        Ok(())
    }

    pub fn schedule_parameters(&self,
                               _events: &[AudioUnitParameterEvent])
                               -> Result<()>
    {
        let events = Vec::new();
        unsafe {
            call::cvt_r(ffi::AudioUnitScheduleParameters(self.as_ptr(),
                                                         events.as_ptr() as *const _,
                                                         events.len() as u32))?;
        };
        Ok(())
    }

    pub fn render(&self,
                  action: &mut AudioUnitRenderActionFlags,
                  time_stamp: &AudioTimeStampRef,
                  output_bus_number: u32,
                  number_frames: u32,
                  data: &mut AudioBufferListRef)
                  -> Result<()>
    {
        let mut new_action: ffi::AudioUnitRenderActionFlags = (*action).into();
        unsafe {
            call::cvt_r(ffi::AudioUnitRender(self.as_ptr(),
                                             &mut new_action,
                                             time_stamp.as_ptr(),
                                             output_bus_number,
                                             number_frames,
                                             data.as_ptr()))?;
            *action = AudioUnitRenderActionFlags::from(new_action);
        }
        Ok(())
    }

    pub fn process(&self,
                   action: &mut AudioUnitRenderActionFlags,
                   time_stamp: &AudioTimeStampRef,
                   number_frames: u32,
                   data: &mut AudioBufferListRef)
                   -> Result<()>
    {
        let mut new_action = (*action).into();
        unsafe {
            call::cvt_r(ffi::AudioUnitProcess(self.as_ptr(),
                                              &mut new_action,
                                              time_stamp.as_ptr(),
                                              number_frames,
                                              data.as_ptr()))?;
            *action = AudioUnitRenderActionFlags::from(new_action);
        }
        Ok(())
    }

    /*
    pub fn process_multiple(&self,
                            action: &mut AudioUnitRenderActionFlags,
                            time_stamp: &AudioTimeStamp,
                            number_frames: u32,
                            number_input_buffer_lists: u32,
                            input_buffer_lists: &[&AudioBufferList],
                            output_buffer_lists: &[&mut AudioBufferList]) -> Result<()> {
    pub fn AudioUnitProcessMultiple(
        inUnit: AudioUnit,
        ioActionFlags: *mut AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inNumberFrames: u32,
        inNumberInputBufferLists: u32,
        inInputBufferLists: *mut *const AudioBufferList,
        inNumberOutputBufferLists: u32,
        ioOutputBufferLists: *mut *mut AudioBufferList,
    ) -> OSStatus;
        Ok(())
    }
         */

    pub fn reset(&self,
                 scope: AudioUnitScope,
                 element: AudioUnitElement)
                 -> Result<()>
    {
        unsafe {
            call::cvt_r(ffi::AudioUnitReset(self.as_ptr(), scope.into(), element))?;
        }
        Ok(())
    }

    // Properties
    // kAudioUnitProperty_ClassInfo
    // kAudioUnitProperty_MakeConnection
    // kAudioUnitProperty_SampleRate
    // kAudioUnitProperty_ParameterList
    // kAudioUnitProperty_ParameterInfo
    // kAudioUnitProperty_CPULoad
    // kAudioUnitProperty_StreamFormat
    // kAudioUnitProperty_ElementCount
    // kAudioUnitProperty_Latency
    // kAudioUnitProperty_SupportedNumChannels
    // kAudioUnitProperty_MaximumFramesPerSlice
    // kAudioUnitProperty_ParameterValueStrings
    // kAudioUnitProperty_AudioChannelLayout
    //    pub fn audio_channel_layout(&self) -> AudioChannelLayout {}
    // kAudioUnitProperty_TailTime
    // kAudioUnitProperty_BypassEffect
    // kAudioUnitProperty_LastRenderError
    // kAudioUnitProperty_SetRenderCallback
    // kAudioUnitProperty_FactoryPresets
    // kAudioUnitProperty_RenderQuality
    // kAudioUnitProperty_HostCallbacks
    // kAudioUnitProperty_InPlaceProcessing
    // kAudioUnitProperty_ElementName
    // kAudioUnitProperty_SupportedChannelLayoutTags
    // kAudioUnitProperty_PresentPreset
    // kAudioUnitProperty_DependentParameters
    // kAudioUnitProperty_InputSamplesInOutput
    // kAudioUnitProperty_ShouldAllocateBuffer
    // kAudioUnitProperty_FrequencyResponse
    // kAudioUnitProperty_ParameterHistoryInfo
    // kAudioUnitProperty_NickName
    // kAudioUnitProperty_OfflineRender
    // kAudioUnitProperty_ParameterIDName
    // kAudioUnitProperty_ParameterStringFromValue
    // kAudioUnitProperty_ParameterClumpName
    // kAudioUnitProperty_ParameterValueFromString
    // kAudioUnitProperty_ContextName
    // kAudioUnitProperty_PresentationLatency
    // kAudioUnitProperty_ClassInfoFromDocument
    // kAudioUnitProperty_RequestViewController
    // kAudioUnitProperty_ParametersForOverview
    // kAudioUnitProperty_SupportsMPE
    // kAudioUnitProperty_FastDispatch
    // kAudioUnitProperty_SetExternalBuffer
    // kAudioUnitProperty_GetUIComponentList
    // kAudioUnitProperty_CocoaUI
    // kAudioUnitProperty_IconLocation
    // kAudioUnitProperty_AUHostIdentifier
    // kAudioUnitProperty_MIDIOutputCallbackInfo
    // kAudioUnitProperty_MIDIOutputCallback
}

/*
    pub fn set_input_callback<CB>(&mut self, cb: CB) -> Result<AudioUnitRenderHandle>
    where
        CB: FnMut(&AudioUnitRenderActionFlags,
              &AudioTimeStamp,
              u32,
              u32,
              &mut AudioBufferList)
            + Send
            + 'static,
    {
        let thunk = Box::into_raw(Box::new(
            CallbackThunk { cb: Box::new(cb) as Box<AudioUnitRenderCB> },
        ));

        let data = ffi::AURenderCallbackStruct {
            inputProc: Some(audio_unit_render_cb),
            inputProcRefCon: thunk as *mut _,
        };
        try!(self.set_property(
            Self::SET_INPUT_CALLBACK,
            AudioUnitScope::Global,
            0,
            &data,
        ));
        Ok(CallbackHandle { thunk: thunk })
    }
*/

pub extern fn audio_unit_render_cb(ref_con: *mut c_void,
                                   action: *mut ffi::AudioUnitRenderActionFlags,
                                   time_stamp: *const ffi::AudioTimeStamp,
                                   bus_number: u32,
                                   number_frames: u32,
                                   data: *mut ffi::AudioBufferList)
                                   -> ffi::OSStatus
{
    panic::wrap(|| unsafe {
        let payload = &mut *(ref_con as *mut CallbackThunk<AudioUnitRenderCB>);
        let callback = &mut payload.cb;
        let mut new_action =
            super::AudioUnitRenderActionFlags::from_bits_truncate(*action);
        let time_stamp = AudioTimeStampRef::from_ptr(time_stamp as _);
        let mut data = AudioBufferListRef::from_ptr(data);
        callback(&mut new_action,
                 &time_stamp,
                 bus_number,
                 number_frames,
                 &mut data);
        *action = new_action.bits();
    });
    0
}

pub extern fn audio_unit_property_listener(ref_con: *mut c_void,
                                           unit: ffi::AudioUnit,
                                           id: ffi::AudioUnitPropertyID,
                                           scope: ffi::AudioUnitScope,
                                           element: ffi::AudioUnitElement)
{
    panic::wrap(|| unsafe {
        let payload = &mut *(ref_con as *mut CallbackThunk<AudioUnitPropertyListenerCB>);
        let callback = &mut payload.cb;
        let unit = AudioUnitRef::from_ptr(unit);
        let scope = AudioUnitScope::from(scope);
        callback(unit, id, scope, element);
    });
}
