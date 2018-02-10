use std::path::Path;

use gl;
use gl::types::*;

use opengl;

pub struct Program {
    pub p:                 opengl::Program,
    
    // Uniform locations.
    pub viewport_size_idx: GLint,
    pub sprite_idx:        GLint,
    
    // Attribute locations.
    pub position_idx:      GLuint,
    pub uv_idx:            GLuint
}

impl Program {
    pub fn new() -> Program {
        let mut program = Program {
            p: opengl::Program::new(
                &Path::new("glsl/ui.vert.glsl"),
                &Path::new("glsl/ui.frag.glsl")
            ),
            viewport_size_idx: 0, sprite_idx: 0, position_idx: 0, uv_idx: 0
        };
        program.configure_indices();
        program
    }
    
    pub fn bind_textures(&self, texture_id: GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::Uniform1i(self.sprite_idx, 0);
        }
    }
    
    fn configure_indices(&mut self) {
        self.viewport_size_idx = self.p.get_uniform_location("viewportSize");
        self.sprite_idx        = self.p.get_uniform_location("sprite");
        self.position_idx      = self.p.get_attrib_location( "position");
        self.uv_idx            = self.p.get_attrib_location( "uv");
    }
}