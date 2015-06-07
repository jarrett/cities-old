use std::mem;
use std::ptr;
use libc::{c_void};
use cgmath::{Vector, EuclideanVector, Vector3, Point, Point3};
use gl;
use gl::types::*;

use world::World;
use terrain;
use overlay::Overlay;
use camera::Camera;
use math::Quad;
use mouse;

static WATER_Z: f32 = 5.0;

pub struct Chunk {
    terrain_positions: Vec<Point3<f32>>,
    terrain_normals: Vec<Vector3<f32>>,
  
    water_positions: Vec<Point3<f32>>,
    water_depths: Vec<f32>,
    
    min_x:    usize, // Minimum X position.
    min_y:    usize, // Minimum Y position.
    x_verts:  usize, // Number of verts along the X axis.
    y_verts:  usize, // Number of verts along the Y axis.
    x_size:   usize, // X dimension. Equal to x_verts - 1.
    y_size:   usize, // Y dimension. Equal to y_verts - 1.
    
    index_buffer:             GLuint, // Used by terrain and water.
    terrain_position_buffer:  GLuint,
    terrain_normal_buffer:    GLuint,
    terrain_vao:              GLuint,
    water_position_buffer:    GLuint,
    water_depth_buffer:       GLuint,
    water_vao:                GLuint,
    
    pub overlay:              Overlay,
    
    positions_buffered:       bool,
    normals_buffered:         bool,
    indices_buffered:         bool,
    depths_buffered:          bool
}

pub struct Quads<'a> {
    x: usize,
    y: usize,
    chunk: &'a Chunk
}

impl Chunk {
    pub fn new(
        terrain_program: &terrain::ground::Program, water_program: &terrain::water::Program,
        min_x: usize, min_y: usize, x_verts: usize, y_verts: usize
    ) -> Chunk {
        let vec_size: usize = (x_verts * y_verts) as usize;
        let mut chunk = Chunk {
            terrain_positions: Vec::with_capacity(vec_size),
            terrain_normals:   Vec::with_capacity(vec_size),
            water_positions:   Vec::with_capacity(vec_size),
            water_depths:      Vec::with_capacity(vec_size),
            
            min_x:   min_x,           min_y:   min_y,
            x_verts: x_verts,         y_verts: y_verts,
            x_size:  x_verts - 1,     y_size:  y_verts - 1,
            
            index_buffer: 0, terrain_position_buffer: 0, terrain_normal_buffer: 0,
            terrain_vao: 0, water_position_buffer: 0, water_depth_buffer: 0, water_vao: 0,
            
            overlay: Overlay::new(x_verts - 1, y_verts - 1),
            
            positions_buffered: false, normals_buffered: false,
            indices_buffered: false, depths_buffered: false
        };
        
        // Initialize each vertex to a default value. X and Y positions can be determined
        // with the information we already have. For the water, the Z position is always
        // the same. For the terrain, the Z position defaults to zero. Normals default to
        // straight up.
        for y in 0usize..y_verts {
            for x in 0usize..x_verts {
                // The absolutize functions require signed values.
                let x_s: i32 = x as i32;
                let y_s: i32 = y as i32;
                let abs_x: f32 = chunk.absolutize_x(x_s) as f32;
                let abs_y: f32 = chunk.absolutize_y(y_s) as f32;
                
                chunk.water_depths.push(0.0);
                chunk.water_positions.push(Point3::new(abs_x, abs_y, WATER_Z));
                
                chunk.terrain_positions.push(Point3::new(abs_x, abs_y, 0.0));
                
                chunk.terrain_normals.push(Vector3::new(0.0, 0.0, 1.0));
            }
        }
        
        unsafe {
            gl::GenBuffers(1,      &mut chunk.terrain_position_buffer);
            gl::GenBuffers(1,      &mut chunk.terrain_normal_buffer);
            gl::GenBuffers(1,      &mut chunk.index_buffer);
            gl::GenVertexArrays(1, &mut chunk.terrain_vao);
            gl::GenBuffers(1,      &mut chunk.water_position_buffer);
            gl::GenBuffers(1,      &mut chunk.water_depth_buffer);
            gl::GenVertexArrays(1, &mut chunk.water_vao);
        }
        
        chunk.configure_vaos(terrain_program, water_program);
        
        chunk
    }
    
