use bevy_ecs::prelude::*;

use crate::{handles::CameraHandle, camera::Camera};

#[derive(Default, Resource)]
pub struct CameraResourceManager {
    pub cameras: std::collections::HashMap<CameraHandle, Camera>,
}

impl CameraResourceManager {
    pub fn add_camera(&mut self, camera: Camera) -> CameraHandle {
        let id = camera.id;
        self.cameras.insert(id, camera);
        id
    }

    pub fn get_camera(&self, camera_id: CameraHandle) -> Option<&Camera> {
        self.cameras.get(&camera_id)
    }

    pub fn get_camera_mut(&mut self, camera_id: CameraHandle) -> Option<&mut Camera> {
        self.cameras.get_mut(&camera_id)
    }

    #[allow(dead_code)]
    pub fn remove_camera(&mut self, camera_id: CameraHandle) {
        self.cameras.remove(&camera_id);
    }
}


