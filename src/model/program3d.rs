use std::default::Default;
use std::path::Path;
use gl;
use gl::types::*;

use texture;
use texture::Texture;

use glutil;

pub struct Program3d {
    pub id:             GLuint,
    
    // Uniform locations.
    pub camera_idx:     GLint,
    pub orbit_idx:      GLint,
    pub direction_idx:  GLint,
    pub origin_idx:     GLint,
    pub sprite_idx:     GLint,
    
    // Attribute locations.
    pub position_idx:   GLuint,
    pub uv_idx:         GLuint,
    
    pub test_tex:      Texture
}

impl Program3d {
    pub fn new() -> Program3d {
        let id = glutil::make_program(&Path::new("glsl/model3d.vert.glsl"), &Path::new("glsl/model.frag.glsl"));
        
        let tex_cfg: texture::Config = Default::default();
        
        Program3d {
            id:             id,
            
            camera_idx:     glutil::get_uniform_location(id, "camera"),
            orbit_idx:      glutil::get_uniform_location(id, "orbit"),
            direction_idx:  glutil::get_uniform_location(id, "direction"),
            origin_idx:     glutil::get_uniform_location(id, "origin"),
            sprite_idx:     glutil::get_uniform_location(id, "sprite"),
            
            position_idx:   glutil::get_attrib_location( id, "position"),
            uv_idx:         glutil::get_attrib_location( id, "uv"),
            
            test_tex:       Texture::new(&Path::new("assets/textures/cliff.jpg"), &tex_cfg)
        }
    }
    
    pub fn bind_textures(&self, texture_id: GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            //gl::BindTexture(gl::TEXTURE_2D, self.test_tex.id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::Uniform1i(self.sprite_idx, 0);
        }
    }
}