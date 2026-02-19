#[derive(Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub gl_tex: Option<glow::Texture>, // GPU handle
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            gl_tex: None,
        }
    }
}
