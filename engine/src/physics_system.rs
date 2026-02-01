use crate::movement_system::MovementSystem;
use crate::velocity_component::VelocityComponent;
use crate::WorldBasis;
use crate::{physics_component::PhysicsComponent, transform_component::TransformComponent};
use bevy_ecs::prelude::*;
pub struct PhysicsSystem {}

impl PhysicsSystem {
    pub fn update_bodies(
        mut query: Query<(
            &mut TransformComponent,
            &mut VelocityComponent,
            &PhysicsComponent,
        )>,
    ) {
        let delta_time = 1.0 / 60.0; // Assuming a fixed time step of 1/60 seconds
        let g = WorldBasis::gravity_vector();
        for (mut transform, mut velocity, _physics) in query.iter_mut() {
            velocity.translational += g * delta_time;
            transform.position += velocity.translational * delta_time;

            transform.rotation =
                MovementSystem::apply_rotation(&transform.rotation, &velocity.angular, delta_time);
        }
    }
}
