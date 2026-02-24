use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use glam::Vec3;

use crate::audio::{audio_mixer::ListenerInfo, voice::Voice};

#[derive(Debug)]
pub(crate) struct Track {
    pub(crate) volume: f32,
    pub(crate) playing: bool,
    pub(crate) voices: Vec<Voice>,
    pub(crate) buffer: Vec<f32>,
    pub(crate) channels: usize,
    pub(crate) finished_indices_buffer: Vec<usize>,
    pub(crate) muted: bool,
}
impl Track {
    pub fn fill_buffer_from_voices(
        &mut self,
        listener_info: Option<&ListenerInfo>,
        required_frames: usize,
        source_map: &HashMap<Entity, Vec3>,
    ) {
        self.finished_indices_buffer.clear();
        self.buffer.fill(0.0);
        if !self.playing {
            return;
        }
        let mute_gain = if self.muted { 0.0 } else { 1.0 };
        for (i, voice) in self.voices.iter_mut().enumerate() {
            if voice.next_block(listener_info, required_frames, source_map) {
                for frame in 0..required_frames {
                    for ch in 0..self.channels {
                        let src_ch = if voice.channels() == 1 { 0 } else { ch };
                        self.buffer[frame * self.channels + ch] += voice.buffer
                            [frame * voice.channels() + src_ch]
                            * self.volume
                            * mute_gain;
                    }
                }
            } else {
                self.finished_indices_buffer.push(i);
            }
        }
        for &index in self.finished_indices_buffer.iter().rev() {
            self.voices.swap_remove(index);
        }
    }
}
