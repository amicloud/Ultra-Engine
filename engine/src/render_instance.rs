use crate::handles::*;
use glam::Mat4;

#[derive(Clone, Debug)]
pub struct RenderInstance {
    pub mesh_id: MeshHandle,
    pub transform: Mat4,
    pub material_id: MaterialHandle,
}
