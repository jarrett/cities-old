use std::path::Path;
use gl;
use gl::types::*;
use image;
use image::GenericImage;
use libc::c_void;

use texture::Config;

pub struct Texture {
    pub id: GLuint,
    pub width: u32,
    pub height: u32
}

impl Texture {
    pub fn new(path: &Path, config: &Config) -> Texture {
        let mut tex = Texture {id: 0, width: 0, height: 0};
        unsafe {
            gl::GenTextures(1, &mut tex.id);
            gl::BindTexture(gl::TEXTURE_2D, tex.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, config.min_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, config.mag_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, config.wrap_s as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, config.wrap_t as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, config.max_level);
        }
        
        let dyn_img: image::DynamicImage = image::open(path).unwrap();
        
        let (width, height) = dyn_img.dimensions();
        tex.width = width;
        tex.height = height;
        
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
            gl::TexImage2D(
                gl::TEXTURE_2D,     // Target
                0,                  // Mipmap level.
                format as GLint,    // Internal format, e.g. gl::RGBA.
                width as GLint,     // Width.
                height as GLint,    // Height.
                0,                  // Border.
                format,             // Input format, e.g. gl::RGBA.
                gl::UNSIGNED_BYTE,  // Input data type.
                buffer.as_ptr() as *const c_void
            );
            
            gl::GenerateMipmap(gl::TEXTURE_2D);
            
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        
        tex
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}