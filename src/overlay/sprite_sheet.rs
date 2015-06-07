use std::path::{Path, PathBuf};
use std::fs;
use gl::types::*;

use futil::{IoErrorLine, walk_ext};

pub struct SpriteSheet {
    pub width: usize,
    pub height: usize,
    pub texture_id: GLuint
}

impl SpriteSheet {
    pub fn new(width: usize, height: usize, paths: &Vec<PathBuf>, config: &Config) -> SpriteSheet {        
        let mut sheet = SpriteSheet {
            width: width, height: height, texture_id: 0
        };
        
        unsafe {
            gl::GenTextures(1, &mut sheet.texture_id);
        }
        
        sheet
    }
    
    pub fn load_dir(width: usize, height: usize, path: &Path, config: &Config) -> Result<SpriteSheet, IoErrorLine> {
        let image_paths = try!(walk_ext(path, "png"));
        Ok(SpriteSheet::new(width, height, &image_paths, config))
    }
}