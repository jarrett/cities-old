use std::fmt::Debug;
use std::ptr;
use std::mem;
use std::path::Path;
use gl;
use gl::types::*;
use libc::{c_void};
use cgmath::{Point, Point3};

use glutil;
use camera::Camera;
use math::PLine3;

#[allow(dead_code)]
pub struct DebugLines {
    pub positions:       Vec<Point3<f32>>,
    pub colors:          Vec<Point3<f32>>,
    pub indices:         Vec<u16>,
    
    pub position_buffer: GLuint,
    pub color_buffer:    GLuint,
    pub index_buffer:    GLuint,
    pub vao:             GLuint,
    
    pub program:         GLuint,
    pub position_idx:    GLuint,
    pub color_idx:       GLuint,
    pub model_view_idx:  GLint,
    pub projection_idx:  GLint,
    
    pub next_attr:       usize,
    pub next_index:      usize,
}

impl DebugLines {
    pub fn new() -> DebugLines {
        let mut lines = DebugLines {
            positions: Vec::new(), colors: Vec::new(), indices: Vec::new(),
            position_buffer: 0, color_buffer: 0, index_buffer: 0, vao: 0,
            program: 0, position_idx: 0, color_idx: 0,
            model_view_idx: 0, projection_idx: 0,
            next_attr: 0, next_index: 0
        };
    
        unsafe {
            gl::GenBuffers(1,      &mut lines.position_buffer);
            gl::GenBuffers(1,      &mut lines.color_buffer);
            gl::GenBuffers(1,      &mut lines.index_buffer);
            gl::GenVertexArrays(1, &mut lines.vao);
            lines.program        = glutil::make_program(&Path::new("glsl/debug-lines.vert.glsl"), &Path::new("glsl/debug-lines.frag.glsl"));
            lines.position_idx   = glutil::get_attrib_location(lines.program, "position");
            lines.color_idx      = glutil::get_attrib_location(lines.program, "color");
            lines.model_view_idx = glutil::get_uniform_location(lines.program, "modelView");
            lines.projection_idx = glutil::get_uniform_location(lines.program, "projection");
        
            gl::BindVertexArray(lines.vao);
        
            gl::BindBuffer(gl::ARRAY_BUFFER, lines.position_buffer);
            gl::EnableVertexAttribArray(lines.position_idx);
            gl::VertexAttribPointer(
              lines.position_idx as GLuint,
              3,
              gl::FLOAT,
              gl::FALSE,
              0,
              ptr::null::<c_void>() as *const c_void,
            );
        
            gl::BindBuffer(gl::ARRAY_BUFFER, lines.color_buffer);
            gl::EnableVertexAttribArray(lines.color_idx);
            gl::VertexAttribPointer(
              lines.color_idx as GLuint,
              3,
              gl::FLOAT,
              gl::FALSE,
              0,
              ptr::null::<c_void>() as *const c_void,
            );
        
            gl::BindVertexArray(0);
        }
        
        lines
    }
    
    pub fn add_pline3(
        &mut self, line: &PLine3,
        r1: f32, g1: f32, b1: f32,
        r2: f32, g2: f32, b2: f32
    ) {
        self.add_segment(
            Point3::from_vec(&line.ori), Point3::from_vec(&line.dir),
            r1, g1, b1,
            r2, g2, b2
        );
    }
    
    pub fn add_segment(
        &mut self, p1: Point3<f32>, p2: Point3<f32>,
        r1: f32, g1: f32, b1: f32,
        r2: f32, g2: f32, b2: f32
    ) {
        let index_offset = self.positions.len() as u16;
        self.indices.push_all(&[index_offset, index_offset + 1]);
        
        self.positions.push(p1);
        self.positions.push(p2);
        
        self.colors.push(Point3::new(r1, g1, b1));
        self.colors.push(Point3::new(r2, g2, b2));
        
        self.buffer();
    }
    
    pub fn clear(&mut self) {
        self.positions.clear();
        self.colors.clear();
        self.indices.clear();
        self.buffer();
    }
    
    pub fn draw(&self, camera: &Camera) {
        unsafe {
            gl::LineWidth(1.0);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::UseProgram(self.program);
            gl::UniformMatrix4fv(self.model_view_idx, 1, gl::FALSE, mem::transmute(&camera.model_view));
            gl::UniformMatrix4fv(self.projection_idx, 1, gl::FALSE, mem::transmute(&camera.projection));
            gl::DrawElements(gl::LINES, self.positions.len() as i32, gl::UNSIGNED_SHORT, ptr::null::<c_void>() as *const c_void);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    
    fn buffer(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.position_buffer);
            gl::BufferData(
              gl::ARRAY_BUFFER,
              // 4 bytes per float, 3 floats per vertex.
              4 * 3 * self.positions.len() as i64,
              self.positions.as_ptr() as *const c_void,
              gl::DYNAMIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
            gl::BufferData(
              gl::ARRAY_BUFFER,
              // 4 bytes per float, 3 floats per vertex.
              4 * 3 * self.colors.len() as i64,
              self.colors.as_ptr() as *const c_void,
              gl::DYNAMIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(
              gl::ELEMENT_ARRAY_BUFFER,
              // 2 bytes per index.
              2 * self.indices.len() as i64,
              self.indices.as_ptr() as *const c_void,
              gl::DYNAMIC_DRAW
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}