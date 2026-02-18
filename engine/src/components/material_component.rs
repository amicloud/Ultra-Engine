use bevy_ecs::prelude::*;

use crate::assets::handles::MaterialHandle;

#[derive(Component, Debug, Clone, Copy)]
pub struct MaterialComponent {
    pub material_id: MaterialHandle,
}
