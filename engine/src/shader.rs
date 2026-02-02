use std::{collections::HashMap, ffi::OsStr, fs};

use crate::handles::TextureHandle;
use glow::HasContext;
use std::hash::Hash;
pub enum UniformValue {
    Float(f32),
    Vec3(glam::Vec3),
    Mat4(glam::Mat4),
    #[allow(dead_code)]
    Int(i32),
    Texture {
        handle: TextureHandle,
        unit: u32,
    },
}

pub struct Shader {
    pub program: glow::Program,
    pub uniforms: HashMap<String, glow::UniformLocation>,
}

impl Hash for Shader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash based on the program ID
        self.program.hash(state);
    }
}

impl Shader {
    pub fn new(gl: &glow::Context, vertex_src: &OsStr, fragment_src: &OsStr) -> Self {
        unsafe {
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            let vertex_shader_source = fs::read_to_string(&vertex_src.to_str().unwrap())
                .expect("Failed to read vertex shader file");
            gl.shader_source(vertex_shader, &vertex_shader_source);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                panic!(
                    "Vertex shader compilation failed: {}",
                    gl.get_shader_info_log(vertex_shader)
                );
            } else {
                println!("Vertex shader compiled successfully.");
            }

            let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            let fragment_shader_source = fs::read_to_string(&fragment_src.to_str().unwrap())
                .expect("Failed to read fragment shader file");
            gl.shader_source(fragment_shader, &fragment_shader_source);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                panic!(
                    "Fragment shader compilation failed: {}",
                    gl.get_shader_info_log(fragment_shader)
                );
            } else {
                println!("Fragment shader compiled successfully.");
            }

            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!(
                    "Program linking failed: {}",
                    gl.get_program_info_log(program)
                );
            } else {
                println!("Program linked successfully");
            }

            let mut uniforms = HashMap::new();
            let count = gl.get_program_parameter_i32(program, glow::ACTIVE_UNIFORMS);

            for i in 0..count {
                if let Some(info) = gl.get_active_uniform(program, i as u32) {
                    if let Some(loc) = gl.get_uniform_location(program, &info.name) {
                        println!("Found uniform: {}", info.name);
                        uniforms.insert(info.name, loc);
                    }
                }
            }

            let shader = Shader { program, uniforms };

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            shader
        }
    }
}
