use {OSStatus, Boolean, CFStringRef, CFURLRef};

use au_component::{AudioUnitParameterID, AudioUnitPropertyID, AURenderCallback, AudioUnitElement,
                   AUInputSamplesInOutputCallback, AudioUnit, AudioUnitParameterValue,
                   AudioUnitScope};
use core_audio_sys::{AudioClassDescription, AudioBufferList, AudioTimeStamp};

use std::mem;
use std::os::raw::{c_char, c_void};

e! {
    CF_ENUM(AudioUnitScope) {
        kAudioUnitScope_Global		= 0,
        kAudioUnitScope_Input		= 1,
        kAudioUnitScope_Output		= 2,
        kAudioUnitScope_Group		= 3,
        kAudioUnitScope_Part		= 4,
        kAudioUnitScope_Note		= 5,
        kAudioUnitScope_Layer		= 6,
        kAudioUnitScope_LayerItem	= 7
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID)
    {
        // range (0 -> 999)
        kAudioUnitProperty_ClassInfo			= 0,
        kAudioUnitProperty_MakeConnection		= 1,
        kAudioUnitProperty_SampleRate			= 2,
        kAudioUnitProperty_ParameterList		= 3,
        kAudioUnitProperty_ParameterInfo		= 4,
        kAudioUnitProperty_CPULoad			= 6,
        kAudioUnitProperty_StreamFormat			= 8,
        kAudioUnitProperty_ElementCount			= 11,
        kAudioUnitProperty_Latency			= 12,
        kAudioUnitProperty_SupportedNumChannels		= 13,
        kAudioUnitProperty_MaximumFramesPerSlice	= 14,
        kAudioUnitProperty_ParameterValueStrings	= 16,
        kAudioUnitProperty_AudioChannelLayout		= 19,
        kAudioUnitProperty_TailTime			= 20,
        kAudioUnitProperty_BypassEffect			= 21,
        kAudioUnitProperty_LastRenderError		= 22,
        kAudioUnitProperty_SetRenderCallback		= 23,
        kAudioUnitProperty_FactoryPresets		= 24,
        kAudioUnitProperty_RenderQuality		= 26,
        kAudioUnitProperty_HostCallbacks		= 27,
        kAudioUnitProperty_InPlaceProcessing		= 29,
        kAudioUnitProperty_ElementName			= 30,
        kAudioUnitProperty_SupportedChannelLayoutTags	= 32,
        kAudioUnitProperty_PresentPreset		= 36,
        kAudioUnitProperty_DependentParameters		= 45,
        kAudioUnitProperty_InputSamplesInOutput		= 49,
        kAudioUnitProperty_ShouldAllocateBuffer		= 51,
        kAudioUnitProperty_FrequencyResponse		= 52,
        kAudioUnitProperty_ParameterHistoryInfo		= 53,
        kAudioUnitProperty_NickName                     = 54,
        kAudioUnitProperty_OfflineRender		= 37,
        kAudioUnitProperty_ParameterIDName		= 34,
        kAudioUnitProperty_ParameterStringFromValue	= 33,
        kAudioUnitProperty_ParameterClumpName		= 35,
        kAudioUnitProperty_ParameterValueFromString	= 38,
        kAudioUnitProperty_ContextName			= 25,
        kAudioUnitProperty_PresentationLatency		= 40,
        kAudioUnitProperty_ClassInfoFromDocument	= 50,
        kAudioUnitProperty_RequestViewController	= 56,
        kAudioUnitProperty_ParametersForOverview	= 57,
        kAudioUnitProperty_SupportsMPE			= 58,

        kAudioUnitProperty_FastDispatch			= 5,
        kAudioUnitProperty_SetExternalBuffer		= 15,
        kAudioUnitProperty_GetUIComponentList		= 18,
        kAudioUnitProperty_CocoaUI			= 31,
        kAudioUnitProperty_IconLocation			= 39,
        kAudioUnitProperty_AUHostIdentifier		= 46,

        kAudioUnitProperty_MIDIOutputCallbackInfo       = 47,
        kAudioUnitProperty_MIDIOutputCallback           = 48,
    };
}

cs! {
    #define kAUPresetVersionKey                         "version"
    #define kAUPresetTypeKey                            "type"
    #define kAUPresetSubtypeKey                         "subtype"
    #define kAUPresetManufacturerKey                    "manufacturer"
    #define kAUPresetDataKey                            "data"
    #define kAUPresetNameKey                            "name"
    #define kAUPresetRenderQualityKey                   "render-quality"
    #define kAUPresetCPULoadKey                         "cpu-load"
    #define kAUPresetElementNameKey                     "element-name"
    #define kAUPresetExternalFileRefs                   "file-references"

    #define kAUPresetVSTDataKey                         "vstdata"
    #define kAUPresetVSTPresetKey                       "vstpreset"

    #define kAUPresetMASDataKey                         "masdata"

    #define kAUPresetPartKey                            "part"
}

