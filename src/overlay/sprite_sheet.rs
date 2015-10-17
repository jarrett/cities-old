use std::iter::repeat;
use std::path::{Path, PathBuf};
use std::fs;
use gl;
use gl::types::*;
use image;
use image::RgbaImage;

use futil::{IoErrorLine, walk_ext};
use texture::Config;

// The width and height of an overlay sprite, measured in pixels.
const SPRITE_PIXELS: usize = 64;

const SHEET_SPRITES: usize = 16;

const SHEET_PIXELS: usize = SPRITE_PIXELS * SHEET_SPRITES;

pub struct SpriteSheet {
    pub texture_id: GLuint
}

impl SpriteSheet {
    pub fn new(paths: &Vec<PathBuf>, config: &Config) -> SpriteSheet {
        // RGBA requires four bytes per pixel.
        let mut buffer: Vec<u8> = repeat(255).take(SHEET_PIXELS * SHEET_PIXELS * 4).collect();
        
        for (i, path) in paths.iter().enumerate() {
            // Ignore any images that fail to open.
            if let Ok(dyn_img) = image::open(&path) {
                let img:      RgbaImage = dyn_img.to_rgba();
                // Ignore any images with the wrong size.
                if img.width() as usize == SPRITE_PIXELS && img.height() as usize == SPRITE_PIXELS {
                    // Min X and Y coordinates in the overall sprite sheet. The sprite's
                    // upper-left will be aligned to this point.                  
                    let min_x:    usize     = (i % SHEET_SPRITES) * SPRITE_PIXELS;
                    let min_y:    usize     = (i / SHEET_SPRITES) * SPRITE_PIXELS;
                
                    let img_w:    usize     = img.width() as usize;
                    let img_h:    usize     = img.height() as usize;
                    let img_raw:  Vec<u8>   = img.into_raw();
            
                    // Copy the image into the buffer at its appropriate position.
                    // This is done row-by-row. rel_ coordinates are relative to the image's
                    // upper-left corner. abs_ coordinates are relative to the packed buffer's
                    // upper-left corner.
                    for rel_y in 0..img_h {
                        let abs_y = rel_y + min_y;
                        // The offset into the buffer for the first pixel in this row.
                        let row_offset = (abs_y * SHEET_PIXELS + min_x) * 4;
                        for rel_x in 0..img_w {
                            let buffer_idx = row_offset + rel_x * 4;
                            let img_idx    = (rel_y * img_w + rel_x) * 4;
                            buffer[buffer_idx + 0] = img_raw[img_idx + 0];
                            buffer[buffer_idx + 1] = img_raw[img_idx + 1];
                            buffer[buffer_idx + 2] = img_raw[img_idx + 2];
                            buffer[buffer_idx + 3] = img_raw[img_idx + 3];
                        }
                    }
                }
            }
        }
        
        let mut sheet = SpriteSheet {texture_id: 0};
        
        unsafe {
            gl::GenTextures(1, &mut sheet.texture_id);
            gl::BindTexture(gl::TEXTURE_2D, sheet.texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, config.min_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, config.mag_filter as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, config.wrap_s as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, config.wrap_t as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, config.max_level);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        
        sheet
    }
    
    pub fn load_dir(path: &Path, config: &Config) -> Result<SpriteSheet, IoErrorLine> {
        let image_paths = try!(walk_ext(path, "png"));
        Ok(SpriteSheet::new(&image_paths, config))
    }
}