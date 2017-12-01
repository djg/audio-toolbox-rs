use audio_toolbox_sys as ffi;
use core_audio::Result;
use ffi_binding::Binding;
use std::{mem, ptr};

bitflags! {
    pub struct AudioComponentFlags: ffi::AudioComponentFlags {
        const UNSEARCHABLE = ffi::kAudioComponentFlag_Unsearchable;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AudioComponentDescription(ffi::AudioComponentDescription);

impl AudioComponentDescription {
    pub fn new(kind: ffi::OSType,
               sub_kind: ffi::OSType,
               manufacturer: ffi::OSType) -> Self {
        AudioComponentDescription(ffi::AudioComponentDescription {
            componentType: kind,
            componentSubType: sub_kind,
            componentManufacturer: manufacturer,
            ..Default::default()
        })
    }

    pub fn kind(&self) -> ffi::OSType {
        self.0.componentType
    }

    pub fn sub_kind(&self) -> ffi::OSType {
        self.0.componentSubType 
    }

    pub fn manufacturer(&self) -> ffi::OSType {
        self.0.componentManufacturer 
    }
    
    pub fn flags(&self) -> u32 {
        self.0.componentFlags
    }

    pub fn count(&self) -> usize {
        unsafe {
            call!(ffi::AudioComponentCount(self.as_ffi())) as _
        }
    }

    pub fn as_ffi_mut(&mut self) -> *mut ffi::AudioComponentDescription {
        &mut self.0
    }
}

impl Binding for AudioComponentDescription {
    type Ffi = *const ffi::AudioComponentDescription;

    fn as_ffi(&self) -> Self::Ffi {
        &self.0
    }

    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
        AudioComponentDescription(*ffi)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AudioComponent(ffi::AudioComponent);

impl AudioComponent {
    pub fn iter(desc: &AudioComponentDescription) -> AudioComponentIter {
        AudioComponentIter {
            comp: AudioComponent(ptr::null_mut()),
            desc: desc,
        }
    }

    pub fn description(&self) -> Result<AudioComponentDescription> {
        let mut desc = AudioComponentDescription::default();
        unsafe {
            try_call!(ffi::AudioComponentGetDescription(self.as_ffi(),
                                                        desc.as_ffi_mut()));
        }
        Ok(desc)
    }
    
    pub fn version(&self) -> Result<u32> {
        let mut version = 0u32;
        unsafe {
            try_call!(ffi::AudioComponentGetVersion(self.as_ffi(), &mut version));
        }
        Ok(version)
    }
    
    pub fn new(&self) -> Result<AudioComponentInstance> {
        let mut instance: ffi::AudioComponentInstance = unsafe { mem::uninitialized() };
        unsafe {
            try_call!(ffi::AudioComponentInstanceNew(self.as_ffi(),
                                                     &mut instance));
            Ok(Binding::from_ffi(instance))
        }
    }
}

impl Binding for AudioComponent {
    type Ffi = ffi::AudioComponent;

    fn as_ffi(&self) -> Self::Ffi {
        self.0
    }

    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
        assert!(!ffi.is_null());
        AudioComponent(ffi)
    }
}

pub struct AudioComponentIter<'a> {
    comp: AudioComponent,
    desc: &'a AudioComponentDescription,
}

impl<'a> Iterator for AudioComponentIter<'a> {
    type Item = AudioComponent;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let raw = ffi::AudioComponentFindNext(self.comp.as_ffi(), self.desc.as_ffi());
            match Binding::from_ffi_opt(raw) {
                Some(comp) => {
                    self.comp = comp;
                    Some(comp)
                }
                None => None,
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AudioComponentInstance(ffi::AudioComponentInstance);

impl AudioComponentInstance {}

impl Binding for AudioComponentInstance {
    type Ffi = ffi::AudioComponentInstance;

    fn as_ffi(&self) -> Self::Ffi {
        self.0
    }

    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
        AudioComponentInstance(ffi)
    }
}

impl Drop for AudioComponentInstance {
    fn drop(&mut self) {
        unsafe {
            call!(ffi::AudioComponentInstanceDispose(self.as_ffi()));
        }
    }
}