s! {
    #[derive(Clone, Copy)]
    struct AudioUnitConnection {
        pub sourceAudioUnit: AudioUnit,
        pub sourceOutputNumber: u32,
        pub destInputNumber: u32,
    }

    #[derive(Clone, Copy)]
    struct AUChannelInfo {
        pub inChannels: i16,
        pub outChannels: i16,
    }

    #[derive(Clone, Copy)]
    struct AudioUnitExternalBuffer {
        pub buffer: *mut u8,
        pub size: u32,
    }

    #[derive(Clone, Copy)]
    struct AURenderCallbackStruct {
        pub inputProc: AURenderCallback,
        pub inputProcRefCon: *mut c_void,
    }

    #[derive(Clone, Copy)]
    struct Struct_AUPreset {
        pub presetNumber: i32,
        pub presetName: CFStringRef,
    }
}

e! {
    enum {
        kRenderQuality_Max				= 127,
        kRenderQuality_High				= 96,
        kRenderQuality_Medium				= 64,
        kRenderQuality_Low				= 32,
        kRenderQuality_Min				= 0
    };
}

e! {
    enum {
        kNumberOfResponseFrequencies = 1024
    };
}

s! {
    #[derive(Clone, Copy)]
    struct AudioUnitFrequencyResponseBin {
        pub mFrequency: f64,
        pub mMagnitude: f64,
    }
}

pub type HostCallback_GetBeatAndTempo = Option<
    extern "C" fn(inHostUserData: *mut c_void,
                  outCurrentBeat: *mut f64,
                  outCurrentTempo: *mut f64)
                  -> OSStatus,
>;
pub type HostCallback_GetMusicalTimeLocation =
    Option<
        extern "C" fn(inHostUserData: *mut c_void,
                      outDeltaSampleOffsetToNextBeat: *mut u32,
                      outTimeSig_Numerator: *mut f32,
                      outTimeSig_Denominator: *mut u32,
                      outCurrentMeasureDownBeat: *mut f64)
                      -> OSStatus,
    >;
pub type HostCallback_GetTransportState =
    Option<
        extern "C" fn(inHostUserData: *mut c_void,
                      outIsPlaying: *mut Boolean,
                      outTransportStateChanged: *mut Boolean,
                      outCurrentSampleInTimeLine: *mut f64,
                      outIsCycling: *mut Boolean,
                      outCycleStartBeat: *mut f64,
                      outCycleEndBeat: *mut f64)
                      -> OSStatus,
    >;
pub type HostCallback_GetTransportState2 =
    Option<
        extern "C" fn(inHostUserData: *mut c_void,
                      outIsPlaying: *mut Boolean,
                      outIsRecording: *mut Boolean,
                      outTransportStateChanged: *mut Boolean,
                      outCurrentSampleInTimeLine: *mut f64,
                      outIsCycling: *mut Boolean,
                      outCycleStartBeat: *mut f64,
                      outCycleEndBeat: *mut f64)
                      -> OSStatus,
    >;

s! {
    #[derive(Clone, Copy)]
    struct HostCallbackInfo {
        pub hostUserData: *mut c_void,
        pub beatAndTempoProc: HostCallback_GetBeatAndTempo,
        pub musicalTimeLocationProc: HostCallback_GetMusicalTimeLocation,
        pub transportStateProc: HostCallback_GetTransportState,
        pub transportStateProc2: HostCallback_GetTransportState2,
    }

    #[derive(Clone, Copy)]
    struct AUDependentParameter {
        pub mScope: AudioUnitScope,
        pub mParameterID: AudioUnitParameterID,
    }

    #[derive(Clone, Copy)]
    struct AudioUnitCocoaViewInfo {
        pub mCocoaAUViewBundleLocation: CFURLRef,
        pub mCocoaAUViewClass: [CFStringRef; 1usize],
    }

    #[derive(Clone, Copy)]
    struct AUHostVersionIdentifier {
        pub hostName: CFStringRef,
        pub hostVersion: u32,
    }
}

pub enum MIDIPacketList { }
pub type AUMIDIOutputCallback = extern fn(
    userData: *mut c_void,
    timeStamp: *const AudioTimeStamp,
    midiOutNum: u32,
    pktlist: *const MIDIPacketList)
    -> OSStatus;

