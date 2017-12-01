#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate core_audio_sys;

#[macro_use]
mod macros;
mod au_component;
mod audio_component;
mod audio_unit_properties;

pub use au_component::*;
pub use audio_component::*;
pub use audio_unit_properties::*;

pub type Boolean = u8;
pub type OSStatus = i32;
pub type FourCharCode = u32;
pub type OSType = FourCharCode;

pub enum CFDictionary {}
pub type CFDictionaryRef = *const CFDictionary;

pub enum CFString {}
pub type CFStringRef = *const CFString;

pub enum CFURL {}
pub type CFURLRef = *const CFURL;
