// Variable-sized sprites for models. The sprites get packed into one or more
// sprite-sheets, each of which owns an OpenGL texture.

use std::path::{Path, PathBuf};
use std::iter;
use std::collections::HashMap;
use std::rc::Rc;
use gl;
use gl::types::*;
use image;
use image::{GenericImage, DynamicImage, RgbaImage};
use cgmath::{VectorSpace, Vector2};

use opengl::{Texture2d, TextureConfig};
use super::{WidthHeight, pack_some, sort_for_packing};
use futil::{IoErrorLine, walk_ext};

pub struct Sheet {
    pub width: usize,
    pub height: usize,
    pub by_name: HashMap<String, Rc<Sprite>>,
    pub textures: Vec<Texture2d>
}

pub struct Sprite {
    pub texture: Rc<Texture2d>,
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
    fn width(&self)  -> usize { self.inner.dimensions().0 as usize }
    fn height(&self) -> usize { self.inner.dimensions().1 as usize }
}

impl Sheet {
    pub fn new(width: usize, height: usize, paths: &Vec<PathBuf>, config: &TextureConfig) -> Sheet {
        let mut sheet = Sheet {
            width: width, height: height, by_name: HashMap::new(), textures: Vec::new()
        };
        
        let mut images_to_pack: Vec<ImageWrapper> = paths.iter().filter_map(|path: &PathBuf| {
            // Ignore any images that fail to open.
            if let Ok(image) = image::open(&path) {
                Some(ImageWrapper { inner: image, path: path.clone() })
            } else {
                None
            }
        }).collect();
        
        sort_for_packing(&mut images_to_pack);
        
        // Generate one or more OpenGL textures. Each iteration, we remove from
        // images_to_pack as many sprites as we can fit in a single texture.
        while !images_to_pack.is_empty() {
            sheet.pack_one_texture(width, height, config, &mut images_to_pack);
        }
        
        sheet
    }
    
    pub fn load_dir(width: usize, height: usize, path: &Path, config: &TextureConfig) -> Result<Sheet, IoErrorLine> {
        let image_paths = try!(walk_ext(path, "png"));
        Ok(Sheet::new(width, height, &image_paths, config))
    }
    
    // Takes a list of images left to pack. Creates a new OpenGL texture and pushes it
    // onto textures. Pops images_to_pack one-by-one until either the vector is empty or
    // the OpenGL texture can't fit any more sprites.
    // 
    // images_to_pack should have been sorted with sort_for_packing prior to calling this.
    pub fn pack_one_texture(
        &mut self,
        width: usize,
        height: usize,
        config: &TextureConfig,
        mut images_to_pack: &mut Vec<ImageWrapper>
    ) {
        let mut current_texture = Texture2d::new(config, width, height);
        
        let mut packed_images = pack_some(width, width, &mut images_to_pack);
    
        // RGBA requires four bytes per pixel.
        let mut buffer: Vec<u8> = iter::repeat(255).take(width * height * 4).collect();
        
        // A temporary place to store the sprite data that we generate. After the finished
        // texture has been uploaded to the GPU, we'll use sprite_data to construct
        // Sprites and insert them into self.by_name.
        let mut sprite_data: Vec<(String, Vector2<GLfloat>, Vector2<GLfloat>)> = Vec::new();
        
        for packed in packed_images.drain(..) {
            let (min_x, min_y): (usize, usize)        = (packed.min_x, packed.min_y);
            let wrapper:        ImageWrapper          = packed.into_inner();
            let name:           String                = String::from(wrapper.path.file_stem().unwrap().to_str().unwrap());
            let img:            RgbaImage             = wrapper.into_inner().to_rgba();
            let img_w:          usize                 = img.width() as usize;
            let img_h:          usize                 = img.height() as usize;
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
                    let buffer_idx = row_offset + rel_x * 4;
                    let img_idx    = (rel_y * img_w + rel_x) * 4;
                    buffer[buffer_idx + 0] = img_raw[img_idx + 0];
                    buffer[buffer_idx + 1] = img_raw[img_idx + 1];
                    buffer[buffer_idx + 2] = img_raw[img_idx + 2];
                    buffer[buffer_idx + 3] = img_raw[img_idx + 3];
                }
            }
            
            sprite_data.push((
                name,
                // Offset.
                Vector2::new(
                    min_x as GLfloat / width as GLfloat,
                    min_y as GLfloat / height as GLfloat,
                ),
                // UV scale.
                Vector2::new(
                    img_w as GLfloat / width as GLfloat,
                    img_h as GLfloat / height as GLfloat
                )
            ));
        }
        
        current_texture.upload(
            0, // Mipmap level.
            gl::RGBA, // Internal format.
            gl::RGBA, // Input format.
            gl::UNSIGNED_BYTE, // Input data type.
            &buffer,
            true // Generate mipmaps.
        );
        
        let current_texture_rc = Rc::new(current_texture);
        
        for (name, offset, uv_scale) in sprite_data.drain(..) {
            self.by_name.insert(
                name,
                Rc::new(Sprite {
                    texture: current_texture_rc.clone(),
                    offset: offset,
                    uv_scale: uv_scale
                })
            );
        }
    }
    
    pub fn format_all(&self) -> String {
        let keys: Vec<String> = self.by_name.keys().cloned().collect();
        keys.join(", ")
    }
}