s! {
    #[derive(Clone, Copy)]
    struct AUMIDIOutputCallbackStruct {
        pub midiOutputCallback: Option<AUMIDIOutputCallback>,
        pub userData: *mut c_void,
    }

    #[derive(Clone, Copy)]
    struct AUInputSamplesInOutputCallbackStruct {
        pub inputToOutputCallback: Option<AUInputSamplesInOutputCallback>,
        pub userData: *mut c_void,
    }

    #[derive(Clone, Copy)]
    struct AudioUnitParameterHistoryInfo {
        pub updatesPerSecond: f32,
        pub historyDurationInSeconds: f32,
    }
}


e! {
    typedef CF_ENUM(u32, AudioUnitParameterUnit)
    {
        kAudioUnitParameterUnit_Generic			= 0,
        kAudioUnitParameterUnit_Indexed			= 1,
        kAudioUnitParameterUnit_Boolean			= 2,
        kAudioUnitParameterUnit_Percent			= 3,
        kAudioUnitParameterUnit_Seconds			= 4,
        kAudioUnitParameterUnit_SampleFrames		= 5,
        kAudioUnitParameterUnit_Phase			= 6,
        kAudioUnitParameterUnit_Rate			= 7,
        kAudioUnitParameterUnit_Hertz			= 8,
        kAudioUnitParameterUnit_Cents			= 9,
        kAudioUnitParameterUnit_RelativeSemiTones	= 10,
        kAudioUnitParameterUnit_MIDINoteNumber		= 11,
        kAudioUnitParameterUnit_MIDIController		= 12,
        kAudioUnitParameterUnit_Decibels		= 13,
        kAudioUnitParameterUnit_LinearGain		= 14,
        kAudioUnitParameterUnit_Degrees			= 15,
        kAudioUnitParameterUnit_EqualPowerCrossfade     = 16,
        kAudioUnitParameterUnit_MixerFaderCurve1	= 17,
        kAudioUnitParameterUnit_Pan			= 18,
        kAudioUnitParameterUnit_Meters			= 19,
        kAudioUnitParameterUnit_AbsoluteCents		= 20,
        kAudioUnitParameterUnit_Octaves			= 21,
        kAudioUnitParameterUnit_BPM			= 22,
        kAudioUnitParameterUnit_Beats                   = 23,
        kAudioUnitParameterUnit_Milliseconds		= 24,
        kAudioUnitParameterUnit_Ratio			= 25,
        kAudioUnitParameterUnit_CustomUnit		= 26
    };
}

e! {
    typedef CF_OPTIONS(u32, AudioUnitParameterOptions)
    {
        kAudioUnitParameterFlag_CFNameRelease		= (1 << 4),

        kAudioUnitParameterFlag_OmitFromPresets		= (1 << 13),
        kAudioUnitParameterFlag_PlotHistory		= (1 << 14),
        kAudioUnitParameterFlag_MeterReadOnly		= (1 << 15),

        // bit positions 18,17,16 are set aside for display scales. bit 19 is reserved.
        kAudioUnitParameterFlag_DisplayMask		= (7 << 16) | (1 << 22),
        kAudioUnitParameterFlag_DisplaySquareRoot	= (1 << 16),
        kAudioUnitParameterFlag_DisplaySquared		= (2 << 16),
        kAudioUnitParameterFlag_DisplayCubed		= (3 << 16),
        kAudioUnitParameterFlag_DisplayCubeRoot		= (4 << 16),
        kAudioUnitParameterFlag_DisplayExponential	= (5 << 16),

        kAudioUnitParameterFlag_HasClump                = (1 << 20),
        kAudioUnitParameterFlag_ValuesHaveStrings	= (1 << 21),

        kAudioUnitParameterFlag_DisplayLogarithmic      = (1 << 22),

        kAudioUnitParameterFlag_IsHighResolution        = (1 << 23),
        kAudioUnitParameterFlag_NonRealTime             = (1 << 24),
        kAudioUnitParameterFlag_CanRamp                 = (1 << 25),
        kAudioUnitParameterFlag_ExpertMode              = (1 << 26),
        kAudioUnitParameterFlag_HasCFNameString         = (1 << 27),
        kAudioUnitParameterFlag_IsGlobalMeta            = (1 << 28),
        kAudioUnitParameterFlag_IsElementMeta		= (1 << 29),
        kAudioUnitParameterFlag_IsReadable		= (1 << 30),
        kAudioUnitParameterFlag_IsWritable		= (1 << 31)
    };
}

#[repr(C)]
pub struct AudioUnitParameterInfo {
    pub name: [c_char; 52usize],
    pub unitName: CFStringRef,
    pub clumpID: u32,
    pub cfNameString: CFStringRef,
    pub unit: AudioUnitParameterUnit,
    pub minValue: AudioUnitParameterValue,
    pub maxValue: AudioUnitParameterValue,
    pub defaultValue: AudioUnitParameterValue,
    pub flags: u32,
}

