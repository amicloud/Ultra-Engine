use std::collections::HashMap;
use std::ffi::OsStr;
use std::hash::Hash;

use glow::Context;
use slotmap::SlotMap;

use crate::assets::{handles::ShaderHandle, shader::Shader};

#[derive(Default)]
pub struct ShaderResource {
    shaders: SlotMap<ShaderHandle, Shader>,
    shader_cache: HashMap<ShaderKey, ShaderHandle>,
}

#[derive(Hash, PartialEq, Eq)]
struct ShaderKey {
    vertex_path: String,
    fragment_path: String,
}

impl ShaderResource {
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
