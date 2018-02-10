use std::mem;
use std::ptr;
use gl;
use gl::types::*;
use cgmath::{Point3, Vector2};

use opengl::{Vbo, Vao, Attributes, Indices};
use model;

pub struct Buffers {
    pub positions: Vec<Point3<f32>>,
    pub uvs:       Vec<Vector2<f32>>,
    pub indices:   Vec<u16>,
    
    pub position_buffer: Vbo<Attributes>,
    pub uv_buffer:       Vbo<Attributes>,
    pub index_buffer:    Vbo<Indices>,
    
    pub vao:             Vao,
    
    pub uploaded:        bool
}

impl Buffers {
    pub fn new() -> Buffers {
        Buffers {
            positions:        Vec::new(),
            uvs:              Vec::new(),
            indices:          Vec::new(),
            position_buffer:  Vbo::new(),
            uv_buffer:        Vbo::new(),
            index_buffer:     Vbo::new(),
            vao:              Vao::new(),
            uploaded:         false
        }
    }
    
    pub fn upload(&mut self, program: &model::Program3d) {
        self.position_buffer.buffer_data(
            4 * 3 * self.positions.len(),
            &self.positions,
            gl::STATIC_DRAW
        );
        
        self.uv_buffer.buffer_data(
            4 * 2 * self.uvs.len(),
            &self.uvs,
            gl::STATIC_DRAW
        );
        
        self.index_buffer.buffer_data(
            mem::size_of::<u16>() * self.indices.len(),
            &self.indices,
            gl::STATIC_DRAW
        );
        
        unsafe {
            self.vao.bind();
            self.vao.attrib(&self.position_buffer, program.position_idx, 3, gl::FLOAT, 0, 0);
            self.vao.attrib(&self.uv_buffer, program.uv_idx, 2, gl::FLOAT, 0, 0);
            Vao::unbind();
        }
        
        self.uploaded = true;
    }
}