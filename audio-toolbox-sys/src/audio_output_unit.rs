use {AudioUnit, OSStatus};

extern {
    pub fn AudioOutputUnitStart(ci: AudioUnit) -> OSStatus;
    pub fn AudioOutputUnitStop(ci: AudioUnit) -> OSStatus;
}

// selector range
pub const kAudioOutputUnitRange: u32 = 512;
pub const kAudioOutputUnitStartSelect: u32 = 513;
pub const kAudioOutputUnitStopSelect: u32 = 514;
