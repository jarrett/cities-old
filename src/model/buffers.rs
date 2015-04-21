use std::mem;
use std::ptr;
use libc::{c_void};
use gl;
use gl::types::*;
use cgmath::{Point3, Vector2};
use model;

pub struct Buffers {
    pub positions: Vec<Point3<f32>>,
    pub uvs:       Vec<Vector2<f32>>,
    pub indices:   Vec<u16>,
    
    pub position_buffer: GLuint,
    pub uv_buffer:       GLuint,
    pub index_buffer:    GLuint,
    
    pub vao:             GLuint,
    
    pub uploaded:        bool
}

impl Buffers {
    pub fn new() -> Buffers {
        let mut buffers = Buffers {
            positions: Vec::new(),
            uvs:       Vec::new(),
            indices:   Vec::new(),
            
            position_buffer: 0, uv_buffer: 0, index_buffer: 0, vao: 0, uploaded: false
        };
        
        unsafe {
            gl::GenBuffers(1,      &mut buffers.position_buffer);
            gl::GenBuffers(1,      &mut buffers.uv_buffer);
            gl::GenBuffers(1,      &mut buffers.index_buffer);
            gl::GenVertexArrays(1, &mut buffers.vao);
        }
        
        buffers
    }
    
    pub fn upload(&mut self, program: &model::Program3d) {
        unsafe {
            gl::BindVertexArray(self.vao);
            
            // Positions.
            gl::BindBuffer(gl::ARRAY_BUFFER, self.position_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<f32>() * 3 * self.positions.len()) as i64,
                self.positions.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            gl::EnableVertexAttribArray(program.position_idx);
            gl::VertexAttribPointer(program.position_idx, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            
            // UVs.
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<f32>() * 2 * self.uvs.len()) as i64,
                self.uvs.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            gl::EnableVertexAttribArray(program.uv_idx);
            gl::VertexAttribPointer(program.uv_idx, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            
            // Indices.
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mem::size_of::<u16>() * self.indices.len()) as i64,
                self.indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            
            gl::BindVertexArray(0);
        }
        self.uploaded = true;
    }
}