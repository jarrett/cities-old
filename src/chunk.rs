use std::mem;
use world::World;
use libc::{c_void};
use cgmath::*;
use gl;
use gl::types::*;

pub struct Chunk {
    terrain_positions: Vec<Vector3<f32>>,
    terrain_normals: Vec<Vector3<f32>>,
  
    water_positions: Vec<Vector2<f32>>,
    water_depths: Vec<f32>,
  
    min_x:    u32, // Minimum X position.
    min_y:    u32, // Minimum Y position.
    x_verts:  u32, // Number of verts along the X axis.
    y_verts:  u32, // Number of verts along the Y axis.
    x_size:   u32, // X dimension. Equal to x_verts - 1.
    y_size:   u32, // Y dimension. Equal to y_verts - 1.
    
    index_buffer:             GLuint, // Used by terrain and water.
    terrain_position_buffer:  GLuint,
    terrain_normal_buffer:    GLuint,
    terrain_vao:              GLuint,
    water_position_buffer:    GLuint,
    water_depth_buffer:       GLuint,
    water_vao:                GLuint
}

impl Chunk {
    // Signed because we can look beyond the boundaries of the current chunk.
    pub fn absolutize_x(&self, rel_x: i32) -> i32 {
        rel_x + self.min_x as i32
    }
    
    // Signed because we can look beyond the boundaries of the current chunk.
    pub fn absolutize_y(&self, rel_y: i32) -> i32 {
        rel_y + self.min_y as i32
    }
    
    // Returns the index of the vertex at the given relative coords.
    pub fn vi(&self, rel_x: u32, rel_y: u32) -> usize {
        ((self.x_verts * rel_y) + rel_x) as usize
    }
  
