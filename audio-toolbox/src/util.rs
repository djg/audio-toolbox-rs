use audio_toolbox_sys as ffi;
use call;

pub unsafe fn component_instance_dispose(instance: ffi::AudioComponentInstance) {
    call::cvt_r(ffi::AudioComponentInstanceDispose(instance)).expect("Disposing \
                                                                      component \
                                                                      instance should \
                                                                      succeed.");
}