impl Default for AudioUnitParameterInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

e! {
    enum {
        kAudioUnitClumpID_System = 0
    };
}

#[inline]
pub fn GetAudioUnitParameterDisplayType(
    flags: AudioUnitParameterOptions,
) -> AudioUnitParameterOptions {
    flags & kAudioUnitParameterFlag_DisplayMask
}

#[inline]
pub fn SetAudioUnitParameterDisplayType(
    flags: AudioUnitParameterOptions,
    displayType: AudioUnitParameterOptions,
) -> AudioUnitParameterOptions {
    (flags & !kAudioUnitParameterFlag_DisplayMask) | displayType
}

e! {
    enum {
        kAudioUnitParameterName_Full = 0xffffffff
    };
}

s! {
    #[derive(Clone, Copy)]
    struct AudioUnitParameterNameInfo {
        pub inID: AudioUnitParameterID,
        pub inDesiredLength: i32,
        pub outName: CFStringRef,
    }

    #[derive(Clone, Copy)]
    struct AudioUnitParameterStringFromValue {
        pub inParamID: AudioUnitParameterID,
        pub inValue: *const AudioUnitParameterValue,
        pub outString: CFStringRef,
    }

    #[derive(Clone, Copy)]
    struct AudioUnitParameterValueFromString {
        pub inParamID: AudioUnitParameterID,
        pub inString: CFStringRef,
        pub outValue: AudioUnitParameterValue,
    }
}

cs! {
    #define kAudioUnitConfigurationInfo_HasCustomView	"HasCustomView"
    #define kAudioUnitConfigurationInfo_ChannelConfigurations	"ChannelConfigurations"
    #define kAudioUnitConfigurationInfo_InitialInputs	"InitialInputs"
    #define kAudioUnitConfigurationInfo_InitialOutputs	"InitialOutputs"
    #define kAudioUnitConfigurationInfo_IconURL			"IconURL"
    #define kAudioUnitConfigurationInfo_BusCountWritable	"BusCountWritable"
    #define kAudioUnitConfigurationInfo_SupportedChannelLayoutTags	"SupportedChannelLayoutTags"
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        // range  (2000 -> 2999)
        kAudioOutputUnitProperty_IsRunning		= 2001
    };
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioUnitProperty_AllParameterMIDIMappings	= 41,
        kAudioUnitProperty_AddParameterMIDIMapping	= 42,
        kAudioUnitProperty_RemoveParameterMIDIMapping   = 43,
        kAudioUnitProperty_HotMapParameterMIDIMapping   = 44
    };
}

e! {
    typedef CF_OPTIONS(u32, AUParameterMIDIMappingFlags) {
        kAUParameterMIDIMapping_AnyChannelFlag		= (1 << 0),
        kAUParameterMIDIMapping_AnyNoteFlag		= (1 << 1),
        kAUParameterMIDIMapping_SubRange		= (1 << 2),
        kAUParameterMIDIMapping_Toggle			= (1 << 3),
        kAUParameterMIDIMapping_Bipolar			= (1 << 4),
        kAUParameterMIDIMapping_Bipolar_On		= (1 << 5)
    };
}

s! {
    #[derive(Clone, Copy)]
    struct Struct_AUParameterMIDIMapping {
        pub mScope: AudioUnitScope,
        pub mElement: AudioUnitElement,
        pub mParameterID: AudioUnitParameterID,
        pub mFlags: u32,
        pub mSubRangeMin: AudioUnitParameterValue,
        pub mSubRangeMax: AudioUnitParameterValue,
        pub mStatus: u8,
        pub mData1: u8,
            reserved1: u8,
            reserved2: u8,
            reserved3: u32,
    }
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        // range  (1000 -> 1999)
        kMusicDeviceProperty_MIDIXMLNames		= 1006,
        kMusicDeviceProperty_PartGroup			= 1010,
        kMusicDeviceProperty_DualSchedulingMode		= 1013,
        kMusicDeviceProperty_SupportsStartStopNote	= 1014
    };
}

e! {
    enum {
        kMusicDeviceSampleFrameMask_SampleOffset        = 0xFFFFFF,
        kMusicDeviceSampleFrameMask_IsScheduled         = 0x01000000
    };
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        // range (3020->3040)
        kAudioUnitOfflineProperty_InputSize		= 3020,
        kAudioUnitOfflineProperty_OutputSize		= 3021,
        kAudioUnitOfflineProperty_StartOffset		= 3022,
        kAudioUnitOfflineProperty_PreflightRequirements	= 3023,
        kAudioUnitOfflineProperty_PreflightName		= 3024
    };
}

