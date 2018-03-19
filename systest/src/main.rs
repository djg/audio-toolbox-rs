#![allow(bad_style, improper_ctypes)]

extern crate audio_toolbox_sys;

use audio_toolbox_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
