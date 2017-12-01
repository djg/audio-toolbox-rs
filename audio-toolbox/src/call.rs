use audio_toolbox_sys as ffi;
use core_audio::Error;
use ffi_binding::Convert;
use AudioUnitSubType;

macro_rules! try_call {
    (ffi::$p:ident ($($e:expr),*)) => ({
        match ::call::try(call!(ffi::$p($($e),*))) { 
            Ok(o) => o,
            Err(e) => return Err(e).into()
        }
    })
}

pub fn try(ret: ffi::OSStatus) -> Result<(), Error> {
    match ret {
        0 => Ok(()),
        e => Err(Error::from_osstatus(e)),
    }
}

impl Convert<ffi::OSType> for AudioUnitSubType {
    fn convert(&self) -> ffi::OSType {
        match *self {
            AudioUnitSubType::GenericOutput => ffi::kAudioUnitSubType_GenericOutput,
            AudioUnitSubType::VoiceProcessingIO => ffi::kAudioUnitSubType_VoiceProcessingIO,
            AudioUnitSubType::HALOutput => ffi::kAudioUnitSubType_HALOutput,
            AudioUnitSubType::DefaultOutput => ffi::kAudioUnitSubType_DefaultOutput,
            AudioUnitSubType::SystemOutput => ffi::kAudioUnitSubType_SystemOutput,
            
            AudioUnitSubType::DLSSynth => ffi::kAudioUnitSubType_DLSSynth,
            AudioUnitSubType::Sampler => ffi::kAudioUnitSubType_Sampler,
            AudioUnitSubType::MIDISynth => ffi::kAudioUnitSubType_MIDISynth,
            
            AudioUnitSubType::AUConverter => ffi::kAudioUnitSubType_AUConverter,
            AudioUnitSubType::Varispeed => ffi::kAudioUnitSubType_Varispeed,
            AudioUnitSubType::DeferredRenderer => ffi::kAudioUnitSubType_DeferredRenderer,
            AudioUnitSubType::Splitter => ffi::kAudioUnitSubType_Splitter,
            AudioUnitSubType::MultiSplitter => ffi::kAudioUnitSubType_MultiSplitter,
            AudioUnitSubType::Merger => ffi::kAudioUnitSubType_Merger,
            AudioUnitSubType::NewTimePitch => ffi::kAudioUnitSubType_NewTimePitch,
            AudioUnitSubType::AUiPodTimeOther => ffi::kAudioUnitSubType_AUiPodTimeOther,
            AudioUnitSubType::RoundTripAAC => ffi::kAudioUnitSubType_RoundTripAAC,
            
            AudioUnitSubType::TimePitch => ffi::kAudioUnitSubType_TimePitch,
            
            AudioUnitSubType::PeakLimiter => ffi::kAudioUnitSubType_PeakLimiter,
            AudioUnitSubType::DynamicsProcessor => ffi::kAudioUnitSubType_DynamicsProcessor,
            AudioUnitSubType::LowPassFilter => ffi::kAudioUnitSubType_LowPassFilter,
            AudioUnitSubType::HighPassFilter => ffi::kAudioUnitSubType_HighPassFilter,
            AudioUnitSubType::BandPassFilter => ffi::kAudioUnitSubType_BandPassFilter,
            AudioUnitSubType::HighShelfFilter => ffi::kAudioUnitSubType_HighShelfFilter,
            AudioUnitSubType::LowShelfFilter => ffi::kAudioUnitSubType_LowShelfFilter,
            AudioUnitSubType::ParametricEQ => ffi::kAudioUnitSubType_ParametricEQ,
            AudioUnitSubType::Distortion => ffi::kAudioUnitSubType_Distortion,
            AudioUnitSubType::Delay => ffi::kAudioUnitSubType_Delay,
            AudioUnitSubType::SampleDelay => ffi::kAudioUnitSubType_SampleDelay,
            AudioUnitSubType::NBandEQ => ffi::kAudioUnitSubType_NBandEQ,
            
            AudioUnitSubType::GraphicEQ => ffi::kAudioUnitSubType_GraphicEQ,
            AudioUnitSubType::MultiBandCompressor => ffi::kAudioUnitSubType_MultiBandCompressor,
            AudioUnitSubType::MatrixReverb => ffi::kAudioUnitSubType_MatrixReverb,
            AudioUnitSubType::Pitch => ffi::kAudioUnitSubType_Pitch,
            AudioUnitSubType::AUFilter => ffi::kAudioUnitSubType_AUFilter,
            AudioUnitSubType::NetSend => ffi::kAudioUnitSubType_NetSend,
            AudioUnitSubType::RogerBeep => ffi::kAudioUnitSubType_RogerBeep,
            
            AudioUnitSubType::MultiChannelMixer => ffi::kAudioUnitSubType_MultiChannelMixer,
            AudioUnitSubType::MatrixMixer => ffi::kAudioUnitSubType_MatrixMixer,
            AudioUnitSubType::SpatialMixer => ffi::kAudioUnitSubType_SpatialMixer,
            
            AudioUnitSubType::StereoMixer => ffi::kAudioUnitSubType_StereoMixer,
            AudioUnitSubType::_3DMixer => ffi::kAudioUnitSubType_3DMixer,
            
            AudioUnitSubType::SphericalHeadPanner => ffi::kAudioUnitSubType_SphericalHeadPanner,
            AudioUnitSubType::VectorPanner => ffi::kAudioUnitSubType_VectorPanner,
            AudioUnitSubType::SoundFieldPanner => ffi::kAudioUnitSubType_SoundFieldPanner,
            AudioUnitSubType::HRTFPanner => ffi::kAudioUnitSubType_HRTFPanner,
            
            AudioUnitSubType::NetReceive => ffi::kAudioUnitSubType_NetReceive,
            AudioUnitSubType::ScheduledSoundPlayer => ffi::kAudioUnitSubType_ScheduledSoundPlayer,
            AudioUnitSubType::AudioFilePlayer => ffi::kAudioUnitSubType_AudioFilePlayer,
        }
    }
}
