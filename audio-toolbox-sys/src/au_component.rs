use audio_component::AudioComponentInstance;
use core_audio_sys::{AudioTimeStamp, Boolean, OSStatus, AudioBufferList};

use std::mem;
use std::os::raw::c_void;

//================================================================================================
pub type AudioUnit = AudioComponentInstance;

e! {
    CF_ENUM(u32) {
        kAudioUnitType_Output                           = 0x61756f75,
        kAudioUnitType_MusicDevice                      = 0x61756d75,
        kAudioUnitType_MusicEffect                      = 0x61756d66,
        kAudioUnitType_FormatConverter                  = 0x61756663,
        kAudioUnitType_Effect                           = 0x61756678,
        kAudioUnitType_Mixer                            = 0x61756d78,
        kAudioUnitType_Panner                           = 0x6175706e,
        kAudioUnitType_Generator                        = 0x6175676e,
        kAudioUnitType_OfflineEffect                    = 0x61756f6c,
        kAudioUnitType_MIDIProcessor                    = 0x61756d69,

        kAudioUnitManufacturer_Apple                    = 0x6170706c,

        kAudioUnitSubType_GenericOutput                 = 0x67656e72,
        kAudioUnitSubType_VoiceProcessingIO             = 0x7670696f,

        kAudioUnitSubType_HALOutput                     = 0x6168616c,
        kAudioUnitSubType_DefaultOutput                 = 0x64656620,
        kAudioUnitSubType_SystemOutput                  = 0x73797320,

        kAudioUnitSubType_DLSSynth                      = 0x646c7320,
        kAudioUnitSubType_Sampler                       = 0x73616d70,
        kAudioUnitSubType_MIDISynth                     = 0x6d73796e,

        kAudioUnitSubType_AUConverter                   = 0x636f6e76,
        kAudioUnitSubType_Varispeed                     = 0x76617269,
        kAudioUnitSubType_DeferredRenderer              = 0x64656672,
        kAudioUnitSubType_Splitter                      = 0x73706c74,
        kAudioUnitSubType_MultiSplitter                 = 0x6d73706c,
        kAudioUnitSubType_Merger                        = 0x6d657267,
        kAudioUnitSubType_NewTimePitch                  = 0x6e757470,
        kAudioUnitSubType_AUiPodTimeOther               = 0x6970746f,
        kAudioUnitSubType_RoundTripAAC                  = 0x72616163,

        kAudioUnitSubType_TimePitch                     = 0x746d7074,

        kAudioUnitSubType_PeakLimiter                   = 0x6c6d7472,
        kAudioUnitSubType_DynamicsProcessor             = 0x64636d70,
        kAudioUnitSubType_LowPassFilter                 = 0x6c706173,
        kAudioUnitSubType_HighPassFilter                = 0x68706173,
        kAudioUnitSubType_BandPassFilter                = 0x62706173,
        kAudioUnitSubType_HighShelfFilter               = 0x68736866,
        kAudioUnitSubType_LowShelfFilter                = 0x6c736866,
        kAudioUnitSubType_ParametricEQ                  = 0x706d6571,
        kAudioUnitSubType_Distortion                    = 0x64697374,
        kAudioUnitSubType_Delay                         = 0x64656c79,
        kAudioUnitSubType_SampleDelay                   = 0x73646c79,
        kAudioUnitSubType_NBandEQ                       = 0x6e626571,

        kAudioUnitSubType_GraphicEQ                     = 0x67726571,
        kAudioUnitSubType_MultiBandCompressor           = 0x6d636d70,
        kAudioUnitSubType_MatrixReverb                  = 0x6d726576,
        kAudioUnitSubType_Pitch                         = 0x746d7074,
        kAudioUnitSubType_AUFilter                      = 0x66696c74,
        kAudioUnitSubType_NetSend                       = 0x6e736e64,
        kAudioUnitSubType_RogerBeep                     = 0x726f6772,

        kAudioUnitSubType_MultiChannelMixer             = 0x6d636d78,
        kAudioUnitSubType_MatrixMixer                   = 0x6d786d78,
        kAudioUnitSubType_SpatialMixer                  = 0x3364656d,

        kAudioUnitSubType_StereoMixer                   = 0x736d7872,
	kAudioUnitSubType_3DMixer                       = 0x33646d78,

        kAudioUnitSubType_SphericalHeadPanner           = 0x73706872,
        kAudioUnitSubType_VectorPanner                  = 0x76626173,
        kAudioUnitSubType_SoundFieldPanner              = 0x616d6269,
        kAudioUnitSubType_HRTFPanner                    = 0x68727466,

        kAudioUnitSubType_NetReceive                    = 0x6e726376,
        kAudioUnitSubType_ScheduledSoundPlayer          = 0x7373706c,
        kAudioUnitSubType_AudioFilePlayer               = 0x6166706c,
    };
}

