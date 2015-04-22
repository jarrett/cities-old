use std::path::{Path, PathBuf};
use std::fs;
use std::iter;
use std::collections::HashMap;
use std::rc::Rc;
use gl;
use gl::types::*;
use image;
use image::{GenericImage, DynamicImage, RgbaImage};
use libc::{c_void};
use cgmath::{Vector, Vector2};

use super::{WidthHeight, Packed, pack_some, sort_for_packing, Config};
use futil::IoErrorLine;

pub struct Spritesheet {
    pub width: u32,
    pub height: u32,
    pub by_name: HashMap<String, Rc<Sprite>>,
    pub texture_ids: Vec<GLuint>
}

pub struct Sprite {
    pub texture_id: GLuint,
    // Offset within the sprite sheet, in the interval (0, 1).
    pub offset: Vector2<GLfloat>,
    // The sprite sheet and the individual sprite have different UV spaces. This scale
    // factor converts between them.
    pub uv_scale: Vector2<GLfloat>
}

impl Sprite {
    // The sprite sheet and the individual sprite have different UV spaces. This method
    // converts from sprite to sheet space.
    pub fn in_sheet_space(&self, sprite_space: &Vector2<GLfloat>) -> Vector2<GLfloat> {
        sprite_space.mul_v(&self.uv_scale).add_v(&self.offset)
    }
}

struct ImageWrapper {
    inner: image::DynamicImage,
    path: PathBuf
}

impl ImageWrapper {
    fn into_inner(self) -> image::DynamicImage { self.inner }
}

impl WidthHeight for ImageWrapper {
    fn width(&self)  -> u32 { self.inner.dimensions().0 }
    fn height(&self) -> u32 { self.inner.dimensions().1 }
}

impl Spritesheet {
    pub fn new(width: u32, height: u32, paths: &Vec<PathBuf>, config: &Config) -> Spritesheet {
        let mut sheet = Spritesheet {
            width: width, height: height, by_name: HashMap::new(), texture_ids: Vec::new()
        };
        
        let mut images_to_pack: Vec<ImageWrapper> = paths.iter().map(|path: &PathBuf| {
            let image: DynamicImage = image::open(&path).unwrap();
            ImageWrapper { inner: image, path: path.clone() }
        }).collect();
        
        sort_for_packing(&mut images_to_pack);
        
        while !images_to_pack.is_empty() {
            sheet.pack_one_texture(width, height, config, &mut images_to_pack);
        }
        
        sheet
    }
    
    pub fn load_dir(width: u32, height: u32, path: &Path, config: &Config) -> Result<Spritesheet, IoErrorLine> {
        let mut image_paths: Vec<PathBuf> = Vec::new();
        let walk = tryln!(fs::walk_dir(path));
        for entry in walk {
            let path: &PathBuf = &entry.unwrap().path();
            match path.extension() {
                Some(os_str) if os_str == "png" => {
                    image_paths.push(path.clone());
                },
                _ => ()
            }
        }
        Ok(Spritesheet::new(width, height, &image_paths, config))
    }
    
    // Takes a list of images left to pack. Creates a new OpenGL texture and pushes its
    // ID onto texture_ids. Pops images_to_pack one-by-one until either the vector is
    // empty or the OpenGL texture can't fit any more sprites.
    // 
    // images_to_pack should have been sorted with sort_for_packing prior to calling this.
    pub fn pack_one_texture(&mut self, width: u32, height: u32, config: &Config, mut images_to_pack: &mut Vec<ImageWrapper>) {
        let current_texture_id = Spritesheet::new_texture(config, &mut self.texture_ids);
        
        let mut packed_images = pack_some(width, width, &mut images_to_pack);
    
        // RGBA requires four bytes per pixel.
        let mut buffer: Vec<u8> = iter::repeat(255).take((width * height * 4) as usize).collect();
        
        for packed in packed_images.drain() {
            let (min_x, min_y): (u32, u32)            = (packed.min_x, packed.min_y);
            let wrapper:        ImageWrapper          = packed.into_inner();
            let name:           String                = String::from_str(wrapper.path.file_stem().unwrap().to_str().unwrap());
            let img:            RgbaImage             = wrapper.into_inner().to_rgba();
            let img_w:          u32                   = img.width();
            let img_h:          u32                   = img.height();
            let img_raw:        Vec<u8>               = img.into_raw();
        
            // Copy the image into the buffer at its appropriate position.
            // This is done row-by-row. rel_ coordinates are relative to the image's
            // upper-left corner. abs_ coordinates are relative to the packed buffer's
            // upper-left corner.
            for rel_y in 0..img_h {
                let abs_y = rel_y + min_y;
                // The offset into the buffer for the first pixel in this row.
                let row_offset = (abs_y * width + min_x) * 4;
                for rel_x in 0..img_w {
                    let buffer_idx = (row_offset + rel_x * 4) as usize;
                    let img_idx    = ((rel_y * img_w + rel_x) * 4) as usize;
                    buffer[buffer_idx + 0] = img_raw[img_idx + 0];
                    buffer[buffer_idx + 1] = img_raw[img_idx + 1];
                    buffer[buffer_idx + 2] = img_raw[img_idx + 2];
                    buffer[buffer_idx + 3] = img_raw[img_idx + 3];
                }
            }
        
            self.by_name.insert(
                name,
                Rc::new(Sprite {
                    texture_id: current_texture_id,
                    offset: Vector2::new(
                        min_x as GLfloat / width as GLfloat,
                        min_y as GLfloat / height as GLfloat,
                    ),
                    uv_scale: Vector2::new(
                        img_w as GLfloat / width as GLfloat,
                        img_h as GLfloat / height as GLfloat
                    )
                })
            );
        }
    
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, current_texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,     // Target
                0,                  // Mipmap level.
                gl::RGBA as GLint,  // Internal format.
                width as GLint,     // Width.
                height as GLint,    // Height.
                0,                  // Border.
                gl::RGBA,           // Input format.
                gl::UNSIGNED_BYTE,  // Input data type.
                buffer.as_ptr() as *const c_void
            );
            
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
    
    pub fn new_texture(config: &Config, texture_ids: &mut Vec<GLuint>) -> GLuint {
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, config.min_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, config.mag_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, config.wrap_s as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, config.wrap_t as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, config.max_level);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        texture_ids.push(texture_id);
        texture_id
    }
    
    pub fn format_all(&self) -> String {
        let keys: Vec<String> = self.by_name.keys().cloned().collect();
        keys.connect(", ")
    }
}

impl Drop for Spritesheet {
    fn drop(&mut self) {
        for mut id in self.texture_ids.drain() {
            unsafe {
                gl::DeleteTextures(1, &mut id);
            }
        }
    }
}