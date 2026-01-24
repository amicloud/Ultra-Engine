use nalgebra::Matrix4;
pub struct RenderInstance {
    pub mesh_id: u32,
    pub transform: Matrix4<f32>,
}
