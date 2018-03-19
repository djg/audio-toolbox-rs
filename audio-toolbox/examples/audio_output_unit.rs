extern crate audio_toolbox;
extern crate core_audio;

use audio_toolbox::{AudioOutputUnit, AudioUnitScope, AudioUnitSubType};
use core_audio::AudioDevice;

macro_rules! p {
    ($unit:expr, $scope:expr, $elem:expr, $($p:ident),*) => {
        $(
            if let Ok((size, writable)) = $unit.get_property_info(AudioDevice::$p, $scope, $elem) {
                println!(concat!(stringify!($p), " = {}, {}"), size, writable);
            }
        )*
    }
}

fn main() {
    let unit = AudioOutputUnit::new_instance(|c| {
        let desc = c.description();
        if let Ok(desc) = desc {
            println!(
                "type = {:?}, sub_type = \
                 {:?}, manufacturer = \
                 {:?}, flags = {:x}",
                desc.kind(),
                desc.sub_kind(),
                desc.manufacturer(),
                desc.flags()
            );
        }
        false
    });
    assert!(unit.is_ok());

    let unit = AudioOutputUnit::new_instance(|c| {
        if let Ok(desc) = c.description() {
            desc.sub_kind() == AudioUnitSubType::HALOutput
        } else {
            false
        }
    }).unwrap();
    assert!(unit.is_some());
    let unit = unit.unwrap();

    for &(scope, elem) in &[(AudioUnitScope::Output, 0), (AudioUnitScope::Input, 1)] {
        println!("*** {:?} ***", scope);
        // core_audio_sys::kAudioDevicePropertyPlugIn,
        // core_audio_sys::kAudioDevicePropertyDeviceHasChanged,

        p!(
            unit,
            scope,
            elem,
            PLUG_IN,
            DEVICE_HAS_CHANGED,
            DEVICE_IS_RUNNING_SOMEWHERE,
            R_OVERLOAD,
            IOSTOPPED_ABNORMALLY,
            HOG_MODE,
            BUFFER_FRAME_SIZE,
            BUFFER_FRAME_SIZE_RANGE,
            USES_VARIABLE_BUFFER_FRAME_SIZES,
            IOCYCLE_USAGE,
            STREAM_CONFIGURATION,
            IOPROC_STREAM_USAGE,
            ACTUAL_SAMPLE_RATE,
            CLOCK_DEVICE,
            JACK_IS_CONNECTED,
            VOLUME_SCALAR,
            VOLUME_DECIBELS,
            VOLUME_RANGE_DECIBELS,
            VOLUME_SCALAR_TO_DECIBELS,
            VOLUME_DECIBELS_TO_SCALAR,
            STEREO_PAN,
            STEREO_PAN_CHANNELS,
            MUTE,
            SOLO,
            PHANTOM_POWER,
            PHASE_INVERT,
            CLIP_LIGHT,
            TALKBACK,
            LISTENBACK,
            DATA_SOURCE,
            DATA_SOURCES,
            DATA_SOURCE_NAME_FOR_IDCFSTRING,
            DATA_SOURCE_KIND_FOR_ID,
            CLOCK_SOURCE,
            CLOCK_SOURCES,
            CLOCK_SOURCE_NAME_FOR_IDCFSTRING,
            CLOCK_SOURCE_KIND_FOR_ID,
            PLAY_THRU,
            PLAY_THRU_SOLO,
            PLAY_THRU_VOLUME_SCALAR,
            PLAY_THRU_VOLUME_DECIBELS,
            PLAY_THRU_VOLUME_RANGE_DECIBELS,
            PLAY_THRU_VOLUME_SCALAR_TO_DECIBELS,
            PLAY_THRU_VOLUME_DECIBELS_TO_SCALAR,
            PLAY_THRU_STEREO_PAN,
            PLAY_THRU_STEREO_PAN_CHANNELS,
            PLAY_THRU_DESTINATION,
            PLAY_THRU_DESTINATIONS,
            PLAY_THRU_DESTINATION_NAME_FOR_IDCFSTRING,
            CHANNEL_NOMINAL_LINE_LEVEL,
            CHANNEL_NOMINAL_LINE_LEVELS,
            CHANNEL_NOMINAL_LINE_LEVEL_NAME_FOR_IDCFSTRING,
            HIGH_PASS_FILTER_SETTING,
            HIGH_PASS_FILTER_SETTINGS,
            HIGH_PASS_FILTER_SETTING_NAME_FOR_IDCFSTRING,
            SUB_VOLUME_SCALAR,
            SUB_VOLUME_DECIBELS,
            SUB_VOLUME_RANGE_DECIBELS,
            SUB_VOLUME_SCALAR_TO_DECIBELS,
            SUB_VOLUME_DECIBELS_TO_SCALAR,
            SUB_MUTE
        );
    }
}
