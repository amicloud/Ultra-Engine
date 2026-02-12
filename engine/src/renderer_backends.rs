pub trait GraphicsBackend {
    type Buffer;
    type Program;
    type VertexArray;

    fn create_buffer(&mut self) -> Self::Buffer;

    fn bind_array_buffer(&mut self, buffer: Option<Self::Buffer>);
    fn upload_buffer_data(&mut self, data: &[u8], dynamic: bool);
    fn upload_buffer_sub_data(&mut self, offset: i32, data: &[u8]);

    fn use_program(&mut self, program: Option<Self::Program>);
    fn bind_vertex_array(&mut self, vao: Option<Self::VertexArray>);

    fn draw_elements_instanced(
        &mut self,
        count: i32,
        instance_count: i32,
    );
}
pub struct NullBackend {
    frames_rendered: u64,
}

pub struct GlowBackend {
    gl: Rc<GlowContext>,
    frames_rendered: u64,
    vao_cache: HashMap<VaoKey, glow::VertexArray>,
}

impl GraphicsBackend for NullBackend {
    type Buffer = ();
    type Program = ();
    type VertexArray = ();

    fn create_buffer(&mut self) -> Self::Buffer {}

    fn bind_array_buffer(&mut self, _buffer: Option<Self::Buffer>) {}
    fn upload_buffer_data(&mut self, _data: &[u8], _dynamic: bool) {}
    fn upload_buffer_sub_data(&mut self, _offset: i32, _data: &[u8]) {}

    fn use_program(&mut self, _program: Option<Self::Program>) {}
    fn bind_vertex_array(&mut self, _vao: Option<Self::VertexArray>) {}

    fn draw_elements_instanced(
        &mut self,
        _count: i32,
        _instance_count: i32,
    ) {
        self.frames_rendered += 1;
    }
}

impl GraphicsBackend for GlowBackend {
    type Buffer = glow::Buffer;
    type Program = glow::Program;
    type VertexArray = glow::VertexArray;

    fn create_buffer(&mut self) -> Self::Buffer {
        unsafe { self.gl.create_buffer().unwrap() }
    }

    fn bind_array_buffer(&mut self, buffer: Option<Self::Buffer>) {
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, buffer);
        }
    }

    fn upload_buffer_data(&mut self, data: &[u8], dynamic: bool) {
        unsafe {
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                data,
                if dynamic {
                    glow::DYNAMIC_DRAW
                } else {
                    glow::STATIC_DRAW
                },
            );
        }
    }

    fn upload_buffer_sub_data(&mut self, offset: i32, data: &[u8]) {
        unsafe {
            self.gl
                .buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, offset, data);
        }
    }

    fn use_program(&mut self, program: Option<Self::Program>) {
        unsafe { self.gl.use_program(program) }
    }

    fn bind_vertex_array(&mut self, vao: Option<Self::VertexArray>) {
        unsafe { self.gl.bind_vertex_array(vao) }
    }

    fn draw_elements_instanced(
        &mut self,
        count: i32,
        instance_count: i32,
    ) {
        unsafe {
            self.gl.draw_elements_instanced(
                glow::TRIANGLES,
                count,
                glow::UNSIGNED_INT,
                0,
                instance_count,
            );
        }
        self.frames_rendered += 1;
    }
}