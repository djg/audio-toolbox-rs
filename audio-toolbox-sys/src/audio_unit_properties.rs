use {Boolean, CFStringRef, CFURLRef, OSStatus};

use au_component::{AUInputSamplesInOutputCallback, AURenderCallback, AudioUnit,
                   AudioUnitElement, AudioUnitParameterID, AudioUnitParameterValue,
                   AudioUnitPropertyID, AudioUnitScope};
use core_audio_sys::{AudioBufferList, AudioClassDescription, AudioTimeStamp};

use std::fmt::{self, Debug};
use std::mem;
use std::os::raw::{c_char, c_double, c_float, c_void};

pub const kAudioUnitScope_Global: AudioUnitScope = 0;
pub const kAudioUnitScope_Input: AudioUnitScope = 1;
pub const kAudioUnitScope_Output: AudioUnitScope = 2;
pub const kAudioUnitScope_Group: AudioUnitScope = 3;
pub const kAudioUnitScope_Part: AudioUnitScope = 4;
pub const kAudioUnitScope_Note: AudioUnitScope = 5;
pub const kAudioUnitScope_Layer: AudioUnitScope = 6;
pub const kAudioUnitScope_LayerItem: AudioUnitScope = 7;

// range (0 -> 999)
pub const kAudioUnitProperty_ClassInfo: AudioUnitPropertyID = 0;
pub const kAudioUnitProperty_MakeConnection: AudioUnitPropertyID = 1;
pub const kAudioUnitProperty_SampleRate: AudioUnitPropertyID = 2;
pub const kAudioUnitProperty_ParameterList: AudioUnitPropertyID = 3;
pub const kAudioUnitProperty_ParameterInfo: AudioUnitPropertyID = 4;
pub const kAudioUnitProperty_CPULoad: AudioUnitPropertyID = 6;
pub const kAudioUnitProperty_StreamFormat: AudioUnitPropertyID = 8;
pub const kAudioUnitProperty_ElementCount: AudioUnitPropertyID = 11;
pub const kAudioUnitProperty_Latency: AudioUnitPropertyID = 12;
pub const kAudioUnitProperty_SupportedNumChannels: AudioUnitPropertyID = 13;
pub const kAudioUnitProperty_MaximumFramesPerSlice: AudioUnitPropertyID = 14;
pub const kAudioUnitProperty_ParameterValueStrings: AudioUnitPropertyID = 16;
pub const kAudioUnitProperty_AudioChannelLayout: AudioUnitPropertyID = 19;
pub const kAudioUnitProperty_TailTime: AudioUnitPropertyID = 20;
pub const kAudioUnitProperty_BypassEffect: AudioUnitPropertyID = 21;
pub const kAudioUnitProperty_LastRenderError: AudioUnitPropertyID = 22;
pub const kAudioUnitProperty_SetRenderCallback: AudioUnitPropertyID = 23;
pub const kAudioUnitProperty_FactoryPresets: AudioUnitPropertyID = 24;
pub const kAudioUnitProperty_RenderQuality: AudioUnitPropertyID = 26;
pub const kAudioUnitProperty_HostCallbacks: AudioUnitPropertyID = 27;
pub const kAudioUnitProperty_InPlaceProcessing: AudioUnitPropertyID = 29;
pub const kAudioUnitProperty_ElementName: AudioUnitPropertyID = 30;
pub const kAudioUnitProperty_SupportedChannelLayoutTags: AudioUnitPropertyID = 32;
pub const kAudioUnitProperty_PresentPreset: AudioUnitPropertyID = 36;
pub const kAudioUnitProperty_DependentParameters: AudioUnitPropertyID = 45;
pub const kAudioUnitProperty_InputSamplesInOutput: AudioUnitPropertyID = 49;
pub const kAudioUnitProperty_ShouldAllocateBuffer: AudioUnitPropertyID = 51;
pub const kAudioUnitProperty_FrequencyResponse: AudioUnitPropertyID = 52;
pub const kAudioUnitProperty_ParameterHistoryInfo: AudioUnitPropertyID = 53;
pub const kAudioUnitProperty_NickName: AudioUnitPropertyID = 54;
pub const kAudioUnitProperty_OfflineRender: AudioUnitPropertyID = 37;
pub const kAudioUnitProperty_ParameterIDName: AudioUnitPropertyID = 34;
pub const kAudioUnitProperty_ParameterStringFromValue: AudioUnitPropertyID = 33;
pub const kAudioUnitProperty_ParameterClumpName: AudioUnitPropertyID = 35;
pub const kAudioUnitProperty_ParameterValueFromString: AudioUnitPropertyID = 38;
pub const kAudioUnitProperty_ContextName: AudioUnitPropertyID = 25;
pub const kAudioUnitProperty_PresentationLatency: AudioUnitPropertyID = 40;
pub const kAudioUnitProperty_ClassInfoFromDocument: AudioUnitPropertyID = 50;
pub const kAudioUnitProperty_RequestViewController: AudioUnitPropertyID = 56;
pub const kAudioUnitProperty_ParametersForOverview: AudioUnitPropertyID = 57;
pub const kAudioUnitProperty_SupportsMPE: AudioUnitPropertyID = 58;

