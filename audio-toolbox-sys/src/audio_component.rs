use super::*;
use std::mem;
use std::os::raw::c_void;

e! {
    typedef CF_OPTIONS(u32, AudioComponentFlags) {
        kAudioComponentFlag_Unsearchable                = 1,
    };
}

s! {
    #[derive(Clone, Copy)]
    struct AudioComponentDescription {
        pub componentType: OSType,
        pub componentSubType: OSType,
        pub componentManufacturer: OSType,
        pub componentFlags: u32,
        pub componentFlagsMask: u32,
    }
}

pub enum OpaqueAudioComponent {}
pub type AudioComponent = *mut OpaqueAudioComponent;
pub enum ComponentInstanceRecord {}
pub type AudioComponentInstance = *mut ComponentInstanceRecord;

pub type AudioComponentMethod = extern fn(_self: *mut c_void, ...) -> OSStatus;

s! {
    #[derive(Clone, Copy)]
    struct AudioComponentPlugInInterface {
        pub Open: Option<extern fn(_self: *mut c_void, mInstance: AudioComponentInstance) -> OSStatus>,
        pub Close: Option<extern fn(_self: *mut c_void) -> OSStatus>,
        pub Lookup: Option<extern fn(selector: i16) -> Option<AudioComponentMethod>>,
        pub reserved: *mut c_void,
    }
}

pub type AudioComponentFactoryFunction =
    Option<
        extern "C" fn(inDesc: *const AudioComponentDescription)
                      -> *mut AudioComponentPlugInInterface,
    >;

extern "C" {
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
    pub fn AudioComponentGetVersion(inComponent: AudioComponent, outVersion: *mut u32) -> OSStatus;
    pub fn AudioComponentInstanceNew(
        inComponent: AudioComponent,
        outInstance: *mut AudioComponentInstance,
    ) -> OSStatus;
    pub fn AudioComponentInstanceDispose(inInstance: AudioComponentInstance) -> OSStatus;
    pub fn AudioComponentInstanceGetComponent(inInstance: AudioComponentInstance)
        -> AudioComponent;
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

e! {
    typedef CF_ENUM(u32, AudioComponentValidationResult)
    {
        kAudioComponentValidationResult_Unknown = 0,
        kAudioComponentValidationResult_Passed,
        kAudioComponentValidationResult_Failed,
        kAudioComponentValidationResult_TimedOut,
        kAudioComponentValidationResult_UnauthorizedError_Open,
        kAudioComponentValidationResult_UnauthorizedError_Init,
    };
}

cs! {
    #define kAudioComponentConfigurationInfo_ValidationResult   "ValidationResult"
    #define kAudioComponentValidationParameter_TimeOut          "TimeOut"
    #define kAudioComponentValidationParameter_ForceValidation	"ForceValidation"
}
