use std::sync::{Arc, RwLock};

use bevy_ecs::prelude::*;
use slotmap::SlotMap;

use crate::assets::{handles::MaterialHandle, material::Material};

#[derive(Default)]
pub struct MaterialStorage {
    pub materials: SlotMap<MaterialHandle, Material>,
}

#[derive(Resource, Default, Clone)]
pub struct MaterialResource(pub Arc<RwLock<MaterialStorage>>);
impl MaterialResource {
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, MaterialStorage> {
        match self.0.read() {
            Ok(g) => g,
            Err(e) => {
                log::error!("MaterialResource read lock poisoned; recovering inner value");
                e.into_inner()
            }
        }
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, MaterialStorage> {
        match self.0.write() {
            Ok(g) => g,
            Err(e) => {
                log::error!("MaterialResource write lock poisoned; recovering inner value");
                e.into_inner()
            }
        }
    }
}

impl MaterialStorage {
    pub fn add_material(&mut self, material: Material) -> MaterialHandle {
        self.materials.insert(material)
    }

    pub fn get_material(&self, material_id: MaterialHandle) -> Option<&Material> {
        self.materials.get(material_id)
    }

    #[allow(dead_code)]
    pub fn remove_material(&mut self, material_id: MaterialHandle) {
        self.materials.remove(material_id);
    }
}