pub const kAudioUnitProperty_FastDispatch: AudioUnitPropertyID = 5;
pub const kAudioUnitProperty_SetExternalBuffer: AudioUnitPropertyID = 15;
pub const kAudioUnitProperty_GetUIComponentList: AudioUnitPropertyID = 18;
pub const kAudioUnitProperty_CocoaUI: AudioUnitPropertyID = 31;
pub const kAudioUnitProperty_IconLocation: AudioUnitPropertyID = 39;
pub const kAudioUnitProperty_AUHostIdentifier: AudioUnitPropertyID = 46;

pub const kAudioUnitProperty_MIDIOutputCallbackInfo: AudioUnitPropertyID = 47;
pub const kAudioUnitProperty_MIDIOutputCallback: AudioUnitPropertyID = 48;

pub const kAUPresetVersionKey: &'static str = "version";
pub const kAUPresetTypeKey: &'static str = "type";
pub const kAUPresetSubtypeKey: &'static str = "subtype";
pub const kAUPresetManufacturerKey: &'static str = "manufacturer";
pub const kAUPresetDataKey: &'static str = "data";
pub const kAUPresetNameKey: &'static str = "name";
pub const kAUPresetRenderQualityKey: &'static str = "render-quality";
pub const kAUPresetCPULoadKey: &'static str = "cpu-load";
pub const kAUPresetElementNameKey: &'static str = "element-name";
pub const kAUPresetExternalFileRefs: &'static str = "file-references";

pub const kAUPresetVSTDataKey: &'static str = "vstdata";
pub const kAUPresetVSTPresetKey: &'static str = "vstpreset";

pub const kAUPresetMASDataKey: &'static str = "masdata";

pub const kAUPresetPartKey: &'static str = "part";

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitConnection {
    pub sourceAudioUnit: AudioUnit,
    pub sourceOutputNumber: u32,
    pub destInputNumber: u32,
}

