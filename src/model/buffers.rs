use std::mem;
use std::ptr;
use libc::{c_void};
use gl;
use gl::types::*;
use cgmath::{Point3, Vector2};

use opengl::{Vbo, Vao, Attributes, Indices};
use model;

pub struct Buffers {
    pub positions: Vec<Point3<f32>>,
    pub uvs:       Vec<Vector2<f32>>,
    pub indices:   Vec<u16>,
    
    pub position_buffer: Vbo,
    pub uv_buffer:       Vbo,
    pub index_buffer:    Vbo,
    
    pub vao:             Vao,
    
    pub uploaded:        bool
}

impl Buffers {
    pub fn new() -> Buffers {
        Buffers {
            positions:        Vec::new(),
            uvs:              Vec::new(),
            indices:          Vec::new(),
            position_buffer:  Vbo::new(Attributes),
            uv_buffer:        Vbo::new(Attributes),
            index_buffer:     Vbo::new(Indices),
            vao:              Vao::new(),
            uploaded:         false
        }
    }
    
    pub fn upload(&mut self, program: &model::Program3d) {
        unsafe {
            self.vao.bind();
            
            // Positions.
            self.position_buffer.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<f32>() * 3 * self.positions.len()) as i64,
                self.positions.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            gl::EnableVertexAttribArray(program.position_idx);
            gl::VertexAttribPointer(program.position_idx, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            
            // UVs.
            self.uv_buffer.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<f32>() * 2 * self.uvs.len()) as i64,
                self.uvs.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            gl::EnableVertexAttribArray(program.uv_idx);
            gl::VertexAttribPointer(program.uv_idx, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
            
            Vbo::unbind(Attributes);
            Vao::unbind();
            
            // Indices.
            self.index_buffer.bind();
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mem::size_of::<u16>() * self.indices.len()) as i64,
                self.indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            Vbo::unbind(Indices);
        }
        self.uploaded = true;
    }
}