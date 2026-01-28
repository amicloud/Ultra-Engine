use bevy_ecs::component::Component;

use crate::handles::CameraHandle;

#[derive(Component, Debug, Copy, Clone)]
pub struct CameraComponent {
    pub camera: CameraHandle,
}