use {AudioUnitManufacturer, AudioUnitSubType, AudioUnitType};
use audio_toolbox_sys as ffi;
use call;
use core_audio::Result;
use foreign_types::{ForeignType, ForeignTypeRef};
use std::ptr;
use util::component_instance_dispose;

fn format_ostype(fcc: ffi::OSType) -> String {
    let a: char = (fcc >> 24) as u8 as char;
    let b: char = (fcc >> 16) as u8 as char;
    let c: char = (fcc >> 8) as u8 as char;
    let d: char = fcc as u8 as char;

    format!("'{}{}{}{}' (0x{:08x})", a, b, c, d, fcc)
}

bitflags! {
    pub struct AudioComponentFlags: ffi::AudioComponentFlags {
        const UNSEARCHABLE = ffi::kAudioComponentFlag_Unsearchable;
    }
}

ffi_type_stack! {
    type CType = ffi::AudioComponentDescription;
    pub struct AudioComponentDescription;
    pub struct AudioComponentDescriptionRef;
}

impl AudioComponentDescription {
    pub fn new(kind: ffi::OSType,
               sub_kind: ffi::OSType,
               manufacturer: ffi::OSType)
               -> Self
    {
        AudioComponentDescription(ffi::AudioComponentDescription {
            componentType: kind,
            componentSubType: sub_kind,
            componentManufacturer:
            manufacturer,
            ..Default::default() })
    }
}

impl AudioComponentDescriptionRef {
    pub fn kind(&self) -> AudioUnitType {
        use AudioUnitType::*;
        let desc: &ffi::AudioComponentDescription =
            unsafe { &*self.as_ptr() };
        match desc.componentType {
            ffi::kAudioUnitType_Output => Output,
            ffi::kAudioUnitType_MusicDevice => MusicDevice,
            ffi::kAudioUnitType_MusicEffect => MusicEffect,
            ffi::kAudioUnitType_FormatConverter => FormatConverter,
            ffi::kAudioUnitType_Effect => Effect,
            ffi::kAudioUnitType_Mixer => Mixer,
            ffi::kAudioUnitType_Panner => Panner,
            ffi::kAudioUnitType_Generator => Generator,
            ffi::kAudioUnitType_OfflineEffect => OfflineEffect,
            ffi::kAudioUnitType_MIDIProcessor => MIDIProcessor,
            t => panic!("Unknown AudioUnitType {}", format_ostype(t)),
        }
    }

    pub fn sub_kind(&self) -> AudioUnitSubType {
        use AudioUnitSubType::*;
        let desc: &ffi::AudioComponentDescription =
            unsafe { &*self.as_ptr() };
        match desc.componentSubType {
            ffi::kAudioUnitSubType_GenericOutput => GenericOutput,
            ffi::kAudioUnitSubType_VoiceProcessingIO => VoiceProcessingIO,

            ffi::kAudioUnitSubType_HALOutput => HALOutput,
            ffi::kAudioUnitSubType_DefaultOutput => DefaultOutput,
            ffi::kAudioUnitSubType_SystemOutput => SystemOutput,

            ffi::kAudioUnitSubType_DLSSynth => DLSSynth,
            ffi::kAudioUnitSubType_Sampler => Sampler,
            ffi::kAudioUnitSubType_MIDISynth => MIDISynth,

            ffi::kAudioUnitSubType_AUConverter => AUConverter,
            ffi::kAudioUnitSubType_Varispeed => Varispeed,
            ffi::kAudioUnitSubType_DeferredRenderer => DeferredRenderer,
            ffi::kAudioUnitSubType_Splitter => Splitter,
            ffi::kAudioUnitSubType_MultiSplitter => MultiSplitter,
            ffi::kAudioUnitSubType_Merger => Merger,
            ffi::kAudioUnitSubType_NewTimePitch => NewTimePitch,
            ffi::kAudioUnitSubType_AUiPodTimeOther => AUiPodTimeOther,
            ffi::kAudioUnitSubType_RoundTripAAC => RoundTripAAC,

            // This is same as kAudioUnitSubType_Pitch
            //ffi::kAudioUnitSubType_TimePitch => TimePitch,
            ffi::kAudioUnitSubType_PeakLimiter => PeakLimiter,
            ffi::kAudioUnitSubType_DynamicsProcessor => DynamicsProcessor,
            ffi::kAudioUnitSubType_LowPassFilter => LowPassFilter,
            ffi::kAudioUnitSubType_HighPassFilter => HighPassFilter,
            ffi::kAudioUnitSubType_BandPassFilter => BandPassFilter,
            ffi::kAudioUnitSubType_HighShelfFilter => HighShelfFilter,
            ffi::kAudioUnitSubType_LowShelfFilter => LowShelfFilter,
            ffi::kAudioUnitSubType_ParametricEQ => ParametricEQ,
            ffi::kAudioUnitSubType_Distortion => Distortion,
            ffi::kAudioUnitSubType_Delay => Delay,
            ffi::kAudioUnitSubType_SampleDelay => SampleDelay,
            ffi::kAudioUnitSubType_NBandEQ => NBandEQ,

            ffi::kAudioUnitSubType_GraphicEQ => GraphicEQ,
            ffi::kAudioUnitSubType_MultiBandCompressor => MultiBandCompressor,
            ffi::kAudioUnitSubType_MatrixReverb => MatrixReverb,
            ffi::kAudioUnitSubType_Pitch => Pitch,
            ffi::kAudioUnitSubType_AUFilter => AUFilter,
            ffi::kAudioUnitSubType_NetSend => NetSend,
            ffi::kAudioUnitSubType_RogerBeep => RogerBeep,

            ffi::kAudioUnitSubType_MultiChannelMixer => MultiChannelMixer,
            ffi::kAudioUnitSubType_MatrixMixer => MatrixMixer,
            ffi::kAudioUnitSubType_SpatialMixer => SpatialMixer,

            ffi::kAudioUnitSubType_StereoMixer => StereoMixer,
            ffi::kAudioUnitSubType_3DMixer => _3DMixer,

            ffi::kAudioUnitSubType_SphericalHeadPanner => SphericalHeadPanner,
            ffi::kAudioUnitSubType_VectorPanner => VectorPanner,
            ffi::kAudioUnitSubType_SoundFieldPanner => SoundFieldPanner,
            ffi::kAudioUnitSubType_HRTFPanner => HRTFPanner,

            ffi::kAudioUnitSubType_NetReceive => NetReceive,
            ffi::kAudioUnitSubType_ScheduledSoundPlayer => ScheduledSoundPlayer,
            ffi::kAudioUnitSubType_AudioFilePlayer => AudioFilePlayer,
            st => panic!("Unknown AudioUnitSubType {}", format_ostype(st)),
        }
    }

