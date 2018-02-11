use std::path::Path;

use gl;
use gl::types::{GLint, GLuint, GLenum, GLvoid};
use image;
use image::GenericImage;

pub struct Texture2d {
  id: GLuint,
  width: usize,
  height: usize
}

pub struct Config {
  pub min_filter: GLenum,
  pub mag_filter: GLenum,
  pub wrap_s: GLenum,
  pub wrap_t: GLenum,
  pub max_level: GLint
}

impl Texture2d {
  pub fn new(config: &Config, width: usize, height: usize) -> Texture2d {
    let mut texture = Texture2d {id: 0, width: width, height: height};
    unsafe {
      gl::GenTextures(1, &mut texture.id);
      gl::BindTexture(gl::TEXTURE_2D, texture.id);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, config.min_filter as GLint);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, config.mag_filter as GLint);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, config.wrap_s as GLint);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, config.wrap_t as GLint);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, config.max_level);
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    texture
  }
  
  pub fn from_file(path: &Path, config: &Config) -> Texture2d {
    let dyn_img: image::DynamicImage = image::open(path).unwrap();        
    let (width, height) = dyn_img.dimensions();
    
    let texture = Texture2d::new(config, width as usize, height as usize);
    
    let format: GLenum;
    let buffer: Vec<u8> = match dyn_img {
      image::ImageLuma8(_) | image::ImageRgb8(_) => {
        format = gl::RGB;
        dyn_img.to_rgb().into_raw()
      },
      image::ImageLumaA8(_) | image::ImageRgba8(_) => {
        format = gl::RGBA;
        dyn_img.to_rgba().into_raw()
      }
    };
    
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, texture.id);
      gl::TexImage2D(
        gl::TEXTURE_2D,     // Target
        0,                  // Mipmap level.
        format as GLint,    // Internal format, e.g. gl::RGBA.
        width as GLint,     // Width.
        height as GLint,    // Height.
        0,                  // Border.
        format,             // Input format, e.g. gl::RGBA.
        gl::UNSIGNED_BYTE,  // Input data type.
        buffer.as_ptr() as *const GLvoid
      );
      gl::GenerateMipmap(gl::TEXTURE_2D);
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    
    texture
  }
  
  // Sets the active texture, binds to GL_TEXTURE_2D, and binds to the uniform.
  // Pass texture_slot is a number in the range 0-15.
  pub fn bind(&self, uniform_idx: GLint, texture_slot: u8) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0 + texture_slot as GLenum);
      gl::BindTexture(gl::TEXTURE_2D, self.id);
      gl::Uniform1i(uniform_idx, texture_slot as GLint);
    }
  }
  
  pub fn upload<T>(
    &mut self,
    level: GLint,
    internal_format: GLenum,
    input_format: GLenum,
    input_type: GLenum,
    buffer: &Vec<T>,
    generate_mipmaps: bool
  ) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, self.id);
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0, // Mipmap level.
        internal_format as GLint,
        self.width as GLint,
        self.height as GLint,
        0, // Border.
        input_format,
        input_type,
        buffer.as_ptr() as *const GLvoid
      );
  
      if generate_mipmaps {
        gl::GenerateMipmap(gl::TEXTURE_2D);
      }
  
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
  }
}

impl Drop for Texture2d {
  fn drop(&mut self) {
    unsafe { gl::DeleteTextures(1, &self.id); }
  }
}

impl Default for Config {
  fn default() -> Config {
    Config {
      min_filter: gl::LINEAR_MIPMAP_LINEAR, mag_filter: gl::LINEAR,
      wrap_s: gl::REPEAT, wrap_t: gl::REPEAT, max_level: 4
    }
  }
}