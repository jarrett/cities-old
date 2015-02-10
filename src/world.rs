use std::cell::RefCell;
use num::integer::Integer;
use cgmath::*;

use chunk::Chunk;
use terrain;
use water;
use camera::Camera;

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
    pub chunks: Vec<Vec<RefCell<Chunk>>>
}

impl World {
    pub fn new<T: terrain::Source>(
        terrain_source: T,
        terrain_program: &terrain::Program, water_program: &water::Program,
        chunk_x_verts: u32, chunk_y_verts: u32
      ) -> World {
        let mut world = World {
          x_verts:       terrain_source.x_verts(),
          y_verts:       terrain_source.y_verts(),
          x_size:        terrain_source.x_verts() - 1,
          y_size:        terrain_source.y_verts() - 1,
          chunk_x_verts: chunk_x_verts,
          chunk_y_verts: chunk_y_verts,
          chunk_x_size:  chunk_x_verts - 1,
          chunk_y_size:  chunk_y_verts - 1,
          x_chunks:      0,
          y_chunks:      0,
          chunks:        Vec::with_capacity(((terrain_source.y_verts() - 1) / chunk_y_verts) as usize)
        };
        
        if world.x_verts % chunk_x_verts != 0 {
          panic!("x_verts ({}) is not a multiple of chunk_x_verts ({})", world.x_verts, chunk_x_verts);
        }
        if world.y_verts % chunk_y_verts != 0 {
          panic!("y_verts ({}) is not a multiple of chunk_y_verts ({})", world.y_verts, chunk_y_verts);
        }
  
        world.x_chunks = world.x_size / chunk_x_verts;
        world.y_chunks = world.y_size / chunk_y_verts;
        
        for _ in 0u32..world.y_chunks {
          let inner_vec = Vec::with_capacity(world.x_chunks as usize);
          world.chunks.push(inner_vec);
        }
  
        // Iterate over all the chunks. The chunks aren't actually allocated yet; we're
        // about to create them in this loop. We'll set their vertex positions, but we
        // can't set the normals or buffer to the GPU yet. To calculate the normals, all
        // the positions must already exist. So we need a second pass for that.
        for chunk_y in 0u32..world.y_chunks {
            for chunk_x in 0u32..world.x_chunks {
                // This is where we finally initialize the chunks themselves.
                let min_x: u32 = chunk_x * world.chunk_x_size;
                let min_y: u32 = chunk_y * world.chunk_y_size;
                let mut chunk: Chunk = Chunk::new(terrain_program, water_program, min_x, min_y, chunk_x_verts, chunk_y_verts);
                
                // Drill down deeper: Go through the full X/Y range of each chunk and set the height
                // for each vertex.
                for rel_y in 0u32..chunk_y_verts {
                    for rel_x in 032..chunk_x_verts {
                        let x: u32 = min_x + rel_x;
                        let y: u32 = min_y + rel_y;
                        let height: f32 = terrain_source.vert_z_at(x, y);
                        chunk.set_height(x, y, height);
                    }
                }
                
                // Finally, move the chunk. The vector owns it now, and it's wrapped
                // in a RefCell.
                world.chunks[chunk_y as usize].push(RefCell::new(chunk));
            }
        }
        
        /* Now it's time for the second pass, wherein we calculate the normals and buffer
        to the GPU. */
        for inner_vec in world.chunks.iter() {
            for cell in inner_vec.iter() {
                // Should never panic. If we've
                let chunk = &mut cell.borrow_mut();
                chunk.calc_normals(&world);
                chunk.buffer_positions();
                chunk.buffer_depths();
                chunk.buffer_normals();
                chunk.buffer_indices();
            }
        }
        
        world
    }
    
    // Up to four chunks may contain a point, because some points are on the edges or
    // corners of a chunk. This method returns the one with the highest x, y. Or, if no
    // chunk contains the point, it returns None. We require integer input because with
    // floating-point values, making correct decisions at the edges of chunks could become
    // problematic. If you need to find the chunk containing a floating-point coord, round
    // it yourself in whatever way is logical.
    pub fn chunk_containing(&self, abs_x: u32, abs_y: u32) -> Option<&RefCell<Chunk>> {
        // x_idx and y_idx are indices int the chunks array.
        let x_idx: usize = abs_x.div_floor(&self.chunk_x_size) as usize;
        let y_idx: usize = abs_y.div_floor(&self.chunk_y_size) as usize;
        
        if (x_idx as u32) < self.x_chunks && (y_idx as u32) < self.y_chunks {
            Some(&self.chunks[y_idx][x_idx])
        } else {
            None
        }
    }

    pub fn vert_position_at(&self, abs_x: i32, abs_y: i32) -> Option<Vector3<f32>> {
        if abs_x >= 0 && abs_y >= 0 {
            match self.chunk_containing(abs_x as u32, abs_y as u32) {
                Some(cell) => {
                    let chunk = cell.borrow();
                    chunk.vert_position_at(abs_x as u32, abs_y as u32)
                },
                None => None
            }
        } else {
            None
        }
    }
    
    pub fn draw(&self, camera: &Camera, terrain_program: &terrain::Program, water_program: &water::Program) {
        // Draw the terrain first.
        for inner_vec in self.chunks.iter() {
            for cell in inner_vec.iter() {
                let chunk = cell.borrow();
                chunk.draw_terrain(camera, terrain_program);
            }
        }
        
        // Draw the water second, because it's partially transparent.
        for inner_vec in self.chunks.iter() {
            for cell in inner_vec.iter() {
                let chunk = cell.borrow();
                chunk.draw_water(camera, water_program);
            }
        }
    }
}