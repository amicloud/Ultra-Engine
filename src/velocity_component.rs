use bevy_ecs::component::Component;
use nalgebra::{Quaternion, Vector3};

#[derive(Default, Component)]
pub struct VelocityComponent {
    pub translational: Vector3<f64>,
    pub angular: Vector3<f64>,
}
