use gl;
use gl::types::*;
use texture::Texture;

pub struct TerrainProgram {
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

impl TerrainProgram {
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