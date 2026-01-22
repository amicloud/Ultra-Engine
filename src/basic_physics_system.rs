use crate::transform_component::TransformComponent;
use crate::velocity_component::VelocityComponent;
use bevy_ecs::prelude::*;
use nalgebra::{Quaternion, UnitQuaternion, Vector3};
pub struct BasicPhysicsSystem {}

impl BasicPhysicsSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(mut query: Query<(&mut TransformComponent, &VelocityComponent)>) {
        let delta_time = 1.0 / 60.0; // Assuming a fixed time step of 1/60 seconds
        for (mut transform, velocity) in query.iter_mut() {
            // Update position based on translational velocity
            transform.position += velocity.translational * delta_time;

            // Update rotation based on angular velocity
            let angular_velocity_magnitude = velocity.angular.norm();
            if angular_velocity_magnitude > 0.0 {
                // Rotation is store as a vec3 representing rotation around each axis
                let axis = velocity.angular / angular_velocity_magnitude;
                let angle = angular_velocity_magnitude * delta_time;
                let delta_rotation =
                    UnitQuaternion::from_axis_angle(&nalgebra::Unit::new_normalize(axis), angle);
                let current_rotation = UnitQuaternion::from_quaternion(transform.rotation);
                let new_rotation = delta_rotation * current_rotation;
                transform.rotation = new_rotation.into_inner();
            }
            println!["Transform: position={:?}, rotation={:?}", transform.position, transform.rotation];
        }
    }
}
