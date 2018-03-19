use audio_component::AudioComponentInstance;
use core_audio_sys::{AudioBufferList, AudioTimeStamp, Boolean, OSStatus};

use std::mem;
use std::os::raw::{c_double, c_float, c_void};

//================================================================================================
pub type AudioUnit = AudioComponentInstance;

pub const kAudioUnitType_Output: u32 = 1635086197;
pub const kAudioUnitType_MusicDevice: u32 = 1635085685;
pub const kAudioUnitType_MusicEffect: u32 = 1635085670;
pub const kAudioUnitType_FormatConverter: u32 = 1635083875;
pub const kAudioUnitType_Effect: u32 = 1635083896;
pub const kAudioUnitType_Mixer: u32 = 1635085688;
pub const kAudioUnitType_Panner: u32 = 1635086446;
pub const kAudioUnitType_Generator: u32 = 1635084142;
pub const kAudioUnitType_OfflineEffect: u32 = 1635086188;
pub const kAudioUnitType_MIDIProcessor: u32 = 1635085673;

pub const kAudioUnitManufacturer_Apple: u32 = 1634758764;

pub const kAudioUnitSubType_GenericOutput: u32 = 1734700658;
pub const kAudioUnitSubType_VoiceProcessingIO: u32 = 1987078511;

pub const kAudioUnitSubType_HALOutput: u32 = 1634230636;
pub const kAudioUnitSubType_DefaultOutput: u32 = 1684366880;
pub const kAudioUnitSubType_SystemOutput: u32 = 1937339168;

pub const kAudioUnitSubType_DLSSynth: u32 = 1684828960;
pub const kAudioUnitSubType_Sampler: u32 = 1935764848;
pub const kAudioUnitSubType_MIDISynth: u32 = 1836284270;

pub const kAudioUnitSubType_AUConverter: u32 = 1668247158;
pub const kAudioUnitSubType_Varispeed: u32 = 1986097769;
pub const kAudioUnitSubType_DeferredRenderer: u32 = 1684366962;
pub const kAudioUnitSubType_Splitter: u32 = 1936747636;
pub const kAudioUnitSubType_MultiSplitter: u32 = 1836281964;
pub const kAudioUnitSubType_Merger: u32 = 1835364967;
pub const kAudioUnitSubType_NewTimePitch: u32 = 1853191280;
pub const kAudioUnitSubType_AUiPodTimeOther: u32 = 1768977519;
pub const kAudioUnitSubType_RoundTripAAC: u32 = 1918984547;

pub const kAudioUnitSubType_TimePitch: u32 = 1953329268;

pub const kAudioUnitSubType_PeakLimiter: u32 = 1819112562;
pub const kAudioUnitSubType_DynamicsProcessor: u32 = 1684237680;
pub const kAudioUnitSubType_LowPassFilter: u32 = 1819304307;
pub const kAudioUnitSubType_HighPassFilter: u32 = 1752195443;
pub const kAudioUnitSubType_BandPassFilter: u32 = 1651532147;
pub const kAudioUnitSubType_HighShelfFilter: u32 = 1752393830;
pub const kAudioUnitSubType_LowShelfFilter: u32 = 1819502694;
pub const kAudioUnitSubType_ParametricEQ: u32 = 1886217585;
pub const kAudioUnitSubType_Distortion: u32 = 1684632436;
pub const kAudioUnitSubType_Delay: u32 = 1684368505;
pub const kAudioUnitSubType_SampleDelay: u32 = 1935961209;
pub const kAudioUnitSubType_NBandEQ: u32 = 1851942257;

pub const kAudioUnitSubType_GraphicEQ: u32 = 1735550321;
pub const kAudioUnitSubType_MultiBandCompressor: u32 = 1835232624;
pub const kAudioUnitSubType_MatrixReverb: u32 = 1836213622;
pub const kAudioUnitSubType_Pitch: u32 = 1953329268;
pub const kAudioUnitSubType_AUFilter: u32 = 1718185076;
pub const kAudioUnitSubType_NetSend: u32 = 1853058660;
pub const kAudioUnitSubType_RogerBeep: u32 = 1919903602;