//================================================================================================
e! {
    typedef CF_OPTIONS(u32, AudioUnitRenderActionFlags) {
	kAudioUnitRenderAction_PreRender                = (1 << 2),
	kAudioUnitRenderAction_PostRender		= (1 << 3),
	kAudioUnitRenderAction_OutputIsSilence		= (1 << 4),
	kAudioOfflineUnitRenderAction_Preflight		= (1 << 5),
	kAudioOfflineUnitRenderAction_Render		= (1 << 6),
	kAudioOfflineUnitRenderAction_Complete		= (1 << 7),
	kAudioUnitRenderAction_PostRenderError		= (1 << 8),
	kAudioUnitRenderAction_DoNotCheckRenderArgs	= (1 << 9)
    };
}

e! {
    CF_ENUM(OSStatus) {
	kAudioUnitErr_InvalidProperty			= -10879,
	kAudioUnitErr_InvalidParameter			= -10878,
	kAudioUnitErr_InvalidElement			= -10877,
	kAudioUnitErr_NoConnection			= -10876,
	kAudioUnitErr_FailedInitialization		= -10875,
	kAudioUnitErr_TooManyFramesToProcess            = -10874,
	kAudioUnitErr_InvalidFile			= -10871,
	kAudioUnitErr_UnknownFileType			= -10870,
	kAudioUnitErr_FileNotSpecified			= -10869,
	kAudioUnitErr_FormatNotSupported		= -10868,
	kAudioUnitErr_Uninitialized			= -10867,
	kAudioUnitErr_InvalidScope			= -10866,
	kAudioUnitErr_PropertyNotWritable		= -10865,
	kAudioUnitErr_CannotDoInCurrentContext          = -10863,
	kAudioUnitErr_InvalidPropertyValue		= -10851,
	kAudioUnitErr_PropertyNotInUse			= -10850,
	kAudioUnitErr_Initialized			= -10849,
	kAudioUnitErr_InvalidOfflineRender		= -10848,
	kAudioUnitErr_Unauthorized			= -10847,
        kAudioUnitErr_MIDIOutputBufferFull		= -66753,
        kAudioComponentErr_InstanceInvalidated          = -66749,
	kAudioUnitErr_RenderTimeout			= -66745,
	kAudioUnitErr_ExtensionNotFound			= -66744
    };
}

pub type AudioUnitPropertyID = u32;
pub type AudioUnitScope = u32;
pub type AudioUnitElement = u32;
pub type AudioUnitParameterID = u32;
pub type AudioUnitParameterValue = f32;

e! {
    typedef CF_ENUM(u32, AUParameterEventType)
    {
	kParameterEvent_Immediate	= 1,
	kParameterEvent_Ramped		= 2
    };
}

s! {
    struct AudioUnitParameterEventRamp {
        pub startBufferOffset: i32,
        pub durationInFrames: u32,
        pub startValue: AudioUnitParameterValue,
        pub endValue: AudioUnitParameterValue,
    }
}

