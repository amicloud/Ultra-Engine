use bevy_ecs::component::Component;
use nalgebra::{Matrix4, UnitQuaternion, Vector3};

#[derive(Component, Debug, Copy, Clone)]
pub struct TransformComponent {
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::identity(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl TransformComponent {
    pub fn to_mat4(&self) -> Matrix4<f32> {
        let translation_matrix = Matrix4::new_translation(&self.position);
        let rotation_matrix = Matrix4::from(self.rotation);
        let scale_matrix = Matrix4::new_nonuniform_scaling(&self.scale);

        translation_matrix * rotation_matrix * scale_matrix
    }
}
