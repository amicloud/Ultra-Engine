use bevy_ecs::prelude::*;

use crate::{
    TransformComponent,
    audio::audio_control::AudioControl,
    components::{
        audio_source_component::AudioSourceComponent,
        single_audio_listener_component::SingleAudioListenerComponent,
    },
};

pub struct SpatialAudioSystem;

/// This system is responsible for updating the position and rotation of the audio listener
/// and the position of audio source entities in the world.
impl SpatialAudioSystem {
    /// This currently only supports having a single listener
    pub fn update_listener_position(
        query: Query<
            (Entity, &TransformComponent, &SingleAudioListenerComponent),
            Changed<TransformComponent>,
        >,
        mut audio_command_queue: ResMut<AudioControl>,
    ) {
        if query.iter().count() > 1 {
            log::error!(
                "Multiple entities with SingleAudioListenerComponent found. Only the first one will be used as the audio listener."
            );
        }
        if let Some((_, transform, _)) = query.iter().nth(0) {
            audio_command_queue.update_listener_info(transform.position, transform.rotation);
        }
    }

    pub fn update_moved_sources(
        query: Query<
            (Entity, &TransformComponent, &AudioSourceComponent),
            Changed<TransformComponent>,
        >,
        mut audio_control: ResMut<AudioControl>,
    ) {
        for (entity, transform, _) in query.iter() {
            audio_control.update_source_info(entity, transform.position);
        }
    }

    pub fn remove_deleted_sources(
        mut removed: RemovedComponents<AudioSourceComponent>,
        mut audio_control: ResMut<AudioControl>,
    ) {
        for entity in removed.read() {
            audio_control.remove_spatial_emitter(entity);
        }
    }
}
