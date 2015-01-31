extern crate gl;
extern crate image;
extern crate libc;

use std::io::File;
use self::gl::types::*;
use self::image::*;
use self::libc::{c_void};

pub struct TextureConfig {
    min_filter: GLenum,
    mag_filter: GLenum,
    wrap_s: GLenum,
    wrap_t: GLenum,
    max_level: GLint
}

pub struct Texture {    
    id: GLuint,
    width: u32,
    height: u32
}

impl Texture {
    pub fn new(path: &Path, config: TextureConfig) -> Texture {
        let mut tex = Texture {id: 0, width: 0, height: 0};
        let id_ptr: *mut GLuint = &mut tex.id;
        unsafe {
            gl::GenTextures(1, id_ptr);
            gl::BindTexture(gl::TEXTURE_2D, tex.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, config.min_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, config.mag_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, config.wrap_s as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, config.wrap_t as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, config.max_level);
        }
        
        let dyn_img: DynamicImage = image::open(path).unwrap();
        
        let (width, height) = dyn_img.dimensions();
        tex.width = width;
        tex.height = height;
        
        let format: GLenum;
        let buffer: Vec<u8> = match dyn_img {
            ImageLuma8(_) | ImageRgb8(_) => {
                format = gl::RGB;
                dyn_img.to_rgb().into_raw()
            },
            ImageLumaA8(_) | ImageRgba8(_) => {
                format = gl::RGBA;
                dyn_img.to_rgba().into_raw()
            }
        };
        
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D, 0,
                format as GLint,
                width as GLint,
                height as GLint,
                0,
                format,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const c_void
            );
            
            gl::GenerateMipmap(gl::TEXTURE_2D);
            
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        
        tex
    }
    
    pub fn default_config() -> TextureConfig {
        TextureConfig {
            min_filter: gl::LINEAR_MIPMAP_LINEAR, mag_filter: gl::LINEAR,
            wrap_s: gl::REPEAT, wrap_t: gl::REPEAT, max_level: 4
        }
    }
}