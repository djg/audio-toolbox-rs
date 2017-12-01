use core_audio::{AudioBufferList, AudioTimeStamp, Result};
use ffi_binding::Binding;
use std::mem;
use std::os::raw::c_void;
use panic;

use audio_toolbox_sys as ffi;
use core_audio_sys as ca_ffi;

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

pub enum AudioUnitManufacturer {
    Apple,
}

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

impl Binding for AudioUnitRenderActionFlags {
    type Ffi = ffi::AudioUnitRenderActionFlags;
    fn as_ffi(&self) -> Self::Ffi {
        self.bits()
    }

    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
        Self::from_bits_truncate(ffi)
    }
}

pub struct AudioUnitProperty(ffi::AudioUnitPropertyID);

impl Binding for AudioUnitProperty {
    type Ffi = ffi::AudioUnitPropertyID;
    fn as_ffi(&self) -> Self::Ffi {
        self.0
    }
    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
        AudioUnitProperty(ffi)
    }
}

pub type AudioUnitElement = u32;
// pub type AudioUnitParameterID = u32;
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameter(ffi::AudioUnitParameterID);

impl Binding for AudioUnitParameter {
    type Ffi = ffi::AudioUnitParameterID;
    fn as_ffi(&self) -> Self::Ffi {
        self.0
    }
    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
        AudioUnitParameter(ffi)
    }
}

struct AudioUnitParameterEventRamp {
    pub startBufferOffset: i32,
    pub durationInFrames: u32,
    pub startValue: f32,
    pub endValue: f32,
}

struct AudioUnitParameterEventImmediate {
    pub bufferOffset: u32,
    pub value: f32,
}

pub enum AudioUnitParameterEventData {
    Ramp {
        startBufferOffset: i32,
        durationInFrames: u32,
        startValue: f32,
        endValue: f32,
    },
    Immediate {
        bufferOffset: u32,
        value: f32,
    }
}

pub struct AudioUnitParameterEvent {
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub parameter: AudioUnitParameter,
    pub event: AudioUnitParameterEventData,
}

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

