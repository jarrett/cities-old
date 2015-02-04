use gl;
use gl::types::*;
use texture::Texture;

pub struct WaterProgram {
    pub id:             GLuint,
  
    // Uniform locations.
    pub model_view_idx: GLint,
    pub projection_idx: GLint,
    pub foam_idx:       GLint,
  
    // Attribute locations.
    pub position_idx:   GLuint,
    pub depth_idx:      GLuint,
  
    // Textures.
    pub foam_tex:       Texture
}

impl WaterProgram {
    pub fn bind_textures(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.foam_tex.id);
            gl::Uniform1i(self.foam_idx, 0);
        }
    }
}