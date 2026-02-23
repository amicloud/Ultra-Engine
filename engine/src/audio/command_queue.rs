use bevy_ecs::prelude::*;

use crate::{
    SoundHandle,
    audio::audio_mixer::{ListenerInfo, SourceInfo},
};

#[derive(Debug)]
pub enum AudioCommand {
    PlaySound {
        track: usize,
        sound: SoundHandle,
        volume: f32,
        looping: bool,
        source: Option<Entity>,
    },
    PauseTrack {
        track: usize,
    },
    ResumeTrack {
        track: usize,
    },
    PauseMix,
    ResumeMix,
    MuteMix,
    UnmuteMix,
    UpdateListenerInfo {
        info: ListenerInfo,
    },
    UpdateSourceInfo {
        entity: Entity,
        info: SourceInfo,
    },
}

#[derive(Resource, Default)]
pub struct AudioCommandQueue {
    pub queue: Vec<AudioCommand>,
}

impl AudioCommandQueue {
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn push(&mut self, command: AudioCommand) {
        self.queue.push(command);
    }

    pub fn pop(&mut self) -> Option<AudioCommand> {
        self.queue.pop()
    }
}
