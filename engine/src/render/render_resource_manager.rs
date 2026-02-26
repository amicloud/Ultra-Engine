use bevy_ecs::prelude::*;

use crate::{
    assets::{
        material_resource::MaterialResource, mesh_resource::MeshResource,
        shader_resource::ShaderResource, texture_resource::TextureResource,
    },
    render::render_body_resource::RenderBodyResource,
};

#[derive(Resource)]
pub struct RenderResourceManager {
    pub mesh_resource: MeshResource,
    pub render_body_resource: RenderBodyResource,
    pub material_resource: MaterialResource,
    pub texture_resource: TextureResource,
    pub shader_resource: ShaderResource,
}

impl Default for RenderResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderResourceManager {
    pub fn new() -> Self {
        Self {
            mesh_resource: MeshResource::default(),
            render_body_resource: RenderBodyResource::default(),
            material_resource: MaterialResource::default(),
            texture_resource: TextureResource::default(),
            shader_resource: ShaderResource::default(),
        }
    }
}
