use std::{collections::HashMap, f32::consts::PI, sync::Arc};

use bevy_ecs::entity::Entity;
use glam::Vec3;

use crate::audio::audio_mixer::ListenerInfo;

#[derive(Debug)]
pub(crate) struct Voice {
    samples: Arc<[f32]>,
    cursor: usize,
    volume: f32,
    looping: bool,
    pub(crate) channels: usize,
    pub(crate) buffer: Vec<f32>,
    source: Option<Entity>,
    source_channels: usize,
    itd_delay: ItdDelay,
    lpf: LowPassFilter,
}
const ITD_DELAY_BUFFER_SIZE: usize = 64; // Must be a power of two for efficient wrapping
#[derive(Debug)]
struct ItdDelay {
    buffer: [f32; ITD_DELAY_BUFFER_SIZE],
    write_idx: usize,
    mask: usize,  // buffer.len() - 1 (power-of-two size)
    range: usize, // Maximum delay in samples
}

#[derive(Debug)]
struct LowPassFilter {
    z: f32, // previous output
    alpha: f32,
}

impl LowPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        let out = self.z + self.alpha * (input - self.z);
        self.z = out;
        out
    }
}

impl Voice {
    pub(crate) fn channels(&self) -> usize {
        self.channels
    }

    pub(crate) fn new(
        samples: Arc<[f32]>,
        volume: f32,
        looping: bool,
        source: Option<Entity>,
        source_channels: usize,
        required_buffer_size: usize,
    ) -> Self {
        let itd_scale = 1.0; // Scale factor for ITD effect, for demonstration purposes
        let itd_max_time_seconds = 0.67 / 1000.0; //0.67 ms converted to seconds

        let sample_rate = 44100.0; // This should come in as an argument
        let itd_range = (itd_max_time_seconds * sample_rate * itd_scale) as usize; // Max delay in samples

        let low_pass_frequency_cutoff = 1500.0;
        let alpha = 1.0 - (-2.0 * PI * low_pass_frequency_cutoff / sample_rate).exp();

        Self {
            samples,
            cursor: 0,
            volume,
            looping,
            channels: 2, // We always output stereo from the voice, even if the source is mono. The mixer will handle downmixing if necessary.
            buffer: vec![0.0; required_buffer_size], // stereo output buffer
            source,
            source_channels,
            itd_delay: ItdDelay {
                buffer: [0.0; ITD_DELAY_BUFFER_SIZE],
                write_idx: 0,
                mask: ITD_DELAY_BUFFER_SIZE - 1, // buffer.len() - 1 (power-of-two size)
                range: itd_range,
            },
            lpf: LowPassFilter { z: 0.0, alpha },
        }
    }

    pub(crate) fn next_block(
        &mut self,
        listener_info: Option<&ListenerInfo>,
        required_frames: usize,
        source_map: &HashMap<Entity, Vec3>,
    ) -> bool {
        let total_frames = self.samples.len() / self.channels;
        let frames_to_fill = (total_frames - self.cursor).min(required_frames);

        let mut location = None;
        // Simple pan based spatialization
        let mut pan = 0.0; // -1.0 = full left, 0.0 = center, 1.0 = full right
        if let Some(source) = self.source {
            if let Some(_location) = source_map.get(&source) {
                location = Some(*_location);
            }
        }

        let distance_attenuation =
            if let (Some(location), Some((listener_pos, _))) = (location, listener_info) {
                let distance = location.distance(*listener_pos);
                // Raw inverse square attenuation feels too harsh.
                // Perhaps this should be tweaked or made configurable, but for now
                // we'll just use a modified inverse square that falls off more gently.
                1.0 / (1.0 + (distance.powi(2) / 5.0))
            } else {
                1.0
            };

        if let Some(location) = location {
            if let Some((listener_pos, listener_rot)) = listener_info {
                let dir = (location - listener_pos).normalize_or_zero();
                let right = listener_rot.mul_vec3(Vec3::X);

                // project direction onto listener's horizontal plane (XY)
                let dir_horizontal = Vec3::new(dir.x, dir.y, 0.0).normalize_or_zero();

                // compute signed pan: dot with right vector
                pan = right.dot(dir_horizontal).clamp(-1.0, 1.0);
            }
        }
        
        let ild_strength = 0.33;
        let pan_scaled = pan * ild_strength;

        let pan_rad = (pan_scaled + 1.0) * 0.25 * PI;
        let left_gain = pan_rad.cos();
        let right_gain = pan_rad.sin();

        let delay_signed = pan * self.itd_delay.range as f32;
        let delay_abs = delay_signed.abs();

        let d_int = delay_abs.floor() as usize;
        let frac = delay_abs - d_int as f32;
        let combined_volume = self.volume * distance_attenuation;

        for frame in 0..frames_to_fill {
            let sample_idx = self.cursor * self.source_channels;
            match self.source_channels {
                1 => {
                    let mono = self.samples[sample_idx] * combined_volume;
                    self.itd_delay.buffer[self.itd_delay.write_idx] = mono;

                    let read_base =
                        self.itd_delay.write_idx.wrapping_sub(d_int + 1) & self.itd_delay.mask;
                    let read_next = (read_base + 1) & self.itd_delay.mask;

                    let s0 = self.itd_delay.buffer[read_base];
                    let s1 = self.itd_delay.buffer[read_next];

                    let delayed = s0 * frac + s1 * (1.0 - frac);

                    let (left, right) = if delay_signed > 0.0 {
                        // sound right → left is far ear
                        let filtered = self.lpf.process(delayed);
                        (filtered, mono)
                    } else {
                        let filtered = self.lpf.process(delayed);
                        (mono, filtered)
                    };

                    self.buffer[frame * 2] = left * left_gain; // Left channel
                    self.buffer[frame * 2 + 1] = right * right_gain; // Right channel

                    self.itd_delay.write_idx = (self.itd_delay.write_idx + 1) & self.itd_delay.mask;
                }
                2 => {
                    // Stereo source, apply panning and distance attenuation to each channel
                    // but not ITD since the source is already stereo and that would really mess things up
                    let left_sample = self.samples[sample_idx] * combined_volume;
                    let right_sample = self.samples[sample_idx + 1] * combined_volume;
                    self.buffer[frame * 2] = left_sample * left_gain; // Left channel
                    self.buffer[frame * 2 + 1] = right_sample * right_gain; // Right channel
                }
                _ => {
                    // Unsupported channel count, silence output
                    if frame == 0 {
                        // Log this only for the first frame to avoid spamming the console
                        eprintln!(
                            "Unsupported channel count: {}, expected 1 or 2",
                            self.source_channels
                        );
                    }
                    self.buffer[frame * 2] = 0.0;
                    self.buffer[frame * 2 + 1] = 0.0;
                }
            }
            self.cursor += 1;
        }

        // Zero the rest of the block if we ran out of frames
        for frame in frames_to_fill..required_frames {
            for ch in 0..self.channels {
                self.buffer[frame * self.channels + ch] = 0.0;
            }
        }
        if self.cursor >= total_frames {
            if self.looping {
                self.cursor = 0;
            } else {
                return false;
            }
        }
        return self.looping || self.cursor < total_frames;
    }
}