e! {
    enum {
        kOfflinePreflight_NotRequired                   = 0,
        kOfflinePreflight_Optional                      = 1,
        kOfflinePreflight_Required                      = 2
    };
}


cfg_if! {
    if #[cfg(feature = "with-deprecated")] {
        e! {
            CF_ENUM(AudioUnitPropertyID) {
                // range (3060->3999)
                kAudioUnitProperty_DistanceAttenuationData      = 3600
            }
        }
    } else {}
}

s! {
    #[derive(Clone, Copy)]
    struct AUDistanceAttenuationDataPair {
        pub inDistance: f32,
        pub outGain: f32,
    }

    #[derive(Clone, Copy)]
    struct AUDistanceAttenuationData {
        pub inNumberOfPairs: u32,
        pub pairs: [AUDistanceAttenuationDataPair; 1usize],
    }
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        // range (4000->4020)
        kAudioUnitMigrateProperty_FromPlugin		= 4000,
        kAudioUnitMigrateProperty_OldAutomation		= 4001
    };
}

e! {
    CF_ENUM(u32) {
        kOtherPluginFormat_Undefined                    = 0, //reserving this value for future use
        kOtherPluginFormat_kMAS                         = 1,
        kOtherPluginFormat_kVST                         = 2,
        kOtherPluginFormat_AU                           = 3
    };
}

s! {
    #[derive(Clone, Copy)]
    struct AudioUnitOtherPluginDesc {
        format: u32,
        plugin: AudioClassDescription,
    }

    #[derive(Clone, Copy)]
    struct AudioUnitParameterValueTranslation {
        pub otherDesc: AudioUnitOtherPluginDesc,
        pub otherParamID: u32,
        pub otherValue: f32,
        pub auParamID: AudioUnitParameterID,
        pub auValue: AudioUnitParameterValue,
    }

    #[derive(Clone, Copy)]
    struct AudioUnitPresetMAS_SettingData {
        pub isStockSetting: u32,
        pub settingID: u32,
        pub dataLen: u32,
        pub data: [u8; 1usize],
    }

    #[derive(Clone, Copy)]
    struct AudioUnitPresetMAS_Settings {
        pub manufacturerID: u32,
        pub effectID: u32,
        pub variantID: u32,
        pub settingsVersion: u32,
        pub numberOfSettings: u32,
        pub settings: [AudioUnitPresetMAS_SettingData; 1usize],
    }
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioUnitProperty_SampleRateConverterComplexity = 3014
    };
}

e! {
    CF_ENUM(u32) {
        kAudioUnitSampleRateConverterComplexity_Linear = 0x6c696e65,
        kAudioUnitSampleRateConverterComplexity_Normal = 0x6e6f726d,
        kAudioUnitSampleRateConverterComplexity_Mastering = 0x62617473,
    };
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioOutputUnitProperty_CurrentDevice		= 2000,
        kAudioOutputUnitProperty_ChannelMap		= 2002,
        kAudioOutputUnitProperty_EnableIO		= 2003,
        kAudioOutputUnitProperty_StartTime		= 2004,
        kAudioOutputUnitProperty_SetInputCallback	= 2005,
        kAudioOutputUnitProperty_HasIO			= 2006,
        kAudioOutputUnitProperty_StartTimestampsAtZero  = 2007
    };
}

s! {
    #[derive(Clone, Copy)]
    struct Struct_AudioOutputUnitStartAtTimeParams {
        pub mTimestamp: AudioTimeStamp,
        pub mFlags: u32,
    }
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAUVoiceIOProperty_BypassVoiceProcessing	= 2100,
        kAUVoiceIOProperty_VoiceProcessingEnableAGC	= 2101,
        kAUVoiceIOProperty_MuteOutput			= 2104
    };
}

cfg_if! {
    if #[cfg(feature = "with-deprecated")] {
        e! {
            CF_ENUM(AudioUnitPropertyID) {
                kAUVoiceIOProperty_VoiceProcessingQuality = 2103
            };
        }
    } else {}
}

e! {
CF_ENUM(AudioUnitPropertyID) {
        kAUNBandEQProperty_NumberOfBands		= 2200,
        kAUNBandEQProperty_MaxNumberOfBands		= 2201,
        kAUNBandEQProperty_BiquadCoefficients		= 2203
};
}

e! {
    CF_ENUM(OSStatus) {
        kAUVoiceIOErr_UnexpectedNumberOfInputChannels   = -66784,
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        // General mixers
        kAudioUnitProperty_MeteringMode			= 3007,

        // Matrix Mixer
        kAudioUnitProperty_MatrixLevels			= 3006,
        kAudioUnitProperty_MatrixDimensions		= 3009,
        kAudioUnitProperty_MeterClipping		= 3011,

        // Multichannel Mixer
        kAudioUnitProperty_InputAnchorTimeStamp		= 3016
    };
}