    pub fn manufacturer(&self) -> AudioUnitManufacturer {
        use AudioUnitManufacturer::*;
        let desc: &ffi::AudioComponentDescription =
            unsafe { &*self.as_ptr() };
        match desc.componentManufacturer {
            ffi::kAudioUnitManufacturer_Apple => Apple,
            m => panic!("Unknown AudioUnitManufacture {:?}", format_ostype(m)),
        }
    }

    pub fn flags(&self) -> u32 {
        let desc: &ffi::AudioComponentDescription =
            unsafe { &*self.as_ptr() };
        desc.componentFlags
    }

    pub fn count(&self) -> usize {
        unsafe { ffi::AudioComponentCount(self.as_ptr()) as _ }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AudioComponent(ffi::AudioComponent);

impl AudioComponent {
    pub fn iter(desc: &AudioComponentDescriptionRef) -> AudioComponentIter {
        AudioComponentIter { comp: ptr::null_mut(),
                             desc: desc, }
    }

    pub fn new(&self) -> Result<AudioComponentInstance> {
        let mut ptr: ffi::AudioComponentInstance = ptr::null_mut();
        unsafe {
            call::cvt_r(ffi::AudioComponentInstanceNew(self.0, &mut ptr))?;
            Ok(ForeignType::from_ptr(ptr))
        }
    }

    pub fn description(&self) -> Result<AudioComponentDescription> {
        let mut desc = ffi::AudioComponentDescription::default();
        unsafe {
            call::cvt_r(ffi::AudioComponentGetDescription(self.0, &mut desc))?;
            Ok(ForeignType::from_ptr(&mut desc))
        }
    }

    pub fn version(&self) -> Result<u32> {
        let mut version = 0u32;
        unsafe {
            call::cvt_r(ffi::AudioComponentGetVersion(self.0, &mut version))?;
        }
        Ok(version)
    }
}

impl ::std::convert::From<ffi::AudioComponent> for AudioComponent {
    fn from(ffi: ffi::AudioComponent) -> Self { AudioComponent(ffi) }
}

impl ::std::convert::Into<ffi::AudioComponent> for AudioComponent {
    fn into(self) -> ffi::AudioComponent { self.0 }
}

#[derive(Debug)]
pub struct AudioComponentIter<'a> {
    comp: ffi::AudioComponent,
    desc: &'a AudioComponentDescriptionRef,
}

impl<'a> Iterator for AudioComponentIter<'a> {
    type Item = AudioComponent;

    fn next(&mut self) -> Option<Self::Item> {
        let next = unsafe { ffi::AudioComponentFindNext(self.comp, self.desc.as_ptr()) };
        if next.is_null() {
            None
        } else {
            self.comp = next;
            Some(next.into())
        }
    }
}

//==============================================================================
// Audio Component Instance
foreign_type! {
    type CType = ffi::ComponentInstanceRecord;
    fn drop = component_instance_dispose;
    #[derive(Debug)]
    pub struct AudioComponentInstance;
    pub struct AudioComponentInstanceRef;
}
