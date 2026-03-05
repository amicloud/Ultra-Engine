use std::ffi::OsStr;
use std::hash::Hash;
use std::sync::Arc;
use std::{collections::HashMap, sync::RwLock};

use bevy_ecs::resource::Resource;
use glow::Context;
use slotmap::SlotMap;

use crate::assets::{handles::ShaderHandle, shader::Shader};

#[derive(Default)]
pub struct ShaderStorage {
    shaders: SlotMap<ShaderHandle, Shader>,
    shader_cache: HashMap<ShaderKey, ShaderHandle>,
}

#[derive(Resource, Default, Clone)]
pub struct ShaderResource(pub Arc<RwLock<ShaderStorage>>);
impl ShaderResource {
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, ShaderStorage> {
        match self.0.read() {
            Ok(g) => g,
            Err(e) => {
                log::error!("ShaderResource read lock poisoned; recovering inner value");
                e.into_inner()
            }
        }
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, ShaderStorage> {
        match self.0.write() {
            Ok(g) => g,
            Err(e) => {
                log::error!("ShaderResource write lock poisoned; recovering inner value");
                e.into_inner()
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct ShaderKey {
    vertex_path: String,
    fragment_path: String,
}

impl ShaderStorage {
    pub fn get_or_load(
        &mut self,
        gl: &Context,
        vertex_src: &OsStr,
        fragment_src: &OsStr,
    ) -> ShaderHandle {
        let key = ShaderKey {
            vertex_path: vertex_src.to_string_lossy().into_owned(),
            fragment_path: fragment_src.to_string_lossy().into_owned(),
        };

        if let Some(handle) = self.shader_cache.get(&key) {
            return *handle;
        }

        let shader = Shader::new(gl, vertex_src, fragment_src);

        self.add_shader(shader, key)
    }

    pub fn get_shader(&self, shader_id: ShaderHandle) -> Option<&Shader> {
        self.shaders.get(shader_id)
    }

    fn add_shader(&mut self, shader: Shader, key: ShaderKey) -> ShaderHandle {
        let id = self.shaders.insert(shader);
        self.shader_cache.insert(
            ShaderKey {
                vertex_path: key.vertex_path,
                fragment_path: key.fragment_path,
            },
            id,
        );
        id
    }
}
