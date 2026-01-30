use bevy_ecs::prelude::*;

use crate::handles::RenderBodyHandle;

#[derive(Component, Debug, Copy, Clone)]
pub struct RenderBodyComponent {
    pub render_body_id: RenderBodyHandle,
}
