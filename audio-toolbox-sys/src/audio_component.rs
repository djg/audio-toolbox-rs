use super::*;
use std::mem;
use std::os::raw::c_void;

pub type AudioComponentFlags = u32;
pub const kAudioComponentFlag_Unsearchable: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioComponentDescription {
    pub componentType: OSType,
    pub componentSubType: OSType,
    pub componentManufacturer: OSType,
    pub componentFlags: u32,
    pub componentFlagsMask: u32,
}

impl Default for AudioComponentDescription {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub enum OpaqueAudioComponent {}
pub type AudioComponent = *mut OpaqueAudioComponent;
pub enum ComponentInstanceRecord {}
pub type AudioComponentInstance = *mut ComponentInstanceRecord;

pub type AudioComponentMethod = extern fn(_self: *mut c_void, ...) -> OSStatus;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioComponentPlugInInterface {
    pub Open: Option<
        extern fn(_self: *mut c_void, mInstance: AudioComponentInstance) -> OSStatus,
    >,
    pub Close: Option<extern fn(_self: *mut c_void) -> OSStatus>,
    pub Lookup: Option<extern fn(selector: i16) -> Option<AudioComponentMethod>>,
    pub reserved: *mut c_void,
}

impl Default for AudioComponentPlugInInterface {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type AudioComponentFactoryFunction = Option<
    extern fn(inDesc: *const AudioComponentDescription)
        -> *mut AudioComponentPlugInInterface,
>;

extern {
    pub fn AudioComponentFindNext(
        inComponent: AudioComponent,
        inDesc: *const AudioComponentDescription,
    ) -> AudioComponent;
    pub fn AudioComponentCount(inDesc: *const AudioComponentDescription) -> u32;
    pub fn AudioComponentCopyName(
        inComponent: AudioComponent,
        outName: *mut CFStringRef,
    ) -> OSStatus;
    pub fn AudioComponentGetDescription(
        inComponent: AudioComponent,
        outDesc: *mut AudioComponentDescription,
    ) -> OSStatus;
    pub fn AudioComponentGetVersion(
        inComponent: AudioComponent,
        outVersion: *mut u32,
    ) -> OSStatus;
    pub fn AudioComponentInstanceNew(
        inComponent: AudioComponent,
        outInstance: *mut AudioComponentInstance,
    ) -> OSStatus;
    pub fn AudioComponentInstanceDispose(inInstance: AudioComponentInstance) -> OSStatus;
    pub fn AudioComponentInstanceGetComponent(
        inInstance: AudioComponentInstance,
    ) -> AudioComponent;
    pub fn AudioComponentInstanceCanDo(
        inInstance: AudioComponentInstance,
        inSelectorID: i16,
    ) -> Boolean;
    pub fn AudioComponentRegister(
        inDesc: *const AudioComponentDescription,
        inName: CFStringRef,
        inVersion: u32,
        inFactory: AudioComponentFactoryFunction,
    ) -> AudioComponent;
    pub fn AudioComponentCopyConfigurationInfo(
        inComponent: AudioComponent,
        outConfigurationInfo: *mut CFDictionaryRef,
    ) -> OSStatus;
    pub fn AudioComponentValidate(
        inComponent: AudioComponent,
        inValidationParameters: CFDictionaryRef,
        outValidationResult: *mut AudioComponentValidationResult,
    ) -> OSStatus;
}

pub type AudioComponentValidationResult = u32;
pub const kAudioComponentValidationResult_Unknown: u32 = 0;
pub const kAudioComponentValidationResult_Passed: u32 = 1;
pub const kAudioComponentValidationResult_Failed: u32 = 2;
pub const kAudioComponentValidationResult_TimedOut: u32 = 3;
pub const kAudioComponentValidationResult_UnauthorizedError_Open: u32 = 4;
pub const kAudioComponentValidationResult_UnauthorizedError_Init: u32 = 5;

pub const kAudioComponentConfigurationInfo_ValidationResult: &'static str =
    "ValidationResult";
pub const kAudioComponentValidationParameter_TimeOut: &'static str = "TimeOut";
pub const kAudioComponentValidationParameter_ForceValidation: &'static str =
    "ForceValidation";