s! {
    #[derive(Clone, Copy)]
    struct Struct_AudioUnitMeterClipping {
        pub peakValueSinceLastCall: f32,
        pub sawInfinity: Boolean,
        pub sawNotANumber: Boolean,
    }
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioUnitProperty_ReverbRoomType		= 10,
        kAudioUnitProperty_UsesInternalReverb		= 1005,
        kAudioUnitProperty_SpatializationAlgorithm	= 3000,
        kAudioUnitProperty_SpatialMixerDistanceParams	= 3010,
        kAudioUnitProperty_SpatialMixerAttenuationCurve	= 3013,
        kAudioUnitProperty_SpatialMixerRenderingFlags	= 3003,
    };
}

e! {
    typedef CF_ENUM(u32, AUSpatializationAlgorithm) {
        kSpatializationAlgorithm_EqualPowerPanning      = 0,
        kSpatializationAlgorithm_SphericalHead          = 1,
        kSpatializationAlgorithm_HRTF			= 2,
        kSpatializationAlgorithm_SoundField		= 3,
        kSpatializationAlgorithm_VectorBasedPanning	= 4,
        kSpatializationAlgorithm_StereoPassThrough	= 5,
        kSpatializationAlgorithm_HRTFHQ                 = 6
    };
}

e! {
    typedef CF_ENUM(u32, AUReverbRoomType) {
        kReverbRoomType_SmallRoom                       = 0,
        kReverbRoomType_MediumRoom                      = 1,
        kReverbRoomType_LargeRoom                       = 2,
        kReverbRoomType_MediumHall                      = 3,
        kReverbRoomType_LargeHall                       = 4,
        kReverbRoomType_Plate                           = 5,
        kReverbRoomType_MediumChamber                   = 6,
        kReverbRoomType_LargeChamber                    = 7,
        kReverbRoomType_Cathedral                       = 8,
        kReverbRoomType_LargeRoom2                      = 9,
        kReverbRoomType_MediumHall2                     = 10,
        kReverbRoomType_MediumHall3                     = 11,
        kReverbRoomType_LargeHall2                      = 12
    };
}

e! {
    typedef CF_ENUM(u32, AUSpatialMixerAttenuationCurve) {
        kSpatialMixerAttenuationCurve_Power		= 0,
        kSpatialMixerAttenuationCurve_Exponential	= 1,
        kSpatialMixerAttenuationCurve_Inverse		= 2,
        kSpatialMixerAttenuationCurve_Linear		= 3
    };
}

s! {
    #[derive(Clone, Copy)]
    struct Struct_MixerDistanceParams {
        pub mReferenceDistance: f32,
        pub mMaxDistance: f32,
        pub mMaxAttenuation: f32,
    }
}

e! {
    typedef CF_OPTIONS(u32, AUSpatialMixerRenderingFlags) {
        kSpatialMixerRenderingFlags_InterAuralDelay	= (1 << 0),
        kSpatialMixerRenderingFlags_DistanceAttenuation	= (1 << 2),
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioUnitProperty_3DMixerDistanceParams	= 3010,
        kAudioUnitProperty_3DMixerAttenuationCurve	= 3013,
        kAudioUnitProperty_DopplerShift			= 3002,
        kAudioUnitProperty_3DMixerRenderingFlags	= 3003,
        kAudioUnitProperty_3DMixerDistanceAtten		= 3004,
        kAudioUnitProperty_ReverbPreset			= 3012
    };
}

e! {
    typedef CF_OPTIONS(u32, AU3DMixerRenderingFlags) {
        k3DMixerRenderingFlags_InterAuralDelay		= (1 << 0),
        k3DMixerRenderingFlags_DopplerShift		= (1 << 1),
        k3DMixerRenderingFlags_DistanceAttenuation	= (1 << 2),
        k3DMixerRenderingFlags_DistanceFilter		= (1 << 3),
        k3DMixerRenderingFlags_DistanceDiffusion	= (1 << 4),
        k3DMixerRenderingFlags_LinearDistanceAttenuation = (1 << 5),
        k3DMixerRenderingFlags_ConstantReverbBlend	= (1 << 6)
    };
}

e!{
    typedef CF_ENUM(u32, AU3DMixerAttenuationCurve) {
        k3DMixerAttenuationCurve_Power			= 0,
        k3DMixerAttenuationCurve_Exponential		= 1,
        k3DMixerAttenuationCurve_Inverse		= 2,
        k3DMixerAttenuationCurve_Linear			= 3
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioUnitProperty_ScheduleAudioSlice		= 3300,
        kAudioUnitProperty_ScheduleStartTimeStamp	= 3301,
        kAudioUnitProperty_CurrentPlayTime		= 3302
    };
}

