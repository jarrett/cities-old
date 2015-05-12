use std::default::Default;
use std::path::Path;
use gl;
use gl::types::*;

use glutil;
use texture::Texture;

pub struct Program {
    pub id:             GLuint,
    
    // Uniform locations.
    pub camera_idx:     GLint,
    pub foam_idx:       GLint,
    
    // Attribute locations.
    pub position_idx:   GLuint,
    pub depth_idx:      GLuint,
    
    // Textures.
    pub foam_tex:       Texture
}

impl Program {
    pub fn new() -> Program {
        let id = glutil::make_program(&Path::new("glsl/water.vert.glsl"), &Path::new("glsl/water.frag.glsl"));
        Program {
            id:             id,
            camera_idx:     glutil::get_uniform_location(id, "camera"),
            
            foam_idx:       glutil::get_uniform_location(id, "foam"),
            
            position_idx:   glutil::get_attrib_location( id, "position"),
            depth_idx:      glutil::get_attrib_location( id, "depth"),
            
            foam_tex:       Texture::new(&Path::new("assets/textures/foam.jpg"), &Default::default())
        }
    }
    
    pub fn bind_textures(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.foam_tex.id);
            gl::Uniform1i(self.foam_idx, 0);
        }
    }
}