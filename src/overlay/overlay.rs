/* Overlay stores the overlay textures for each square. Each chunk owns exactly one
Overlay. Each Overlay owns a 3D texture. Each (x, y) coordinate in the texture maps to a
square in the Chunk.

The internal format of the textures is GL_RG8UI. Thus, each pixel is two 8-bit
unsigned ints.

The value of a pixel is interpreted as follows. (R, G) represent the (X, Y) offset into
the overlay sprite-sheet. The pair (255, 255) represents the absence of a sprite.

The depth (Z size) of the 3D texture is 2. We'll refer to the two Z slices as  "layer 0"
and "layer 1." Layer 0 meant for base textures such as grass, asphalt, etc. Layer 1 is
meant for stencil-like textures, such as stripes to be painted on asphalt. The fragment
shader blends the ground texture, layer 0 if any, and layer 1 if any, in that order. */

use std::mem;
use std::iter::repeat;
use gl;
use gl::types::*;
use libc::c_void;

pub struct Overlay {
    x_size: usize,
    y_size: usize,
    map_id: GLuint
}

pub enum Layer {
    Zero,
    One
}

impl Overlay {
    pub fn new(x_size: usize, y_size: usize) -> Overlay {
        let mut overlay = Overlay {
            x_size: x_size, y_size: y_size,
            map_id: 0
        };
        unsafe {
            gl::GenTextures(1, &mut overlay.map_id);
        }
        overlay.init_buffer();
        overlay
    }
    
    pub fn init_buffer(&mut self) {
        // Initialize map_data, filling it with 255. (255, 255) represents the absence
        // of an overlay texture.
        let map_data: Vec<u8> = repeat(255).take(self.x_size * self.y_size * 2 * 4).collect();
        unsafe {
            gl::TexImage3D(
                gl::TEXTURE_3D,       // Target.
                0,                    // Mip-map level.
                gl::RG8UI as GLint,   // Internal format.
                self.x_size as GLint, // Width.
                self.y_size as GLint, // Height.
                2,                    // Depth.
                0,                    // Border.
                gl::RG,               // Source format.
                gl::UNSIGNED_BYTE,    // Source data type.
                map_data.as_ptr() as *const c_void
            );
        }
    }
    
    // Sets the overlay texture at (chunk_x, chunk_y) to the sprite designated by
    // (sprite_x, sprite_y). The latter coordinates are in sprite-sheet space.
    pub fn set(&mut self, chunk_x: usize, chunk_y: usize, layer: Layer, sprite_x: u8, sprite_y: u8) {        
        //let pixel: Vec<u8> = vec![sprite_x, sprite_y];
        let pixel: [u8; 2] = [sprite_x, sprite_y];
        
        unsafe {
            gl::TexSubImage3D(
                gl::TEXTURE_3D,       // Target.
                0,                    // Mip-map level.
                chunk_x as GLint,     // X offset.
                chunk_y as GLint,     // Y offset.
                match layer {         // Z offset.
                    Layer::Zero => 0,
                    Layer::One => 1
                },                  
                1,                    // Width (X).
                1,                    // Height (Y).
                1,                    // Depth (Z).
                gl::RG,               // Source format.
                gl::UNSIGNED_BYTE,    // Source data type.
                mem::transmute(&pixel)
            );
        }
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.map_id);
        }
    }
}