e! {
    typedef CF_OPTIONS(u32, AUScheduledAudioSliceFlags) {
        kScheduledAudioSliceFlag_Complete               = 0x01,
        kScheduledAudioSliceFlag_BeganToRender          = 0x02,
        kScheduledAudioSliceFlag_BeganToRenderLate      = 0x04,
    };
}

pub type ScheduledAudioSliceCompletionProc =
    Option<extern "C" fn(userData: *mut c_void, bufferList: *mut ScheduledAudioSlice)>;
s! {
    #[derive(Clone, Copy)]
    struct ScheduledAudioSlice {
        pub mTimeStamp: AudioTimeStamp,
        pub mCompletionProc: ScheduledAudioSliceCompletionProc,
        pub mCompletionProcUserData: *mut c_void,
        pub mFlags: u32,
        pub mReserved: u32,
        pub mReserved2: *mut c_void,
        pub mNumberFrames: u32,
        pub mBufferList: *mut AudioBufferList,
    }
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioUnitProperty_ScheduledFileIDs		= 3310,
        kAudioUnitProperty_ScheduledFileRegion		= 3311,
        kAudioUnitProperty_ScheduledFilePrime		= 3312,
        kAudioUnitProperty_ScheduledFileBufferSizeFrames = 3313,
        kAudioUnitProperty_ScheduledFileNumberBuffers   = 3314
    };
}

pub type ScheduledAudioFileRegionCompletionProc =
    Option<
        extern "C" fn(userData: *mut c_void,
                      fileRegion: *mut ScheduledAudioFileRegion,
                      result: OSStatus),
    >;

pub enum OpaqueAudioFileID { }

s! {
    #[derive(Clone, Copy)]
    struct ScheduledAudioFileRegion {
        pub mTimeStamp: AudioTimeStamp,
        pub mCompletionProc: ScheduledAudioFileRegionCompletionProc,
        pub mCompletionProcUserData: *mut c_void,
        pub mAudioFile: *mut OpaqueAudioFileID,
        pub mLoopCount: u32,
        pub mStartFrame: i64,
        pub mFramesToPlay: u32,
    }
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        kMusicDeviceProperty_UsesInternalReverb		= kAudioUnitProperty_UsesInternalReverb,
        kMusicDeviceProperty_SoundBankData		= 1008,
        kMusicDeviceProperty_StreamFromDisk		= 1011,
        kMusicDeviceProperty_SoundBankFSRef		= 1012
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kMusicDeviceProperty_InstrumentName		= 1001,
        kMusicDeviceProperty_InstrumentNumber           = 1004
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kMusicDeviceProperty_InstrumentCount            = 1000,
        kMusicDeviceProperty_BankName			= 1007,
        kMusicDeviceProperty_SoundBankURL		= 1100
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAUMIDISynthProperty_EnablePreload		= 4119
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        // range (4100->4999)
        kAUSamplerProperty_LoadInstrument		= 4102,
        kAUSamplerProperty_LoadAudioFiles		= 4101
    };
}

s! {
    #[derive(Clone, Copy)]
    struct AUSamplerInstrumentData {
        pub fileURL: CFURLRef,
        pub instrumentType: u8,
        pub bankMSB: u8,
        pub bankLSB: u8,
        pub presetID: u8,
    }
}

e! {
    enum
    {
        kInstrumentType_DLSPreset                       = 1,
        kInstrumentType_SF2Preset                       = kInstrumentType_DLSPreset,
        kInstrumentType_AUPreset                        = 2,
        kInstrumentType_Audiofile                       = 3,
        kInstrumentType_EXS24                           = 4
    };
}

e! {
    enum
    {
        kAUSampler_DefaultPercussionBankMSB             = 0x78,
        kAUSampler_DefaultMelodicBankMSB                = 0x79,
        kAUSampler_DefaultBankLSB			= 0x00
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioUnitProperty_DeferredRendererPullSize	= 3320,
        kAudioUnitProperty_DeferredRendererExtraLatency	= 3321,
        kAudioUnitProperty_DeferredRendererWaitFrames   = 3322
    };
}



e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAUNetReceiveProperty_Hostname                  = 3511,
        kAUNetReceiveProperty_Password                  = 3512
    };
}


