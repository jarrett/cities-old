use std::path::Path;

use gl;
use gl::types::*;

use glutil;

pub struct Program {
    pub id:             GLuint,
    
    // Uniform locations.
    pub sprite_idx:     GLint,
    
    // Attribute locations.
    pub position_idx:   GLuint,
    pub uv_idx:         GLuint
}

impl Program {
    pub fn new() -> Program {
        let id = glutil::make_program(
            &Path::new("glsl/hud.vert.glsl"),
            &Path::new("glsl/hud.frag.glsl")
        );
        
        Program {
            id:           id,
            sprite_idx:   glutil::get_uniform_location(id, "sprite"),
            position_idx: glutil::get_attrib_location( id, "position"),
            uv_idx:       glutil::get_attrib_location( id, "uv"),
        }
    }
    
    pub fn bind_textures(&self, texture_id: GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::Uniform1i(self.sprite_idx, 0);
        }
    }
}