s! {
    struct AudioUnitParameterEventImmediate {
        pub bufferOffset: u32,
        pub value: AudioUnitParameterValue,
    }
}

s! {
    struct AudioUnitParameterEvent {
        pub scope: AudioUnitScope,
        pub element: AudioUnitElement,
        pub parameter: AudioUnitParameterID,
        pub eventType: AUParameterEventType,
  	    _eventValues: [u32;4],
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

s! {
    struct AudioUnitParameter
    {
	pub mAudioUnit: AudioUnit,
	pub mParameterID: AudioUnitParameterID,
	pub mScope: AudioUnitScope,
	pub mElement: AudioUnitElement,
    }
}

s! {
    struct AudioUnitProperty
    {
	pub mAudioUnit: AudioUnit,
	pub mPropertyID: AudioUnitPropertyID,
	pub mScope: AudioUnitScope,
	pub mElement: AudioUnitElement,
    }
}

pub type AURenderCallback = extern fn(
    inRefCon: *mut c_void,
    ioActionFlags: *mut AudioUnitRenderActionFlags,
    inTimeStamp: *const AudioTimeStamp,
    inBusNumber: u32,
    inNumberFrames: u32,
    ioData: *mut AudioBufferList
) -> OSStatus;

pub type AudioUnitPropertyListenerProc = extern fn(
    inRefCon: *mut c_void,
    inUnit: AudioUnit,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement
);

pub type AUInputSamplesInOutputCallback = extern fn(
    inRefCon: *mut c_void,
    inOutputTimeStamp: *const AudioTimeStamp,
    inInputSample: f64,
    inNumberInputSamples: f64
);

//================================================================================================

extern "C" {
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

e! {
    enum {
        kAudioUnitRange							= 0x0000,	// range of selectors for audio units
	kAudioUnitInitializeSelect				= 0x0001,
	kAudioUnitUninitializeSelect			= 0x0002,
	kAudioUnitGetPropertyInfoSelect			= 0x0003,
	kAudioUnitGetPropertySelect				= 0x0004,
	kAudioUnitSetPropertySelect				= 0x0005,
	kAudioUnitAddPropertyListenerSelect		= 0x000A,
	kAudioUnitRemovePropertyListenerSelect	= 0x000B,
	kAudioUnitRemovePropertyListenerWithUserDataSelect = 0x0012,
	kAudioUnitAddRenderNotifySelect			= 0x000F,
	kAudioUnitRemoveRenderNotifySelect		= 0x0010,
	kAudioUnitGetParameterSelect			= 0x0006,
	kAudioUnitSetParameterSelect			= 0x0007,
	kAudioUnitScheduleParametersSelect		= 0x0011,
	kAudioUnitRenderSelect					= 0x000E,
	kAudioUnitResetSelect					= 0x0009,
	kAudioUnitComplexRenderSelect			= 0x0013,
	kAudioUnitProcessSelect					= 0x0014,
	kAudioUnitProcessMultipleSelect			= 0x0015
    };
}

//================================================================================================

pub type AudioUnitGetParameterProc = extern fn(
    inComponentStorage: *mut c_void,
    inID: AudioUnitParameterID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    outValue: *mut AudioUnitParameterValue
) -> OSStatus;

pub type AudioUnitSetParameterProc = extern fn(
    inComponentStorage: *mut c_void,
    inID: AudioUnitParameterID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    inValue: AudioUnitParameterValue,
    inBufferOffsetInFrames: u32
) -> OSStatus;

//================================================================================================

cfg_if! {
    if #[cfg(feature = "with-deprecated")] {
        e!{
            CF_ENUM(OSStatus) {
	        kAudioUnitErr_IllegalInstrument	        = -10873,
	        kAudioUnitErr_InstrumentTypeNotFound	= -10872,
            };
        }
    } else {}
}

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
