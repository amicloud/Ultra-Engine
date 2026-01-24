use bevy_ecs::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct MeshComponent {
    pub mesh_id: u32,
}