pub const kAudioUnitSubType_MultiChannelMixer: u32 = 1835232632;
pub const kAudioUnitSubType_MatrixMixer: u32 = 1836608888;
pub const kAudioUnitSubType_SpatialMixer: u32 = 862217581;

pub const kAudioUnitSubType_StereoMixer: u32 = 1936554098;
#[cfg(feature = "deprecated")]
pub const kAudioUnitSubType_3DMixer: u32 = 862219640;

pub const kAudioUnitSubType_SphericalHeadPanner: u32 = 1936746610;
pub const kAudioUnitSubType_VectorPanner: u32 = 1986158963;
pub const kAudioUnitSubType_SoundFieldPanner: u32 = 1634558569;
pub const kAudioUnitSubType_HRTFPanner: u32 = 1752331366;

pub const kAudioUnitSubType_NetReceive: u32 = 1852990326;
pub const kAudioUnitSubType_ScheduledSoundPlayer: u32 = 1936945260;
pub const kAudioUnitSubType_AudioFilePlayer: u32 = 1634103404;

//================================================================================================

pub type AudioUnitRenderActionFlags = u32;
pub const kAudioUnitRenderAction_PreRender: u32 = (1 << 2);
pub const kAudioUnitRenderAction_PostRender: u32 = (1 << 3);
pub const kAudioUnitRenderAction_OutputIsSilence: u32 = (1 << 4);
pub const kAudioOfflineUnitRenderAction_Preflight: u32 = (1 << 5);
pub const kAudioOfflineUnitRenderAction_Render: u32 = (1 << 6);
pub const kAudioOfflineUnitRenderAction_Complete: u32 = (1 << 7);
pub const kAudioUnitRenderAction_PostRenderError: u32 = (1 << 8);
pub const kAudioUnitRenderAction_DoNotCheckRenderArgs: u32 = (1 << 9);

pub const kAudioUnitErr_InvalidProperty: OSStatus = -10879;
pub const kAudioUnitErr_InvalidParameter: OSStatus = -10878;
pub const kAudioUnitErr_InvalidElement: OSStatus = -10877;
pub const kAudioUnitErr_NoConnection: OSStatus = -10876;
pub const kAudioUnitErr_FailedInitialization: OSStatus = -10875;
pub const kAudioUnitErr_TooManyFramesToProcess: OSStatus = -10874;
pub const kAudioUnitErr_InvalidFile: OSStatus = -10871;
pub const kAudioUnitErr_UnknownFileType: OSStatus = -10870;
pub const kAudioUnitErr_FileNotSpecified: OSStatus = -10869;
pub const kAudioUnitErr_FormatNotSupported: OSStatus = -10868;
pub const kAudioUnitErr_Uninitialized: OSStatus = -10867;
pub const kAudioUnitErr_InvalidScope: OSStatus = -10866;
pub const kAudioUnitErr_PropertyNotWritable: OSStatus = -10865;
pub const kAudioUnitErr_CannotDoInCurrentContext: OSStatus = -10863;
pub const kAudioUnitErr_InvalidPropertyValue: OSStatus = -10851;
pub const kAudioUnitErr_PropertyNotInUse: OSStatus = -10850;
pub const kAudioUnitErr_Initialized: OSStatus = -10849;
pub const kAudioUnitErr_InvalidOfflineRender: OSStatus = -10848;
pub const kAudioUnitErr_Unauthorized: OSStatus = -10847;
pub const kAudioUnitErr_MIDIOutputBufferFull: OSStatus = -66753;
pub const kAudioComponentErr_InstanceInvalidated: OSStatus = -66749;
pub const kAudioUnitErr_RenderTimeout: OSStatus = -66745;
pub const kAudioUnitErr_ExtensionNotFound: OSStatus = -66744;

pub type AudioUnitPropertyID = u32;
pub type AudioUnitScope = u32;
pub type AudioUnitElement = u32;
pub type AudioUnitParameterID = u32;
pub type AudioUnitParameterValue = c_float;
pub type AUParameterEventType = u32;