impl Default for AudioUnitConnection {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUChannelInfo {
    pub inChannels: i16,
    pub outChannels: i16,
}

impl Default for AUChannelInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitExternalBuffer {
    pub buffer: *mut u8,
    pub size: u32,
}

impl Default for AudioUnitExternalBuffer {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AURenderCallbackStruct {
    pub inputProc: Option<AURenderCallback>,
    pub inputProcRefCon: *mut c_void,
}

impl Default for AURenderCallbackStruct {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Struct_AUPreset {
    pub presetNumber: i32,
    pub presetName: CFStringRef,
}

impl Default for Struct_AUPreset {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kRenderQuality_Max: u32 = 127;
pub const kRenderQuality_High: u32 = 96;
pub const kRenderQuality_Medium: u32 = 64;
pub const kRenderQuality_Low: u32 = 32;
pub const kRenderQuality_Min: u32 = 0;

pub const kNumberOfResponseFrequencies: u32 = 1024;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioUnitFrequencyResponseBin {
    pub mFrequency: c_double,
    pub mMagnitude: c_double,
}

impl Default for AudioUnitFrequencyResponseBin {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type HostCallback_GetBeatAndTempo = Option<
    extern fn(
        inHostUserData: *mut c_void,
        outCurrentBeat: *mut c_double,
        outCurrentTempo: *mut c_double,
    ) -> OSStatus,
>;
pub type HostCallback_GetMusicalTimeLocation = Option<
    extern fn(
        inHostUserData: *mut c_void,
        outDeltaSampleOffsetToNextBeat: *mut u32,
        outTimeSig_Numerator: *mut c_float,
        outTimeSig_Denominator: *mut u32,
        outCurrentMeasureDownBeat: *mut c_double,
    ) -> OSStatus,
>;
pub type HostCallback_GetTransportState = Option<
    extern fn(
        inHostUserData: *mut c_void,
        outIsPlaying: *mut Boolean,
        outTransportStateChanged: *mut Boolean,
        outCurrentSampleInTimeLine: *mut c_double,
        outIsCycling: *mut Boolean,
        outCycleStartBeat: *mut c_double,
        outCycleEndBeat: *mut c_double,
    ) -> OSStatus,
>;
pub type HostCallback_GetTransportState2 = Option<
    extern fn(
        inHostUserData: *mut c_void,
        outIsPlaying: *mut Boolean,
        outIsRecording: *mut Boolean,
        outTransportStateChanged: *mut Boolean,
        outCurrentSampleInTimeLine: *mut c_double,
        outIsCycling: *mut Boolean,
        outCycleStartBeat: *mut c_double,
        outCycleEndBeat: *mut c_double,
    ) -> OSStatus,
>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HostCallbackInfo {
    pub hostUserData: *mut c_void,
    pub beatAndTempoProc: HostCallback_GetBeatAndTempo,
    pub musicalTimeLocationProc: HostCallback_GetMusicalTimeLocation,
    pub transportStateProc: HostCallback_GetTransportState,
    pub transportStateProc2: HostCallback_GetTransportState2,
}

impl Default for HostCallbackInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDependentParameter {
    pub mScope: AudioUnitScope,
    pub mParameterID: AudioUnitParameterID,
}

impl Default for AUDependentParameter {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioUnitCocoaViewInfo {
    pub mCocoaAUViewBundleLocation: CFURLRef,
    pub mCocoaAUViewClass: [CFStringRef; 1usize],
}

impl Default for AudioUnitCocoaViewInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUHostVersionIdentifier {
    pub hostName: CFStringRef,
    pub hostVersion: u32,
}

impl Default for AUHostVersionIdentifier {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub enum MIDIPacketList {}

pub type AUMIDIOutputCallback = extern fn(
    userData: *mut c_void,
    timeStamp: *const AudioTimeStamp,
    midiOutNum: u32,
    pktlist: *const MIDIPacketList,
) -> OSStatus;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUMIDIOutputCallbackStruct {
    pub midiOutputCallback: Option<AUMIDIOutputCallback>,
    pub userData: *mut c_void,
}

impl Default for AUMIDIOutputCallbackStruct {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUInputSamplesInOutputCallbackStruct {
    pub inputToOutputCallback: Option<AUInputSamplesInOutputCallback>,
    pub userData: *mut c_void,
}

impl Default for AUInputSamplesInOutputCallbackStruct {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterHistoryInfo {
    pub updatesPerSecond: c_float,
    pub historyDurationInSeconds: c_float,
}

impl Default for AudioUnitParameterHistoryInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type AudioUnitParameterUnit = u32;
pub const kAudioUnitParameterUnit_Generic: u32 = 0;
pub const kAudioUnitParameterUnit_Indexed: u32 = 1;
pub const kAudioUnitParameterUnit_Boolean: u32 = 2;
pub const kAudioUnitParameterUnit_Percent: u32 = 3;
pub const kAudioUnitParameterUnit_Seconds: u32 = 4;
pub const kAudioUnitParameterUnit_SampleFrames: u32 = 5;
pub const kAudioUnitParameterUnit_Phase: u32 = 6;
pub const kAudioUnitParameterUnit_Rate: u32 = 7;
pub const kAudioUnitParameterUnit_Hertz: u32 = 8;
pub const kAudioUnitParameterUnit_Cents: u32 = 9;
pub const kAudioUnitParameterUnit_RelativeSemiTones: u32 = 10;
pub const kAudioUnitParameterUnit_MIDINoteNumber: u32 = 11;
pub const kAudioUnitParameterUnit_MIDIController: u32 = 12;
pub const kAudioUnitParameterUnit_Decibels: u32 = 13;
pub const kAudioUnitParameterUnit_LinearGain: u32 = 14;
pub const kAudioUnitParameterUnit_Degrees: u32 = 15;
pub const kAudioUnitParameterUnit_EqualPowerCrossfade: u32 = 16;
pub const kAudioUnitParameterUnit_MixerFaderCurve1: u32 = 17;
pub const kAudioUnitParameterUnit_Pan: u32 = 18;
pub const kAudioUnitParameterUnit_Meters: u32 = 19;
pub const kAudioUnitParameterUnit_AbsoluteCents: u32 = 20;
pub const kAudioUnitParameterUnit_Octaves: u32 = 21;
pub const kAudioUnitParameterUnit_BPM: u32 = 22;
pub const kAudioUnitParameterUnit_Beats: u32 = 23;
pub const kAudioUnitParameterUnit_Milliseconds: u32 = 24;
pub const kAudioUnitParameterUnit_Ratio: u32 = 25;
pub const kAudioUnitParameterUnit_CustomUnit: u32 = 26;

pub type AudioUnitParameterOptions = u32;
pub const kAudioUnitParameterFlag_CFNameRelease: u32 = (1 << 4);
pub const kAudioUnitParameterFlag_OmitFromPresets: u32 = (1 << 13);
pub const kAudioUnitParameterFlag_PlotHistory: u32 = (1 << 14);
pub const kAudioUnitParameterFlag_MeterReadOnly: u32 = (1 << 15);
pub const kAudioUnitParameterFlag_DisplayMask: u32 = (7 << 16) | (1 << 22);
pub const kAudioUnitParameterFlag_DisplaySquareRoot: u32 = (1 << 16);
pub const kAudioUnitParameterFlag_DisplaySquared: u32 = (2 << 16);
pub const kAudioUnitParameterFlag_DisplayCubed: u32 = (3 << 16);
pub const kAudioUnitParameterFlag_DisplayCubeRoot: u32 = (4 << 16);
pub const kAudioUnitParameterFlag_DisplayExponential: u32 = (5 << 16);
pub const kAudioUnitParameterFlag_HasClump: u32 = (1 << 20);
pub const kAudioUnitParameterFlag_ValuesHaveStrings: u32 = (1 << 21);
pub const kAudioUnitParameterFlag_DisplayLogarithmic: u32 = (1 << 22);
pub const kAudioUnitParameterFlag_IsHighResolution: u32 = (1 << 23);
pub const kAudioUnitParameterFlag_NonRealTime: u32 = (1 << 24);
pub const kAudioUnitParameterFlag_CanRamp: u32 = (1 << 25);
pub const kAudioUnitParameterFlag_ExpertMode: u32 = (1 << 26);
pub const kAudioUnitParameterFlag_HasCFNameString: u32 = (1 << 27);
pub const kAudioUnitParameterFlag_IsGlobalMeta: u32 = (1 << 28);
pub const kAudioUnitParameterFlag_IsElementMeta: u32 = (1 << 29);
pub const kAudioUnitParameterFlag_IsReadable: u32 = (1 << 30);
pub const kAudioUnitParameterFlag_IsWritable: u32 = (1 << 31);

#[repr(C)]
#[derive(Clone, Copy)]
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

impl Debug for AudioUnitParameterInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioUnitParameterInfo")
            .field("name", &&self.name[..])
            .field("unitName", &self.unitName)
            .field("clumpID", &self.clumpID)
            .field("cfNameString", &self.cfNameString)
            .field("unit", &self.unit)
            .field("minValue", &self.minValue)
            .field("maxValue", &self.maxValue)
            .field("defaultValue", &self.defaultValue)
            .field("flags", &self.flags)
            .finish()
    }
}

impl Default for AudioUnitParameterInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kAudioUnitClumpID_System: u32 = 0;
pub const kAudioUnitParameterName_Full: u32 = 0xffffffff;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterNameInfo {
    pub inID: AudioUnitParameterID,
    pub inDesiredLength: i32,
    pub outName: CFStringRef,
}

impl Default for AudioUnitParameterNameInfo {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterStringFromValue {
    pub inParamID: AudioUnitParameterID,
    pub inValue: *const AudioUnitParameterValue,
    pub outString: CFStringRef,
}

impl Default for AudioUnitParameterStringFromValue {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterValueFromString {
    pub inParamID: AudioUnitParameterID,
    pub inString: CFStringRef,
    pub outValue: AudioUnitParameterValue,
}

impl Default for AudioUnitParameterValueFromString {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kAudioUnitConfigurationInfo_HasCustomView: &'static str = "HasCustomView";
pub const kAudioUnitConfigurationInfo_ChannelConfigurations: &'static str =
    "ChannelConfigurations";
pub const kAudioUnitConfigurationInfo_InitialInputs: &'static str = "InitialInputs";
pub const kAudioUnitConfigurationInfo_InitialOutputs: &'static str = "InitialOutputs";
pub const kAudioUnitConfigurationInfo_IconURL: &'static str = "IconURL";
pub const kAudioUnitConfigurationInfo_BusCountWritable: &'static str = "BusCountWritable";
pub const kAudioUnitConfigurationInfo_SupportedChannelLayoutTags: &'static str =
    "SupportedChannelLayoutTags";

// range  (2000 -> 2999)
pub const kAudioOutputUnitProperty_CurrentDevice: AudioUnitPropertyID = 2000;
pub const kAudioOutputUnitProperty_IsRunning: AudioUnitPropertyID = 2001;
pub const kAudioOutputUnitProperty_ChannelMap: AudioUnitPropertyID = 2002;
pub const kAudioOutputUnitProperty_EnableIO: AudioUnitPropertyID = 2003;
pub const kAudioOutputUnitProperty_StartTime: AudioUnitPropertyID = 2004;
pub const kAudioOutputUnitProperty_SetInputCallback: AudioUnitPropertyID = 2005;
pub const kAudioOutputUnitProperty_HasIO: AudioUnitPropertyID = 2006;
pub const kAudioOutputUnitProperty_StartTimestampsAtZero: AudioUnitPropertyID = 2007;

pub const kAudioUnitProperty_AllParameterMIDIMappings: AudioUnitPropertyID = 41;
pub const kAudioUnitProperty_AddParameterMIDIMapping: AudioUnitPropertyID = 42;
pub const kAudioUnitProperty_RemoveParameterMIDIMapping: AudioUnitPropertyID = 43;
pub const kAudioUnitProperty_HotMapParameterMIDIMapping: AudioUnitPropertyID = 44;

pub type AUParameterMIDIMappingFlags = u32;
pub const kAUParameterMIDIMapping_AnyChannelFlag: u32 = (1 << 0);
pub const kAUParameterMIDIMapping_AnyNoteFlag: u32 = (1 << 1);
pub const kAUParameterMIDIMapping_SubRange: u32 = (1 << 2);
pub const kAUParameterMIDIMapping_Toggle: u32 = (1 << 3);
pub const kAUParameterMIDIMapping_Bipolar: u32 = (1 << 4);
pub const kAUParameterMIDIMapping_Bipolar_On: u32 = (1 << 5);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUParameterMIDIMapping {
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

impl Default for AUParameterMIDIMapping {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

// range  (1000 -> 1999)
pub const kMusicDeviceProperty_MIDIXMLNames: AudioUnitPropertyID = 1006;
pub const kMusicDeviceProperty_PartGroup: AudioUnitPropertyID = 1010;
pub const kMusicDeviceProperty_DualSchedulingMode: AudioUnitPropertyID = 1013;
pub const kMusicDeviceProperty_SupportsStartStopNote: AudioUnitPropertyID = 1014;

pub const kMusicDeviceSampleFrameMask_SampleOffset: u32 = 0xFFFFFF;
pub const kMusicDeviceSampleFrameMask_IsScheduled: u32 = 0x01000000;

// range (3020->3040)
pub const kAudioUnitOfflineProperty_InputSize: AudioUnitPropertyID = 3020;
pub const kAudioUnitOfflineProperty_OutputSize: AudioUnitPropertyID = 3021;
pub const kAudioUnitOfflineProperty_StartOffset: AudioUnitPropertyID = 3022;
pub const kAudioUnitOfflineProperty_PreflightRequirements: AudioUnitPropertyID = 3023;
pub const kAudioUnitOfflineProperty_PreflightName: AudioUnitPropertyID = 3024;

pub const kOfflinePreflight_NotRequired: u32 = 0;
pub const kOfflinePreflight_Optional: u32 = 1;
pub const kOfflinePreflight_Required: u32 = 2;

// range (3060->3999)
#[cfg(feature = "deprecated")]
pub const kAudioUnitProperty_DistanceAttenuationData: AudioUnitPropertyID = 3600;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUDistanceAttenuationDataPair {
    pub inDistance: c_float,
    pub outGain: c_float,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUDistanceAttenuationData {
    pub inNumberOfPairs: u32,
    pub pairs: [AUDistanceAttenuationDataPair; 1],
}

impl Default for AUDistanceAttenuationDataPair {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

// range (4000->4020)
pub const kAudioUnitMigrateProperty_FromPlugin: AudioUnitPropertyID = 4000;
pub const kAudioUnitMigrateProperty_OldAutomation: AudioUnitPropertyID = 4001;

pub const kOtherPluginFormat_Undefined: u32 = 0;
pub const kOtherPluginFormat_kMAS: u32 = 1;
pub const kOtherPluginFormat_kVST: u32 = 2;
pub const kOtherPluginFormat_AU: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitOtherPluginDesc {
    format: u32,
    plugin: AudioClassDescription,
}

impl Default for AudioUnitOtherPluginDesc {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterValueTranslation {
    pub otherDesc: AudioUnitOtherPluginDesc,
    pub otherParamID: u32,
    pub otherValue: c_float,
    pub auParamID: AudioUnitParameterID,
    pub auValue: AudioUnitParameterValue,
}

impl Default for AudioUnitParameterValueTranslation {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitPresetMAS_SettingData {
    pub isStockSetting: u32,
    pub settingID: u32,
    pub dataLen: u32,
    pub data: [u8; 1usize],
}

impl Default for AudioUnitPresetMAS_SettingData {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitPresetMAS_Settings {
    pub manufacturerID: u32,
    pub effectID: u32,
    pub variantID: u32,
    pub settingsVersion: u32,
    pub numberOfSettings: u32,
    pub settings: [AudioUnitPresetMAS_SettingData; 1usize],
}

impl Default for AudioUnitPresetMAS_Settings {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kAudioUnitProperty_SampleRateConverterComplexity: AudioUnitPropertyID = 3014;

pub const kAudioUnitSampleRateConverterComplexity_Linear: u32 = 1818848869;
pub const kAudioUnitSampleRateConverterComplexity_Normal: u32 = 1852797549;
pub const kAudioUnitSampleRateConverterComplexity_Mastering: u32 = 1650553971;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioOutputUnitStartAtTimeParams {
    pub mTimestamp: AudioTimeStamp,
    pub mFlags: u32,
}

impl Default for AudioOutputUnitStartAtTimeParams {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kAUVoiceIOProperty_BypassVoiceProcessing: AudioUnitPropertyID = 2100;
pub const kAUVoiceIOProperty_VoiceProcessingEnableAGC: AudioUnitPropertyID = 2101;
pub const kAUVoiceIOProperty_MuteOutput: AudioUnitPropertyID = 2104;

#[cfg(feature = "deprecated")]
pub const kAUVoiceIOProperty_VoiceProcessingQuality: AudioUnitPropertyID = 2103;

pub const kAUNBandEQProperty_NumberOfBands: AudioUnitPropertyID = 2200;
pub const kAUNBandEQProperty_MaxNumberOfBands: AudioUnitPropertyID = 2201;
pub const kAUNBandEQProperty_BiquadCoefficients: AudioUnitPropertyID = 2203;

pub const kAUVoiceIOErr_UnexpectedNumberOfInputChannels: OSStatus = -66784;

// General mixers
pub const kAudioUnitProperty_MeteringMode: AudioUnitPropertyID = 3007;

// Matrix Mixer
pub const kAudioUnitProperty_MatrixLevels: AudioUnitPropertyID = 3006;
pub const kAudioUnitProperty_MatrixDimensions: AudioUnitPropertyID = 3009;
pub const kAudioUnitProperty_MeterClipping: AudioUnitPropertyID = 3011;

// Multichannel Mixer
pub const kAudioUnitProperty_InputAnchorTimeStamp: AudioUnitPropertyID = 3016;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct AudioUnitMeterClipping {
    pub peakValueSinceLastCall: c_float,
    pub sawInfinity: Boolean,
    pub sawNotANumber: Boolean,
}

impl Default for AudioUnitMeterClipping {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kAudioUnitProperty_ReverbRoomType: AudioUnitPropertyID = 10;
pub const kAudioUnitProperty_UsesInternalReverb: AudioUnitPropertyID = 1005;
pub const kAudioUnitProperty_SpatializationAlgorithm: AudioUnitPropertyID = 3000;
pub const kAudioUnitProperty_SpatialMixerDistanceParams: AudioUnitPropertyID = 3010;
pub const kAudioUnitProperty_SpatialMixerAttenuationCurve: AudioUnitPropertyID = 3013;
pub const kAudioUnitProperty_SpatialMixerRenderingFlags: AudioUnitPropertyID = 3003;

pub type AUSpatializationAlgorithm = u32;
pub const kSpatializationAlgorithm_EqualPowerPanning: u32 = 0;
pub const kSpatializationAlgorithm_SphericalHead: u32 = 1;
pub const kSpatializationAlgorithm_HRTF: u32 = 2;
pub const kSpatializationAlgorithm_SoundField: u32 = 3;
pub const kSpatializationAlgorithm_VectorBasedPanning: u32 = 4;
pub const kSpatializationAlgorithm_StereoPassThrough: u32 = 5;
pub const kSpatializationAlgorithm_HRTFHQ: u32 = 6;

pub type AUReverbRoomType = u32;
pub const kReverbRoomType_SmallRoom: u32 = 0;
pub const kReverbRoomType_MediumRoom: u32 = 1;
pub const kReverbRoomType_LargeRoom: u32 = 2;
pub const kReverbRoomType_MediumHall: u32 = 3;
pub const kReverbRoomType_LargeHall: u32 = 4;
pub const kReverbRoomType_Plate: u32 = 5;
pub const kReverbRoomType_MediumChamber: u32 = 6;
pub const kReverbRoomType_LargeChamber: u32 = 7;
pub const kReverbRoomType_Cathedral: u32 = 8;
pub const kReverbRoomType_LargeRoom2: u32 = 9;
pub const kReverbRoomType_MediumHall2: u32 = 10;
pub const kReverbRoomType_MediumHall3: u32 = 11;
pub const kReverbRoomType_LargeHall2: u32 = 12;

pub type AUSpatialMixerAttenuationCurve = u32;
pub const kSpatialMixerAttenuationCurve_Power: u32 = 0;
pub const kSpatialMixerAttenuationCurve_Exponential: u32 = 1;
pub const kSpatialMixerAttenuationCurve_Inverse: u32 = 2;
pub const kSpatialMixerAttenuationCurve_Linear: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MixerDistanceParams {
    pub mReferenceDistance: c_float,
    pub mMaxDistance: c_float,
    pub mMaxAttenuation: c_float,
}

impl Default for MixerDistanceParams {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type AUSpatialMixerRenderingFlags = u32;
pub const kSpatialMixerRenderingFlags_InterAuralDelay: u32 = (1 << 0);
pub const kSpatialMixerRenderingFlags_DistanceAttenuation: u32 = (1 << 2);

// Deprecated in 10.11
#[cfg(feature = "deprecate")]
pub const kAudioUnitProperty_3DMixerDistanceParams: AudioUnitPropertyID = 3010;
#[cfg(feature = "deprecate")]
pub const kAudioUnitProperty_3DMixerAttenuationCurve: AudioUnitPropertyID = 3013;
#[cfg(feature = "deprecate")]
pub const kAudioUnitProperty_DopplerShift: AudioUnitPropertyID = 3002;
#[cfg(feature = "deprecate")]
pub const kAudioUnitProperty_3DMixerRenderingFlags: AudioUnitPropertyID = 3003;
#[cfg(feature = "deprecate")]
pub const kAudioUnitProperty_3DMixerDistanceAtten: AudioUnitPropertyID = 3004;
#[cfg(feature = "deprecate")]
pub const kAudioUnitProperty_ReverbPreset: AudioUnitPropertyID = 3012;

pub type AU3DMixerRenderingFlags = u32;
pub const k3DMixerRenderingFlags_InterAuralDelay: u32 = (1 << 0);
pub const k3DMixerRenderingFlags_DopplerShift: u32 = (1 << 1);
pub const k3DMixerRenderingFlags_DistanceAttenuation: u32 = (1 << 2);
pub const k3DMixerRenderingFlags_DistanceFilter: u32 = (1 << 3);
pub const k3DMixerRenderingFlags_DistanceDiffusion: u32 = (1 << 4);
pub const k3DMixerRenderingFlags_LinearDistanceAttenuation: u32 = (1 << 5);
pub const k3DMixerRenderingFlags_ConstantReverbBlend: u32 = (1 << 6);

pub type AU3DMixerAttenuationCurve = u32;
pub const k3DMixerAttenuationCurve_Power: u32 = 0;
pub const k3DMixerAttenuationCurve_Exponential: u32 = 1;
pub const k3DMixerAttenuationCurve_Inverse: u32 = 2;
pub const k3DMixerAttenuationCurve_Linear: u32 = 3;

pub const kAudioUnitProperty_ScheduleAudioSlice: AudioUnitPropertyID = 3300;
pub const kAudioUnitProperty_ScheduleStartTimeStamp: AudioUnitPropertyID = 3301;
pub const kAudioUnitProperty_CurrentPlayTime: AudioUnitPropertyID = 3302;

pub type AUScheduledAudioSliceFlags = u32;
pub const kScheduledAudioSliceFlag_Complete: u32 = 1;
pub const kScheduledAudioSliceFlag_BeganToRender: u32 = 2;
pub const kScheduledAudioSliceFlag_BeganToRenderLate: u32 = 4;

pub type ScheduledAudioSliceCompletionProc =
    Option<extern fn(userData: *mut c_void, bufferList: *mut ScheduledAudioSlice)>;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScheduledAudioSlice {
    pub mTimeStamp: AudioTimeStamp,
    pub mCompletionProc: ScheduledAudioSliceCompletionProc,
    pub mCompletionProcUserData: *mut c_void,
    pub mFlags: u32,
    pub mReserved: u32,
    pub mReserved2: *mut c_void,
    pub mNumberFrames: u32,
    pub mBufferList: *mut AudioBufferList,
}

impl Default for ScheduledAudioSlice {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kAudioUnitProperty_ScheduledFileIDs: AudioUnitPropertyID = 3310;
pub const kAudioUnitProperty_ScheduledFileRegion: AudioUnitPropertyID = 3311;
pub const kAudioUnitProperty_ScheduledFilePrime: AudioUnitPropertyID = 3312;
pub const kAudioUnitProperty_ScheduledFileBufferSizeFrames: AudioUnitPropertyID = 3313;
pub const kAudioUnitProperty_ScheduledFileNumberBuffers: AudioUnitPropertyID = 3314;

pub type ScheduledAudioFileRegionCompletionProc = Option<
    extern fn(
        userData: *mut c_void,
        fileRegion: *mut ScheduledAudioFileRegion,
        result: OSStatus,
    ),
>;

pub enum OpaqueAudioFileID {
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ScheduledAudioFileRegion {
    pub mTimeStamp: AudioTimeStamp,
    pub mCompletionProc: ScheduledAudioFileRegionCompletionProc,
    pub mCompletionProcUserData: *mut c_void,
    pub mAudioFile: *mut OpaqueAudioFileID,
    pub mLoopCount: u32,
    pub mStartFrame: i64,
    pub mFramesToPlay: u32,
}

impl Default for ScheduledAudioFileRegion {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kMusicDeviceProperty_UsesInternalReverb: AudioUnitPropertyID =
    kAudioUnitProperty_UsesInternalReverb;
pub const kMusicDeviceProperty_SoundBankData: AudioUnitPropertyID = 1008;
pub const kMusicDeviceProperty_StreamFromDisk: AudioUnitPropertyID = 1011;
pub const kMusicDeviceProperty_SoundBankFSRef: AudioUnitPropertyID = 1012;

pub const kMusicDeviceProperty_InstrumentName: AudioUnitPropertyID = 1001;
pub const kMusicDeviceProperty_InstrumentNumber: AudioUnitPropertyID = 1004;

pub const kMusicDeviceProperty_InstrumentCount: AudioUnitPropertyID = 1000;
pub const kMusicDeviceProperty_BankName: AudioUnitPropertyID = 1007;
pub const kMusicDeviceProperty_SoundBankURL: AudioUnitPropertyID = 1100;

pub const kAUMIDISynthProperty_EnablePreload: AudioUnitPropertyID = 4119;

// range (4100->4999)
pub const kAUSamplerProperty_LoadInstrument: AudioUnitPropertyID = 4102;
pub const kAUSamplerProperty_LoadAudioFiles: AudioUnitPropertyID = 4101;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUSamplerInstrumentData {
    pub fileURL: CFURLRef,
    pub instrumentType: u8,
    pub bankMSB: u8,
    pub bankLSB: u8,
    pub presetID: u8,
}

impl Default for AUSamplerInstrumentData {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kInstrumentType_DLSPreset: u32 = 1;
pub const kInstrumentType_SF2Preset: u32 = kInstrumentType_DLSPreset;
pub const kInstrumentType_AUPreset: u32 = 2;
pub const kInstrumentType_Audiofile: u32 = 3;
pub const kInstrumentType_EXS24: u32 = 4;

pub const kAUSampler_DefaultPercussionBankMSB: u32 = 120;
pub const kAUSampler_DefaultMelodicBankMSB: u32 = 121;
pub const kAUSampler_DefaultBankLSB: u32 = 0;

pub const kAudioUnitProperty_DeferredRendererPullSize: AudioUnitPropertyID = 3320;
pub const kAudioUnitProperty_DeferredRendererExtraLatency: AudioUnitPropertyID = 3321;
pub const kAudioUnitProperty_DeferredRendererWaitFrames: AudioUnitPropertyID = 3322;

pub const kAUNetReceiveProperty_Hostname: AudioUnitPropertyID = 3511;
pub const kAUNetReceiveProperty_Password: AudioUnitPropertyID = 3512;

pub const kAUNetSendProperty_PortNum: AudioUnitPropertyID = 3513;
pub const kAUNetSendProperty_TransmissionFormat: AudioUnitPropertyID = 3514;
pub const kAUNetSendProperty_TransmissionFormatIndex: AudioUnitPropertyID = 3515;
pub const kAUNetSendProperty_ServiceName: AudioUnitPropertyID = 3516;
pub const kAUNetSendProperty_Disconnect: AudioUnitPropertyID = 3517;
pub const kAUNetSendProperty_Password: AudioUnitPropertyID = 3518;

pub const kAUNetSendPresetFormat_PCMFloat32: AudioUnitPropertyID = 0;
pub const kAUNetSendPresetFormat_PCMInt24: AudioUnitPropertyID = 1;
pub const kAUNetSendPresetFormat_PCMInt16: AudioUnitPropertyID = 2;
pub const kAUNetSendPresetFormat_Lossless24: AudioUnitPropertyID = 3;
pub const kAUNetSendPresetFormat_Lossless16: AudioUnitPropertyID = 4;
pub const kAUNetSendPresetFormat_ULaw: AudioUnitPropertyID = 5;
pub const kAUNetSendPresetFormat_IMA4: AudioUnitPropertyID = 6;
pub const kAUNetSendPresetFormat_AAC_128kbpspc: AudioUnitPropertyID = 7;
pub const kAUNetSendPresetFormat_AAC_96kbpspc: AudioUnitPropertyID = 8;
pub const kAUNetSendPresetFormat_AAC_80kbpspc: AudioUnitPropertyID = 9;
pub const kAUNetSendPresetFormat_AAC_64kbpspc: AudioUnitPropertyID = 10;
pub const kAUNetSendPresetFormat_AAC_48kbpspc: AudioUnitPropertyID = 11;
pub const kAUNetSendPresetFormat_AAC_40kbpspc: AudioUnitPropertyID = 12;
pub const kAUNetSendPresetFormat_AAC_32kbpspc: AudioUnitPropertyID = 13;
pub const kAUNetSendPresetFormat_AAC_LD_64kbpspc: AudioUnitPropertyID = 14;
pub const kAUNetSendPresetFormat_AAC_LD_48kbpspc: AudioUnitPropertyID = 15;
pub const kAUNetSendPresetFormat_AAC_LD_40kbpspc: AudioUnitPropertyID = 16;
pub const kAUNetSendPresetFormat_AAC_LD_32kbpspc: AudioUnitPropertyID = 17;
pub const kAUNetSendNumPresetFormats: AudioUnitPropertyID = 18;

// GENERIC
pub const kAudioUnitParameterFlag_Global: u32 = (1 << 0);
pub const kAudioUnitParameterFlag_Input: u32 = (1 << 1);
pub const kAudioUnitParameterFlag_Output: u32 = (1 << 2);
pub const kAudioUnitParameterFlag_Group: u32 = (1 << 3);

pub const kAudioUnitParameterFlag_HasName: u32 =
    kAudioUnitParameterFlag_ValuesHaveStrings;

#[cfg(feature = "deprecated")]
pub const kAudioUnitProperty_SetInputCallback: AudioUnitPropertyID = 7;
pub const kAudioUnitProperty_SRCAlgorithm: AudioUnitPropertyID = 9;
pub const kAudioUnitProperty_MIDIControlMapping: AudioUnitPropertyID = 17;
pub const kAudioUnitProperty_CurrentPreset: AudioUnitPropertyID = 28;

pub const kAudioUnitProperty_ParameterValueName: AudioUnitPropertyID =
    kAudioUnitProperty_ParameterStringFromValue;
pub const kAudioUnitProperty_BusCount: AudioUnitPropertyID =
    kAudioUnitProperty_ElementCount;

pub const kAudioOfflineUnitProperty_InputSize: AudioUnitPropertyID =
    kAudioUnitOfflineProperty_InputSize;
pub const kAudioOfflineUnitProperty_OutputSize: AudioUnitPropertyID =
    kAudioUnitOfflineProperty_OutputSize;

pub const kAudioUnitSRCAlgorithm_Polyphase: u32 = 1886350457;
pub const kAudioUnitSRCAlgorithm_MediumQuality: u32 = 1668510307;

// Deprecated. See AudioUnitParameterStringFromValue for equivalent
// structure, but with clearer field names
#[cfg(feature = "deprecated")]
#[derive(Clone, Copy, Debug)]
pub struct AudioUnitParameterValueName {
    pub inParamID: AudioUnitParameterID,
    pub inValue: *const c_float,
    pub outName: CFStringRef,
}

#[cfg(feature = "deprecated")]
impl Default for AudioUnitParameterValueName {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

// Deprecated. These properties are Apple specific.

pub const kMusicDeviceProperty_GroupOutputBus: AudioUnitPropertyID = 1002;
pub const kMusicDeviceProperty_SoundBankFSSpec: AudioUnitPropertyID = 1003;
pub const kAudioUnitProperty_PannerMode: AudioUnitPropertyID = 3008;

pub const kAudioUnitProperty_SpeakerConfiguration: AudioUnitPropertyID = 3001;

// Deprecated in favor of the newer AudioChannelLayout
// structure and its supporting property.
pub const kSpeakerConfiguration_HeadPhones: u32 = 0;
pub const kSpeakerConfiguration_Stereo: u32 = 1;
pub const kSpeakerConfiguration_Quad: u32 = 2;
pub const kSpeakerConfiguration_5_0: u32 = 3;
pub const kSpeakerConfiguration_5_1: u32 = kSpeakerConfiguration_5_0;

// Deprecated in favor of the newer AUSamplerInstrumentData
// structure and its supporting property.

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUSamplerBankPresetData {
    pub bankURL: CFURLRef,
    pub bankMSB: u8,
    pub bankLSB: u8,
    pub presetID: u8,
    pub reserved: u8,
}

impl Default for AUSamplerBankPresetData {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub const kAUSamplerProperty_LoadPresetFromBank: AudioUnitPropertyID = 4100;
pub const kAUSamplerProperty_BankAndPreset: AudioUnitPropertyID =
    kAUSamplerProperty_LoadPresetFromBank;
