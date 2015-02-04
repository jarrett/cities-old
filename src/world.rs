use num::integer::Integer;
use cgmath::*;

use chunk::Chunk;

pub struct World { 
    pub x_verts: u32,       // X dimension of world.
    pub y_verts: u32,       // Y dimension of world.
    pub x_size: u32,        // X dimension of world. Equal to xVerts - 1.
    pub y_size: u32,        // X dimension of world. Equal to xVerts - 1.
    pub x_chunks: u32,      // Number of chunks along the X axis.
    pub y_chunks: u32,      // Number of chunks along the Y axis.
    pub chunk_x_verts: u32, // X vertices of each chunk.
    pub chunk_y_verts: u32, // Y vertices of each chunk.
    pub chunk_x_size: u32,  // X dimension of each chunk. Equal to chunkXVerts - 1;
    pub chunk_y_size: u32,  // Y dimension of each chunk. Equal to chunkYVerts - 1;
    pub chunks: Vec<Vec<Chunk>>
}

impl World {
    // Up to four chunks may contain a point, because some points are on the edges or
    // corners of a chunk. This method returns the one with the highest x, y. Or, if no
    // chunk contains the point, it returns None. We require integer input because with
    // floating-point values, making correct decisions at the edges of chunks could become
    // problematic. If you need to find the chunk containing a floating-point coord, round
    // it yourself in whatever way is logical.
    pub fn chunk_containing(&self, abs_x: u32, abs_y: u32) -> Option<&Chunk> {
        // x_idx and y_idx are indices int the chunks array.
        let x_idx: usize = abs_x.div_floor(&self.chunk_x_size) as usize;
        let y_idx: usize = abs_y.div_floor(&self.chunk_y_size) as usize;
        
        if ((x_idx as u32) < self.x_chunks && (y_idx as u32) < self.y_chunks) {
            Some(&self.chunks[y_idx][x_idx])
        } else {
            None
        }
    }

    pub fn vert_position_at(&self, abs_x: i32, abs_y: i32) -> Option<Vector3<f32>> {
        if abs_x >= 0 && abs_y >= 0 {
            match self.chunk_containing(abs_x as u32, abs_y as u32) {
                Some(chunk) => {
                    chunk.vert_position_at(abs_x as u32, abs_y as u32)
                },
                None => None
            }
        } else {
            None
        }
    }
}