pub const kParameterEvent_Immediate: u32 = 1;
pub const kParameterEvent_Ramped: u32 = 2;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterEventRamp {
    pub startBufferOffset: i32,
    pub durationInFrames: u32,
    pub startValue: AudioUnitParameterValue,
    pub endValue: AudioUnitParameterValue,
}

impl Default for AudioUnitParameterEventRamp {
    fn default() -> AudioUnitParameterEventRamp {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterEventImmediate {
    pub bufferOffset: u32,
    pub value: AudioUnitParameterValue,
}

impl Default for AudioUnitParameterEventImmediate {
    fn default() -> AudioUnitParameterEventImmediate {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterEvent {
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub parameter: AudioUnitParameterID,
    pub eventType: AUParameterEventType,
    _eventValues: [u32; 4],
}

impl Default for AudioUnitParameterEvent {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl AudioUnitParameterEvent {
    pub unsafe fn ramp(&mut self) -> *mut AudioUnitParameterEventRamp {
        mem::transmute(&self._eventValues)
    }
    pub unsafe fn immediate(&mut self) -> *mut AudioUnitParameterEventImmediate {
        mem::transmute(&self._eventValues)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameter {
    pub mAudioUnit: AudioUnit,
    pub mParameterID: AudioUnitParameterID,
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
}

impl Default for AudioUnitParameter {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitProperty {
    pub mAudioUnit: AudioUnit,
    pub mPropertyID: AudioUnitPropertyID,
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
}

impl Default for AudioUnitProperty {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type AURenderCallback = extern fn(
    inRefCon: *mut c_void,
    ioActionFlags: *mut AudioUnitRenderActionFlags,
    inTimeStamp: *const AudioTimeStamp,
    inBusNumber: u32,
    inNumberFrames: u32,
    ioData: *mut AudioBufferList,
) -> OSStatus;

pub type AudioUnitPropertyListenerProc = extern fn(
    inRefCon: *mut c_void,
    inUnit: AudioUnit,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
);

pub type AUInputSamplesInOutputCallback =
    extern fn(
        inRefCon: *mut c_void,
        inOutputTimeStamp: *const AudioTimeStamp,
        inInputSample: c_double,
        inNumberInputSamples: c_double,
    );

//================================================================================================

extern {
    pub fn AudioUnitInitialize(inUnit: AudioUnit) -> OSStatus;
    pub fn AudioUnitUninitialize(inUnit: AudioUnit) -> OSStatus;
    pub fn AudioUnitGetPropertyInfo(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outDataSize: *mut u32,
        outWritable: *mut Boolean,
    ) -> OSStatus;
    pub fn AudioUnitGetProperty(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outData: *mut c_void,
        ioDataSize: *mut u32,
    ) -> OSStatus;
    pub fn AudioUnitSetProperty(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        inData: *const c_void,
        inDataSize: u32,
    ) -> OSStatus;
    pub fn AudioUnitAddPropertyListener(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inProc: Option<AudioUnitPropertyListenerProc>,
        inProcUserData: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitRemovePropertyListenerWithUserData(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inProc: Option<AudioUnitPropertyListenerProc>,
        inProcUserData: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitAddRenderNotify(
        inUnit: AudioUnit,
        inProc: Option<AURenderCallback>,
        inProcUserData: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitRemoveRenderNotify(
        inUnit: AudioUnit,
        inProc: Option<AURenderCallback>,
        inProcUserData: *mut c_void,
    ) -> OSStatus;
    pub fn AudioUnitGetParameter(
        inUnit: AudioUnit,
        inID: AudioUnitParameterID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outValue: *mut AudioUnitParameterValue,
    ) -> OSStatus;
    pub fn AudioUnitSetParameter(
        inUnit: AudioUnit,
        inID: AudioUnitParameterID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        inValue: AudioUnitParameterValue,
        inBufferOffsetInFrames: u32,
    ) -> OSStatus;
    pub fn AudioUnitScheduleParameters(
        inUnit: AudioUnit,
        inParameterEvent: *const AudioUnitParameterEvent,
        inNumParamEvents: u32,
    ) -> OSStatus;
    pub fn AudioUnitRender(
        inUnit: AudioUnit,
        ioActionFlags: *mut AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inOutputBusNumber: u32,
        inNumberFrames: u32,
        ioData: *mut AudioBufferList,
    ) -> OSStatus;
    pub fn AudioUnitProcess(
        inUnit: AudioUnit,
        ioActionFlags: *mut AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inNumberFrames: u32,
        ioData: *mut AudioBufferList,
    ) -> OSStatus;
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
    pub fn AudioUnitReset(
        inUnit: AudioUnit,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
    ) -> OSStatus;
}

/*
#if defined(__LP64__) || TARGET_OS_IPHONE
extern OSStatus
AudioUnitExtensionSetComponentList(CFStringRef extensionIdentifier, __nullable CFArrayRef audioComponentInfo)
								API_AVAILABLE(macos(10.13), ios(11.0))
								__TVOS_PROHIBITED __WATCHOS_PROHIBITED;

extern __nullable CFArrayRef
AudioUnitExtensionCopyComponentList(CFStringRef extensionIdentifier)
								API_AVAILABLE(macos(10.13), ios(11.0))
								__TVOS_PROHIBITED __WATCHOS_PROHIBITED;
#endif
*/

// range of selectors for audio units
pub const kAudioUnitRange: u32 = 0;
pub const kAudioUnitInitializeSelect: u32 = 1;
pub const kAudioUnitUninitializeSelect: u32 = 2;
pub const kAudioUnitGetPropertyInfoSelect: u32 = 3;
pub const kAudioUnitGetPropertySelect: u32 = 4;
pub const kAudioUnitSetPropertySelect: u32 = 5;
pub const kAudioUnitAddPropertyListenerSelect: u32 = 10;
pub const kAudioUnitRemovePropertyListenerSelect: u32 = 11;
pub const kAudioUnitRemovePropertyListenerWithUserDataSelect: u32 = 18;
pub const kAudioUnitAddRenderNotifySelect: u32 = 15;
pub const kAudioUnitRemoveRenderNotifySelect: u32 = 16;
pub const kAudioUnitGetParameterSelect: u32 = 6;
pub const kAudioUnitSetParameterSelect: u32 = 7;
pub const kAudioUnitScheduleParametersSelect: u32 = 17;
pub const kAudioUnitRenderSelect: u32 = 14;
pub const kAudioUnitResetSelect: u32 = 9;
pub const kAudioUnitComplexRenderSelect: u32 = 19;
pub const kAudioUnitProcessSelect: u32 = 20;
pub const kAudioUnitProcessMultipleSelect: u32 = 21;

//================================================================================================

pub type AudioUnitGetParameterProc = extern fn(
    inComponentStorage: *mut c_void,
    inID: AudioUnitParameterID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    outValue: *mut AudioUnitParameterValue,
) -> OSStatus;

pub type AudioUnitSetParameterProc = extern fn(
    inComponentStorage: *mut c_void,
    inID: AudioUnitParameterID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    inValue: AudioUnitParameterValue,
    inBufferOffsetInFrames: u32,
) -> OSStatus;

//================================================================================================

#[cfg(feature = "deprecated")]
pub const kAudioUnitErr_IllegalInstrument: OSStatus = -10873;
#[cfg(feature = "deprecated")]
pub const kAudioUnitErr_InstrumentTypeNotFound: OSStatus = -10872;

/*
#if !TARGET_RT_64_BIT && !TARGET_OS_IPHONE
// this call is deprecated and replaced by AudioUnitRemovePropertyListenerWithUserData
// this allows apps to use the same function pointer more than once
// you provide the same function ptr and user data as provided when you add a property listener
extern OSStatus
AudioUnitRemovePropertyListener(	AudioUnit						inUnit,
									AudioUnitPropertyID				inID,
									AudioUnitPropertyListenerProc	inProc)
				__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_0,__MAC_10_5, __IPHONE_NA, __IPHONE_NA);
#endif
*/