    // Signed because we can look beyond the boundaries of the current chunk.
    pub fn absolutize_x(&self, rel_x: i32) -> i32 {
        rel_x + self.min_x as i32
    }
    
    // Signed because we can look beyond the boundaries of the current chunk.
    pub fn absolutize_y(&self, rel_y: i32) -> i32 {
        rel_y + self.min_y as i32
    }
    
    pub fn relativize_x(&self, abs_x: usize) -> usize {
        abs_x - self.min_x
    }
    
    pub fn relativize_y(&self, abs_y: usize) -> usize {
        abs_y - self.min_y
    }
    
    // Returns the index of the vertex at the given relative coords.
    pub fn vi(&self, rel_x: usize, rel_y: usize) -> usize {
        ((self.x_verts * rel_y) + rel_x) as usize
    }
    
    pub fn contains_rel(&self, rel_x: i32, rel_y: i32) -> bool {
        rel_x >= 0 && rel_x < self.x_verts as i32 && rel_y >= 0 && rel_y < self.y_verts as i32
    }
    
    pub fn vert_position_at(&self, abs_x: usize, abs_y: usize) -> Option<Point3<f32>> {
      if abs_x >= self.min_x && abs_x <= (self.min_x + self.x_size) &&
         abs_y >= self.min_y && abs_y <= (self.min_y + self.y_size)
      {
          return Some(self.terrain_positions[
            self.vi(self.relativize_x(abs_x), self.relativize_y(abs_y))
          ]);
      } else {
          None
      }
    }
    
    // Does not recalc the terrain normals or rebuffer to the GPU. So be sure to do those
    // as necessary after calling this method. */
    pub fn set_height(&mut self, rel_x: usize, rel_y: usize, abs_z: f32) {
        if rel_x >= self.x_verts { panic!("rel_x ({}) greater than x_verts ({})", rel_x, self.x_verts); }
        if rel_y >= self.x_verts { panic!("rel_y ({}) greater than x_verts ({})", rel_x, self.x_verts); }
        let vert_idx = self.vi(rel_x, rel_y);
        self.terrain_positions[vert_idx].z = abs_z;
        let mut depth: f32 = WATER_Z - abs_z;
        if depth < 0.0 { depth = 0.0; }
        self.water_depths[vert_idx] = depth;
    }
    
    pub fn buffer_depths(&mut self) {
        unsafe {
          gl::BindBuffer(gl::ARRAY_BUFFER, self.water_depth_buffer);
          gl::BufferData(
              gl::ARRAY_BUFFER,
              (mem::size_of::<f32>() as usize * self.x_verts * self.y_verts) as i64,
              self.water_depths.as_ptr() as *const c_void,
              gl::DYNAMIC_DRAW
          );
          gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        self.depths_buffered = true;
    }
    
    pub fn buffer_normals(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.terrain_normal_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<f32>() as usize * 3 * self.x_verts * self.y_verts) as i64,
                self.terrain_normals.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        self.normals_buffered = true;
    }
    