impl Binding for AudioUnitScope {
    type Ffi = ffi::AudioUnitScope;
    fn as_ffi(&self) -> Self::Ffi {
        use AudioUnitScope::*;
        match *self {
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

    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
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

pub type AudioUnitPropertyListenerCB =
    FnMut(AudioUnit, AudioUnitProperty, AudioUnitScope, AudioUnitElement) + Send + 'static;

pub type AudioUnitRenderCB =
    FnMut(&AudioUnitRenderActionFlags, &AudioTimeStamp, u32, u32, &mut AudioBufferList) + Send;

struct CallbackThunk<T: ?Sized> { cb: Box<T> }
pub struct CallbackHandle<T: ?Sized> { thunk: *mut CallbackThunk<T> }

pub type AudioUnitRenderHandle = CallbackHandle<AudioUnitRenderCB>;
pub type AudioUnitPropertyListenerHandle = CallbackHandle<AudioUnitPropertyListenerCB>;

pub struct AudioUnit(ffi::AudioComponentInstance);

impl Binding for AudioUnit {
    type Ffi = ffi::AudioComponentInstance;

    fn as_ffi(&self) -> Self::Ffi {
        self.0
    }

    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
        AudioUnit(ffi)
    }
}

impl AudioUnit {
    pub fn initialize(&self) -> Result<()> {
        unsafe { try_call!(ffi::AudioUnitInitialize(self.as_ffi())) }
        Ok(())
    }

    pub fn uninitialize(&self) -> Result<()> {
        unsafe { try_call!(ffi::AudioUnitUninitialize(self.as_ffi())) }
        Ok(())
    }

    pub fn get_property_info(&self,
                             id: AudioUnitProperty,
                             scope: AudioUnitScope,
                             element: AudioUnitElement) -> Result<(u32, bool)> {
        let mut data_size: u32 = 0;
        let mut writable: ffi::Boolean = 0;
        unsafe {
            try_call!(ffi::AudioUnitGetPropertyInfo(self.as_ffi(),
                                                    id.as_ffi(),
                                                    scope.as_ffi(),
                                                    element,
                                                    &mut data_size as *mut _,
                                                    &mut writable as *mut _));
        }
        Ok((data_size, writable != 0))
    }

    pub fn get_property<T>(&self,
                           id: AudioUnitProperty,
                           scope: AudioUnitScope,
                           element: AudioUnitElement) -> Result<T>
        where T: Sized {
        let mut data = unsafe { mem::uninitialized() };
        let mut data_size = mem::size_of::<T>() as u32;
        unsafe {
            try_call!(ffi::AudioUnitGetProperty(self.as_ffi(),
                                                id.as_ffi(),
                                                scope.as_ffi(),
                                                element,
                                                &mut data as *mut _ as *mut _,
                                                &mut data_size));
        }
        Ok(data)
    }

    pub fn set_property<T>(&mut self,
                           id: AudioUnitProperty,
                           scope: AudioUnitScope,
                           element: AudioUnitElement,
                           data: &T) -> Result<()> {
        unsafe {
            try_call!(ffi::AudioUnitSetProperty(self.as_ffi(),
                                                id.as_ffi(),
                                                scope.as_ffi(),
                                                element,
                                                &data as *const _ as *const _,
                                                mem::size_of::<T>() as u32));
        }
        Ok(())
    }

    pub fn add_property_listener<CB>(&self, id: AudioUnitProperty, cb:CB) -> Result<AudioUnitPropertyListenerHandle>
        where CB: FnMut(AudioUnit, AudioUnitProperty, AudioUnitScope, AudioUnitElement) + Send + 'static {
        let thunk = Box::into_raw(Box::new(CallbackThunk {
            cb: Box::new(cb) as Box<AudioUnitPropertyListenerCB>
        }));
        let cb: ffi::AudioUnitPropertyListenerProc = audio_unit_property_listener;
        unsafe {
            try_call!(ffi::AudioUnitAddPropertyListener(self.as_ffi(),
                                                        id.as_ffi(),
                                                        Some(cb),
                                                        thunk as *mut _));
        }
        Ok(CallbackHandle { thunk })
    }

    pub fn remove_property_listener_with_user_data(&self, id: AudioUnitProperty,
                                                   handle: AudioUnitPropertyListenerHandle) -> Result<()> {
        let cb: ffi::AudioUnitPropertyListenerProc = audio_unit_property_listener;
        unsafe {
            try_call!(ffi::AudioUnitRemovePropertyListenerWithUserData(self.as_ffi(),
                                                                       id.as_ffi(),
                                                                       Some(cb),
                                                                       handle.thunk as *mut _));
            drop(Box::from_raw(handle.thunk));
        }
        Ok(())
    }

    pub fn add_render_notify<CB>(&self, cb: CB) -> Result<AudioUnitRenderHandle>
        where CB: FnMut(&AudioUnitRenderActionFlags, &AudioTimeStamp, u32, u32, &mut AudioBufferList) + Send + 'static
    {
        let thunk = Box::into_raw(Box::new(CallbackThunk {
            cb: Box::new(cb) as Box<AudioUnitRenderCB>
        }));
        let cb: ffi::AURenderCallback = audio_unit_render_cb;
        unsafe {
            try_call!(ffi::AudioUnitAddRenderNotify(self.as_ffi(),
                                                    Some(cb),
                                                    thunk as *mut _));
        }
        Ok(CallbackHandle { thunk: thunk })
    }

    pub unsafe fn remove_render_notify(&self, handle: AudioUnitRenderHandle) -> Result<()> {
        let cb: ffi::AURenderCallback = audio_unit_render_cb;
        unsafe {
            try_call!(ffi::AudioUnitRemoveRenderNotify(self.as_ffi(),
                                                       Some(cb),
                                                       handle.thunk as *mut _));
            drop(Box::from_raw(handle.thunk));
        }
        Ok(())
    }

    pub fn get_parameter(&self,
                         id: AudioUnitParameter,
                         scope: AudioUnitScope,
                         element: AudioUnitElement) -> Result<f32> {
        let mut data: f32 = 0.0;
        unsafe {
            try_call!(ffi::AudioUnitGetParameter(self.as_ffi(),
                                                 id.as_ffi(),
                                                 scope.as_ffi(),
                                                 element,
                                                 &mut data as *mut _));
        }
        Ok(data)
    }

    pub fn set_parameter(&self,
                         id: AudioUnitParameter,
                         scope: AudioUnitScope,
                         element: AudioUnitElement,
                         data: f32) -> Result<()> {
        unsafe {
            try_call!(ffi::AudioUnitSetParameter(self.as_ffi(),
                                                 id.as_ffi(),
                                                 scope.as_ffi(),
                                                 element,
                                                 data,
                                                 0));
        }
        Ok(())
    }

    pub fn schedule_parameters(&self, events: &[AudioUnitParameterEvent]) -> Result<()> {
        let events = Vec::new();
        unsafe {
            try_call!(ffi::AudioUnitScheduleParameters(self.as_ffi(),
                                                       events.as_ptr() as *const _,
                                                       events.len() as u32));
        };
        Ok(())
    }

    pub fn render(&self,
                  action: &mut AudioUnitRenderActionFlags,
                  time_stamp: &AudioTimeStamp,
                  output_bus_number: u32,
                  number_frames: u32,
                  data: &mut AudioBufferList) -> Result<()> {
        let mut new_action = action.as_ffi();
        unsafe {
            try_call!(ffi::AudioUnitRender(self.as_ffi(),
                                           &mut new_action,
                                           time_stamp.as_ffi(),
                                           output_bus_number,
                                           number_frames,
                                           data.as_ffi()));
            *action = AudioUnitRenderActionFlags::from_ffi(new_action);
        }
        Ok(())
    }

    pub fn process(&self,
                   action: &mut AudioUnitRenderActionFlags,
                   time_stamp: &AudioTimeStamp,
                   number_frames: u32,
                   data: &mut AudioBufferList)  -> Result<()> {
        let mut new_action = action.as_ffi();
        unsafe {
            try_call!(ffi::AudioUnitProcess(self.as_ffi(),
                                            &mut new_action,
                                            time_stamp.as_ffi(),
                                            number_frames,
                                            data.as_ffi()));
            *action = AudioUnitRenderActionFlags::from_ffi(new_action);
        }
        Ok(())
    }

    pub fn process_multiple(&self,
                            action: &mut AudioUnitRenderActionFlags,
                            time_stamp: &AudioTimeStamp,
                            number_frames: u32,
                            number_input_buffer_lists: u32,
                            input_buffer_lists: &[&AudioBufferList],
                            output_buffer_lists: &[&mut AudioBufferList]) -> Result<()> {
        /*
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
         */
        Ok(())
    }

    pub fn reset(&self, scope: AudioUnitScope, element: AudioUnitElement) -> Result<()> {
        unsafe {
            try_call!(ffi::AudioUnitReset(self.as_ffi(), scope.as_ffi(), element))
        }
        Ok(())
    }
}

pub extern fn audio_unit_render_cb(ref_con: *mut c_void,
                                   action: *mut ffi::AudioUnitRenderActionFlags,
                                   time_stamp: *const ca_ffi::AudioTimeStamp,
                                   bus_number: u32,
                                   number_frames: u32,
                                   data: *mut ca_ffi::AudioBufferList) -> ffi::OSStatus
{
    panic::wrap(|| unsafe {
        let payload = &mut *(ref_con as *mut CallbackThunk<AudioUnitRenderCB>);
        let callback = &mut payload.cb;
        let mut new_action = super::AudioUnitRenderActionFlags::from_bits_truncate(*action);
        let time_stamp = AudioTimeStamp::from_ffi(time_stamp);
        let mut data = AudioBufferList::from_ffi(data);
        callback(&mut new_action, &time_stamp, bus_number, number_frames, &mut data);
        *action = new_action.bits();
    });
    0
}

pub extern fn audio_unit_property_listener(ref_con: *mut c_void,
                                           unit: ffi::AudioUnit,
                                           id: ffi::AudioUnitPropertyID,
                                           scope: ffi::AudioUnitScope,
                                           element: ffi::AudioUnitElement) {
    panic::wrap(|| unsafe {
        let payload = &mut *(ref_con as *mut CallbackThunk<AudioUnitPropertyListenerCB>);
        let callback = &mut payload.cb;
        let unit = AudioUnit::from_ffi(unit);
        let id = AudioUnitProperty::from_ffi(id);
        let scope = AudioUnitScope::from_ffi(scope);
        callback(unit, id, scope, element);
    });
}
