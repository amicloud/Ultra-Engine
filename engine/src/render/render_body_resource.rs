use std::sync::{Arc, RwLock};

use bevy_ecs::prelude::*;
use slotmap::SlotMap;

use crate::{RenderBodyHandle, render::render_body::RenderBody};

#[derive(Default)]
pub struct RenderBodyStorage {
    pub render_bodies: SlotMap<RenderBodyHandle, RenderBody>,
}

#[derive(Resource, Default, Clone)]
pub struct RenderBodyResource(pub Arc<RwLock<RenderBodyStorage>>);
impl RenderBodyResource {
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, RenderBodyStorage> {
        match self.0.read() {
            Ok(g) => g,
            Err(e) => {
                log::error!("RenderBodyResource read lock poisoned; recovering inner value");
                e.into_inner()
            }
        }
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, RenderBodyStorage> {
        match self.0.write() {
            Ok(g) => g,
            Err(e) => {
                log::error!("RenderBodyResource write lock poisoned; recovering inner value");
                e.into_inner()
            }
        }
    }
}

impl RenderBodyStorage {
    pub fn add_render_body(&mut self, render_body: RenderBody) -> RenderBodyHandle {
        self.render_bodies.insert(render_body)
    }

    pub fn get_render_body(&self, render_body_id: RenderBodyHandle) -> Option<&RenderBody> {
        self.render_bodies.get(render_body_id)
    }

    #[allow(dead_code)]
    pub fn get_render_body_mut(
        &mut self,
        render_body_id: RenderBodyHandle,
    ) -> Option<&mut RenderBody> {
        self.render_bodies.get_mut(render_body_id)
    }

    #[allow(dead_code)]
    pub fn remove_render_body(&mut self, render_body_id: RenderBodyHandle) {
        self.render_bodies.remove(render_body_id);
    }
}
