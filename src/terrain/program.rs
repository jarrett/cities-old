use std::default::Default;
use gl;
use gl::types::*;

use glutil;
use texture;
use texture::Texture;

pub struct Program {
    pub id:             GLuint,
    
    // Uniform locations.
    pub model_view_idx: GLint,
    pub projection_idx: GLint,
    pub underwater_idx: GLint,
    pub flat_idx:       GLint,
    pub slope_idx:      GLint,
    pub cliff_idx:      GLint,
    
    // Attribute locations.
    pub position_idx:   GLuint,
    pub normal_idx:     GLuint,
    
    // Textures.
    pub underwater_tex: Texture,
    pub flat_tex:       Texture,
    pub slope_tex:      Texture,
    pub cliff_tex:      Texture
}

impl Program {
    pub fn new() -> Program {
        let id = glutil::make_program(&Path::new("glsl/terrain.vert.glsl"), &Path::new("glsl/terrain.frag.glsl"));
        let tex_cfg: texture::Config = Default::default();
        Program {
            id:             id,
            
            model_view_idx: glutil::get_uniform_location(id, "model"),
            projection_idx: glutil::get_uniform_location(id, "projection"),
            
            underwater_idx: glutil::get_uniform_location(id, "underwater"),
            flat_idx:       glutil::get_uniform_location(id, "plain"),
            slope_idx:      glutil::get_uniform_location(id, "slope"),
            cliff_idx:      glutil::get_uniform_location(id, "cliff"),
            
            position_idx:   glutil::get_attrib_location( id, "position"),
            normal_idx:     glutil::get_attrib_location( id, "normal"),
            
            underwater_tex: Texture::new(&Path::new("assets/textures/underwater.jpg"), &tex_cfg),
            flat_tex:       Texture::new(&Path::new("assets/textures/plain.jpg"), &tex_cfg),
            slope_tex:      Texture::new(&Path::new("assets/textures/slope.jpg"), &tex_cfg),
            cliff_tex:      Texture::new(&Path::new("assets/textures/cliff.jpg"), &tex_cfg)
        }
    }
    
    pub fn bind_textures(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.underwater_tex.id);
            gl::Uniform1i(self.underwater_idx, 0);
  
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.flat_tex.id);
            gl::Uniform1i(self.flat_idx, 1);
  
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.slope_tex.id);
            gl::Uniform1i(self.slope_idx, 2);
  
            gl::ActiveTexture(gl::TEXTURE3);
            gl::BindTexture(gl::TEXTURE_2D, self.cliff_tex.id);
            gl::Uniform1i(self.cliff_idx, 3);
        }
    }
}