    pub fn buffer_positions(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.terrain_position_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<Point3<f32>>() as usize * self.x_verts * self.y_verts) as i64,
                self.terrain_positions.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW
            );
            
            gl::BindBuffer(gl::ARRAY_BUFFER, self.water_position_buffer);
            gl::BufferData(
              gl::ARRAY_BUFFER,
              (mem::size_of::<Point3<f32>>() as usize * self.x_verts * self.y_verts) as i64,
              self.water_positions.as_ptr() as *const c_void,
              gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        self.positions_buffered = true;
    }
    
    pub fn buffer_indices(&mut self) {
        let size: usize = ((self.y_verts - 1) * (self.x_verts - 1) * 6) as usize;
        let mut indices: Vec<GLushort> = Vec::with_capacity(size);
        for y in 0usize..(self.y_verts - 1) {
            for x in 0usize..(self.x_verts - 1) {
                // Buffer the quad having a min (NW) vertex at (x, y).
                
                // NW triangle.
                indices.push((( y      * self.x_verts) + x    ) as GLushort); // NW.
                indices.push((( y      * self.x_verts) + x + 1) as GLushort); // NE.
                indices.push((((y + 1) * self.x_verts) + x    ) as GLushort); // SW.
      
                // SE triangle.
                indices.push((((y + 1) * self.x_verts) + x + 1) as GLushort); // SE.
                indices.push((((y + 1) * self.x_verts) + x    ) as GLushort); // SW.
                indices.push((( y      * self.x_verts) + x + 1) as GLushort); // NE.
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
        self.indices_buffered = true;
    }
    
    /* Calculates the normal for each vertex. The vertex normal is defined as the average
    normal of all adjacent triangles. (Implicitly, that means each quad gets triangulated
    both ways for the purpose of normal calculation. Kind of weird, but not an issue
    practically speaking.) */
    pub fn calc_normals(&mut self, world: &World) {
        for rel_y in 0usize..(self.y_verts) {
            for rel_x in 0usize..(self.x_verts) {
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
                let root: &Point3<f32> = &(self.terrain_positions[
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
        &self, world: &World, sum_norm: &mut Vector3<f32>, root: &Point3<f32>,
        leg_1_rel_x: i32, leg_1_rel_y: i32, leg_2_rel_x: i32, leg_2_rel_y: i32
    ) {
        let leg_1_abs_x = self.absolutize_x(leg_1_rel_x) as i32;
        let leg_1_abs_y = self.absolutize_y(leg_1_rel_y) as i32;
        let leg_2_abs_x = self.absolutize_x(leg_2_rel_x) as i32;
        let leg_2_abs_y = self.absolutize_y(leg_2_rel_y) as i32;
        // Find the vertex position of each of the legs. If the proposed leg falls outside
        // the world's boundaries, the return value is None.
        // 
        // If this chunk contains the vertex, we can't go through the world. This chunk
        // has already been borrowed mutably, so the world can't access it right now.
        let leg_1_opt: Option<Point3<f32>>;
        let leg_2_opt: Option<Point3<f32>>;
        
        if self.contains_rel(leg_1_rel_x, leg_1_rel_y) {
          leg_1_opt = self.vert_position_at(leg_1_abs_x as usize, leg_1_abs_y as usize);
        } else {
          leg_1_opt = world.vert_position_at(leg_1_abs_x, leg_1_abs_y);
        }
        
        if self.contains_rel(leg_2_rel_x, leg_2_rel_y) {
          leg_2_opt = self.vert_position_at(leg_2_abs_x as usize, leg_2_abs_y as usize);
        } else {
          leg_2_opt = world.vert_position_at(leg_2_abs_x, leg_2_abs_y);
        }
        
        match (leg_1_opt, leg_2_opt) {  
            (Some(leg_1), Some(leg_2)) => {            
                // The direction vectors of each leg of this triangle.
                let leg_dir_1: Vector3<f32> = leg_1.sub_p(root);
                let leg_dir_2: Vector3<f32> = leg_2.sub_p(root);
            
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
    
    fn configure_vaos(&mut self, terrain_program: &terrain::ground::Program, water_program: &terrain::water::Program) {
        unsafe {
            // Terrain.
            gl::BindVertexArray(self.terrain_vao);
        
            gl::BindBuffer(gl::ARRAY_BUFFER, self.terrain_position_buffer);
            gl::EnableVertexAttribArray(terrain_program.position_idx);
            gl::VertexAttribPointer(terrain_program.position_idx, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        
            gl::BindBuffer(gl::ARRAY_BUFFER, self.terrain_normal_buffer);
            gl::EnableVertexAttribArray(terrain_program.normal_idx);
            gl::VertexAttribPointer(terrain_program.normal_idx, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        
            // Water.
            gl::BindVertexArray(self.water_vao);
        
            gl::BindBuffer(gl::ARRAY_BUFFER, self.water_position_buffer);
            gl::EnableVertexAttribArray(water_program.position_idx);
            gl::VertexAttribPointer(water_program.position_idx, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        
            gl::BindBuffer(gl::ARRAY_BUFFER, self.water_depth_buffer);
            gl::EnableVertexAttribArray(water_program.depth_idx);
            gl::VertexAttribPointer(water_program.depth_idx, 1, gl::FLOAT, gl::FALSE, 0, ptr::null());
        
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    
    pub fn draw_terrain(&self, camera: &Camera, terrain_program: &terrain::ground::Program, mouse_hit: &Option<mouse::Hit>) {
        if !self.positions_buffered { panic!("Called draw_terrain before buffering positions"); }
        if !self.normals_buffered   { panic!("Called draw_terrain before buffering normals"); }
        if !self.indices_buffered   { panic!("Called draw_terrain before buffering indices"); }
        unsafe {
            gl::BindVertexArray(self.terrain_vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::UseProgram(terrain_program.id);
            gl::UniformMatrix4fv(terrain_program.camera_idx, 1, gl::FALSE, mem::transmute(&camera.transform));
            match *mouse_hit {
                Some(ref hit) => {
                    gl::Uniform1ui(terrain_program.mouse_in_idx, 1);
                    gl::Uniform3f(terrain_program.mouse_position_idx, hit.at.x, hit.at.y, hit.at.z);
                },
                None => { 
                    gl::Uniform1ui(terrain_program.mouse_in_idx, 0);
                }
            }
            terrain_program.bind_textures();
            // Number of elements to draw = number of quads * 6 verts per quad.
            gl::DrawElements(gl::TRIANGLES, ((self.x_verts - 1) * (self.y_verts - 1) * 6) as i32, gl::UNSIGNED_SHORT, ptr::null());
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_water(&self, camera: &Camera, water_program: &terrain::water::Program) {
        if !self.positions_buffered { panic!("Called draw_terrain before buffering positions"); }
        if !self.depths_buffered    { panic!("Called draw_terrain before buffering depths"); }
        if !self.indices_buffered   { panic!("Called draw_terrain before buffering indices"); }
        unsafe {
            gl::BindVertexArray(self.water_vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::UseProgram(water_program.id);
            gl::UniformMatrix4fv(water_program.camera_idx, 1, gl::FALSE, mem::transmute(&camera.transform));
            water_program.bind_textures();
            // Number of elements to draw = number of quads * 6 verts per quad.
            gl::DrawElements(gl::TRIANGLES, ((self.x_verts - 1) * (self.y_verts - 1) * 6) as i32, gl::UNSIGNED_SHORT, ptr::null());
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    
    pub fn quads(&self) -> Quads {
        Quads::new(self)
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1,      &self.terrain_position_buffer);
            gl::DeleteBuffers(1,      &self.terrain_normal_buffer);
            gl::DeleteBuffers(1,      &self.index_buffer);
            gl::DeleteVertexArrays(1, &self.terrain_vao);
            gl::DeleteBuffers(1,      &self.water_position_buffer);
            gl::DeleteBuffers(1,      &self.water_depth_buffer);
            gl::DeleteVertexArrays(1, &self.water_vao);
        }
    }
}

impl<'a> Quads<'a> {
    pub fn new(chunk: &Chunk) -> Quads {
        Quads { chunk: chunk, x: 0, y: 0 }
    }
}

impl<'a> Iterator for Quads<'a> {
    type Item = Quad;
    
    fn next(&mut self) -> Option<Quad> {
        if self.x == self.chunk.x_size - 1 {
            if self.y == self.chunk.y_size - 1 {
                return None;
            } else {
                self.x = 0;
                self.y = self.y + 1;
            }
        } else {
            self.x = self.x + 1
        }
        Some((
            self.chunk.terrain_positions[self.chunk.vi(self.x    , self.y    )],
            self.chunk.terrain_positions[self.chunk.vi(self.x + 1, self.y    )],
            self.chunk.terrain_positions[self.chunk.vi(self.x + 1, self.y + 1)],
            self.chunk.terrain_positions[self.chunk.vi(self.x    , self.y + 1)]
        ))
    }
}