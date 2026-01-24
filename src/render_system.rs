use crate::{
    mesh_component::MeshComponent, render_instance::RenderInstance, render_queue::RenderQueue,
    transform_component::TransformComponent,
};

use bevy_ecs::prelude::{Query, ResMut};
pub struct RenderSystem {}

impl RenderSystem {
    pub fn extract_render_data(
        query: Query<(&TransformComponent, &MeshComponent)>,
        mut render_queue: ResMut<RenderQueue>,
    ) {
        render_queue.instances.clear();

        for (transform, mesh) in &query {
            render_queue.instances.push(RenderInstance {
                mesh_id: mesh.mesh_id,
                transform: transform.to_mat4(),
            });
        }
    }
}
