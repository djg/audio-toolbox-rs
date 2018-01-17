use audio_toolbox_sys as ffi;
use core_audio::Error;

pub fn cvt_r(ret: ffi::OSStatus) -> Result<(), Error> {
    match ret {
        0 => Ok(()),
        e => Err(Error::from_osstatus(e)),
    }
}
