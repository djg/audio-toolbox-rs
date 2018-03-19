extern crate ctest;

//use std::env;

fn main() {
    // for (k, v) in env::vars_os() {
    //     println!("{:?}={:?}", k, v);
    // }

    let mut cfg = ctest::TestGenerator::new();
    cfg.header(
        "/System/Library/Frameworks/AudioToolbox.framework/Headers/AudioToolbox.h",
    );
    cfg.skip_field(|s, f| {
        if s != "AUDistanceAttenuationData" {
            return false;
        }
        match f {
            "pairs" => true,
            _ => false,
        }
    });
    cfg.skip_signededness(|s| match s {
        "AudioComponent"
        | "AudioComponentFactoryFunction"
        | "AudioComponentInstance"
        | "AudioComponentMethod"
        | "AudioDeviceIOProcID"
        | "AudioUnit"
        | "AudioUnitParameterValue" => true,
        s if s.starts_with("CF") && s.ends_with("Ref") => true,
        s if s.ends_with("Callback") => true,
        s if s.starts_with("HostCallback_") => true,
        s if s.ends_with("Proc") => true,
        _ => false,
    });
    cfg.skip_struct(|s| match s {
        // Implementations of C unions.
        "AudioUnitParameterEventRamp" | "AudioUnitParameterEventImmediate" => true,
        "AUDistanceAttenuationDataPair" => true,
        _ => false,
    });
    cfg.generate("../audio-toolbox-sys/src/lib.rs", "all.rs");
}