e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAUNetSendProperty_PortNum                      = 3513,
        kAUNetSendProperty_TransmissionFormat           = 3514,
        kAUNetSendProperty_TransmissionFormatIndex      = 3515,
        kAUNetSendProperty_ServiceName                  = 3516,
        kAUNetSendProperty_Disconnect                   = 3517,
        kAUNetSendProperty_Password                     = 3518
    };
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAUNetSendPresetFormat_PCMFloat32		= 0,
        kAUNetSendPresetFormat_PCMInt24			= 1,
        kAUNetSendPresetFormat_PCMInt16			= 2,
        kAUNetSendPresetFormat_Lossless24		= 3,
        kAUNetSendPresetFormat_Lossless16		= 4,
        kAUNetSendPresetFormat_ULaw			= 5,
        kAUNetSendPresetFormat_IMA4			= 6,
        kAUNetSendPresetFormat_AAC_128kbpspc		= 7,
        kAUNetSendPresetFormat_AAC_96kbpspc		= 8,
        kAUNetSendPresetFormat_AAC_80kbpspc		= 9,
        kAUNetSendPresetFormat_AAC_64kbpspc		= 10,
        kAUNetSendPresetFormat_AAC_48kbpspc		= 11,
        kAUNetSendPresetFormat_AAC_40kbpspc		= 12,
        kAUNetSendPresetFormat_AAC_32kbpspc		= 13,
        kAUNetSendPresetFormat_AAC_LD_64kbpspc		= 14,
        kAUNetSendPresetFormat_AAC_LD_48kbpspc		= 15,
        kAUNetSendPresetFormat_AAC_LD_40kbpspc		= 16,
        kAUNetSendPresetFormat_AAC_LD_32kbpspc		= 17,
        kAUNetSendNumPresetFormats			= 18
    };
}


// GENERIC
e! {
    enum {
        kAudioUnitParameterFlag_Global			= (1 << 0),	//	parameter scope is global
        kAudioUnitParameterFlag_Input			= (1 << 1),	//	parameter scope is input
        kAudioUnitParameterFlag_Output			= (1 << 2),	//	parameter scope is output
        kAudioUnitParameterFlag_Group			= (1 << 3)	//	parameter scope is group
    };
}

e! {
    enum {
        kAudioUnitParameterFlag_HasName			= kAudioUnitParameterFlag_ValuesHaveStrings
    };
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        //kAudioUnitProperty_SetInputCallback		= 7 -> deprecated
        kAudioUnitProperty_SRCAlgorithm			= 9,
        kAudioUnitProperty_MIDIControlMapping		= 17,
        kAudioUnitProperty_CurrentPreset		= 28,

        kAudioUnitProperty_ParameterValueName		= kAudioUnitProperty_ParameterStringFromValue,
        kAudioUnitProperty_BusCount			= kAudioUnitProperty_ElementCount,

        kAudioOfflineUnitProperty_InputSize		= kAudioUnitOfflineProperty_InputSize,
        kAudioOfflineUnitProperty_OutputSize		= kAudioUnitOfflineProperty_OutputSize
    };
}

e! {
    CF_ENUM(u32) {
        kAudioUnitSRCAlgorithm_Polyphase                = 0x706f6c79,
        kAudioUnitSRCAlgorithm_MediumQuality            = 0x63737263
    };
}

// Deprecated. See AudioUnitParameterStringFromValue for equivalent
// structure, but with clearer field names
s! {
    #[derive(Clone,Copy)]
    struct AudioUnitParameterValueName {
        pub inParamID: AudioUnitParameterID,
        pub inValue: *const f32,
        pub outName: CFStringRef,
    }
}

// Deprecated. These properties are Apple specific.

e! {
    CF_ENUM(AudioUnitPropertyID) {
        kMusicDeviceProperty_GroupOutputBus		= 1002,
        kMusicDeviceProperty_SoundBankFSSpec		= 1003,
        kAudioUnitProperty_PannerMode			= 3008
    };
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAudioUnitProperty_SpeakerConfiguration		= 3001
    };
}

// Deprecated in favor of the newer AudioChannelLayout
// structure and its supporting property.
e! {
    enum {
        kSpeakerConfiguration_HeadPhones		= 0,
        kSpeakerConfiguration_Stereo			= 1,
        kSpeakerConfiguration_Quad			= 2,
        kSpeakerConfiguration_5_0			= 3,
        kSpeakerConfiguration_5_1			= kSpeakerConfiguration_5_0
    };
}


// Deprecated in favor of the newer AUSamplerInstrumentData
// structure and its supporting property.

s! {
    #[derive(Clone, Copy)]
    struct AUSamplerBankPresetData {
        pub bankURL: CFURLRef,
        pub bankMSB: u8,
        pub bankLSB: u8,
        pub presetID: u8,
        pub reserved: u8,
    }
}

e! {
    CF_ENUM(AudioUnitPropertyID) {
        kAUSamplerProperty_LoadPresetFromBank		= 4100,
        kAUSamplerProperty_BankAndPreset		= kAUSamplerProperty_LoadPresetFromBank
    };
}
