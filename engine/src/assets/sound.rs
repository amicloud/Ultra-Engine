use crate::assets::handles::SoundHandle;

pub struct Sound {
    pub id: SoundHandle,
    pub source: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub data: Vec<u8>,
}

impl Sound {
    pub fn new(id: SoundHandle, source: String, sample_rate: u32, channels: u16, data: Vec<u8>) -> Self {
        Self {
            id,
            source,
            sample_rate,
            channels,
            data,
        }
    }
}