    pub fn buffer_depths(&mut self) {
        unsafe {
          gl::BindBuffer(gl::ARRAY_BUFFER, self.water_depth_buffer);
          gl::BufferData(
              gl::ARRAY_BUFFER,
              (mem::size_of::<f32>() as u32 * self.x_verts * self.y_verts) as i64,
              self.water_depths.as_ptr() as *const c_void,
              gl::DYNAMIC_DRAW
          );
          gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
    
    pub fn buffer_normals(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.terrain_normal_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<f32>() as u32 * self.x_verts * self.y_verts) as i64,
                self.terrain_normals.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
    
    /* The vertex attributes are interleaved, position followed by normal. */
    pub fn buffer_positions(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.terrain_position_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<Vector3<f32>>() as u32 * self.x_verts * self.y_verts) as i64,
                self.terrain_positions.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW
            );
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.water_position_buffer);
            gl::BufferData(
              gl::ARRAY_BUFFER,
              (mem::size_of::<Vector3<f32>>() as u32 * self.x_verts * self.y_verts) as i64,
              self.water_positions.as_ptr() as *const c_void,
              gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
    
    pub fn buffer_indices(&mut self) {
        let size: usize = ((self.y_verts - 1) * (self.x_verts - 1) * 6) as usize;
        let mut indices: Vec<GLushort> = Vec::with_capacity(size);
        let mut i: usize = 0;
        for y in 0u32..(self.y_verts - 1) {
            for x in 0u32..(self.x_verts - 1) {
                // Buffer the quad having a min (NW) vertex at (x, y).
                
                // NW triangle.
                indices[i + 0] = (( y      * self.x_verts) + x    ) as GLushort; // NW.
                indices[i + 1] = (( y      * self.x_verts) + x + 1) as GLushort; // NE.
                indices[i + 2] = (((y + 1) * self.x_verts) + x    ) as GLushort;  // SW.
      
                // SE triangle.
                indices[i + 3] = (((y + 1) * self.x_verts) + x + 1) as GLushort; // SE.
                indices[i + 4] = (((y + 1) * self.x_verts) + x    ) as GLushort; // SW.
                indices[i + 5] = (( y      * self.x_verts) + x + 1) as GLushort; // NE.
      
                i += 6;
            }
        }
        unsafe {
          gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
          gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (mem::size_of::<GLushort>() * size) as i64,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW
          );
          gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
    
    /* Calculates the normal for each vertex. The vertex normal is defined as the average
    normal of all adjacent triangles. (Implicitly, that means each quad gets triangulated
    both ways for the purpose of normal calculation. Kind of weird, but not an issue
    practically speaking.) */
    pub fn calc_normals(&mut self, world: &World) {
        
        for rel_y in 0u32..(self.y_verts) {
            for rel_x in 0u32..(self.x_verts) {
                // Calculate the normal for the vertex at (rel_x, rel_y). (rel_x, rel_y)
                // is not in world coords, but vertex indices local to this chunk.
                
                /* maybe_add_tri_normal needs signed integers. Give them names
                for convenience. */
                let rel_x_s: i32 = rel_x as i32;
                let rel_y_s: i32 = rel_y as i32;
                
                // As we iterate over the adjacent triangles, this value accumulates.
                // After we do all the adjacents, we normalize to yield the average
                // normal. We don't have to divide by the number of triangles because
                // normalizing achieves the scaling we need anyway.
                let mut sum_norm: Vector3<f32> = Vector3::new(0f32, 0f32, 0f32);
      
                // The root is the vertex that's not one of the two legs. It's the vertex
                // at the current (x, y).
                let root: &Vector3<f32> = &(self.terrain_positions[
                    self.vi(rel_x, rel_y)
                ]);
      
                // Iterate over the adjacent triangles. (Some of which may not exist, if
                // we're on an edge or corner.)
                self.maybe_add_tri_normal(world, &mut sum_norm, root, rel_x_s - 1, rel_y_s, rel_x_s, rel_y_s - 1); // -x, -y direction (NW).
                self.maybe_add_tri_normal(world, &mut sum_norm, root, rel_x_s + 1, rel_y_s, rel_x_s, rel_y_s - 1); // +x, -y direction (NE).
                self.maybe_add_tri_normal(world, &mut sum_norm, root, rel_x_s + 1, rel_y_s, rel_x_s, rel_y_s + 1); // +x, +y direction (SE).
                self.maybe_add_tri_normal(world, &mut sum_norm, root, rel_x_s - 1, rel_y_s, rel_x_s, rel_y_s + 1); // -x, +y direction (SW).
      
                // The accumulator sum_norm now contains the sum of the normals of all the
                // adjacent triangles. Now we need to normalize it. (See the note above
                // about dividing by the number of triangles.)
                sum_norm.normalize_self();
      
                // We now know the normal for this particular vertex. Copy that value to
                // the terrain_normals vector.
                let vi = self.vi(rel_x, rel_y);
                self.terrain_normals[vi] = sum_norm;
            }
        }
    }
    
    fn maybe_add_tri_normal(
        &self, world: &World, sum_norm: &mut Vector3<f32>, root: &Vector3<f32>,
        leg_1_rel_x: i32, leg_1_rel_y: i32, leg_2_rel_x: i32, leg_2_rel_y: i32
    ) {        
        // Find the vertex position of each of the legs. If the proposed leg falls outside
        // the world's boundaries, the return value is None.
        let leg1_opt: Option<Vector3<f32>> = world.vert_position_at(
            self.absolutize_x(leg_1_rel_x) as i32,
            self.absolutize_y(leg_1_rel_y) as i32
        );
        let leg2_opt: Option<Vector3<f32>> = world.vert_position_at(
            self.absolutize_x(leg_2_rel_x) as i32,
            self.absolutize_y(leg_2_rel_y) as i32
        );
        
        match (leg1_opt, leg2_opt) {  
            (Some(leg1), Some(leg2)) => {
                let leg1: Vector3<f32> = leg1_opt.unwrap();
                let leg2: Vector3<f32> = leg2_opt.unwrap();
            
                // The direction vectors of each leg of this triangle.
                let leg_dir_1: Vector3<f32> = leg1.sub_v(root);
                let leg_dir_2: Vector3<f32> = leg2.sub_v(root);
            
                // The normal of this triangle is the cross product of the two leg directions.
                let mut tri_norm: Vector3<f32> = leg_dir_1.cross(&leg_dir_2).normalize();
            
                // Depending on which way we did the cross product, the normal vector may be
                // pointing up or down. Here we ensure it points up.
                if tri_norm.z < 0f32 { tri_norm.neg_self(); }
                
                sum_norm.add_self_v(&tri_norm);
            },
            _ => {}
        }
    }
}