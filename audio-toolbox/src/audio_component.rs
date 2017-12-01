use audio_toolbox_sys as ffi;
use ffi_binding::Binding;

use std::ptr;

bitflags! {
    pub struct AudioComponentFlags: ffi::AudioComponentFlags {
        const UNSEARCHABLE = ffi::kAudioComponentFlag_Unsearchable;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AudioComponentDescription(ffi::AudioComponentDescription);

impl AudioComponentDescription {
    pub fn new() -> Self {
        AudioComponentDescription(Default::default())
    }

    pub fn kind(&mut self, kind: ffi::OSType) -> &mut Self {
        self.0.componentType = kind;
        self
    }

    pub fn sub_kind(&mut self, kind: ffi::OSType) -> &mut Self {
        self.0.componentSubType = kind;
        self
    }

    pub fn flags(&mut self, flags: u32) -> &mut Self {
        self.0.componentFlags = flags;
        self
    }

    pub fn flags_mask(&mut self, mask: u32) -> &mut Self {
        self.0.componentFlagsMask = mask;
        self
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
