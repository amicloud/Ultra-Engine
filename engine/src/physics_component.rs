use bevy_ecs::component::Component;

#[derive(Clone, Copy)]
pub enum PhysicsType {
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Clone, Copy, Component)]
pub struct PhysicsComponent {
    pub physics_type: PhysicsType,
    pub mass: f32,
}
