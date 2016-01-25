use std::ptr;
use std::mem;
use std::path::Path;
use gl;
use gl::types::*;
use libc::{c_void};
use cgmath::{Point, Point3, Ray3};

use opengl::{Program, Vbo, Vao, Attributes, Indices};
use camera::Camera;

#[allow(dead_code)]
pub struct DebugLines {
    pub positions:       Vec<Point3<f32>>,
    pub colors:          Vec<Point3<f32>>,
    pub indices:         Vec<u16>,
    
    pub position_buffer: Vbo,
    pub color_buffer:    Vbo,
    pub index_buffer:    Vbo,
    pub vao:             Vao,
    
    pub program:         Program,
    pub position_idx:    GLuint,
    pub color_idx:       GLuint,
    pub camera_idx:      GLint,
    
    pub next_attr:       usize,
    pub next_index:      usize,
}

impl DebugLines {
    #[allow(dead_code)]
    pub fn new() -> DebugLines {
        let mut lines = DebugLines {
            positions: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
            position_buffer: Vbo::new(Attributes),
            color_buffer: Vbo::new(Attributes),
            index_buffer: Vbo::new(Indices),
            vao: Vao::new(),
            program: Program::new(
                &Path::new("glsl/debug-lines.vert.glsl"),
                &Path::new("glsl/debug-lines.frag.glsl")
            ),
            position_idx: 0,
            color_idx: 0,
            camera_idx: 0,
            next_attr: 0,
            next_index: 0
        };
    
        unsafe {
            lines.position_idx   = lines.program.get_attrib_location("position");
            lines.color_idx      = lines.program.get_attrib_location("color");
            lines.camera_idx     = lines.program.get_uniform_location("camera");
        
            lines.vao.bind();
        
            lines.position_buffer.bind();
            gl::EnableVertexAttribArray(lines.position_idx);
            gl::VertexAttribPointer(
              lines.position_idx as GLuint,
              3,
              gl::FLOAT,
              gl::FALSE,
              0,
              ptr::null::<c_void>() as *const c_void,
            );
        
            lines.color_buffer.bind();
            gl::EnableVertexAttribArray(lines.color_idx);
            gl::VertexAttribPointer(
              lines.color_idx as GLuint,
              3,
              gl::FLOAT,
              gl::FALSE,
              0,
              ptr::null::<c_void>() as *const c_void,
            );
        
            Vbo::unbind(Attributes);
            Vao::unbind();
        }
        
        lines
    }
    
    #[allow(dead_code)]
    pub fn add_ray3(
        &mut self, ray: &Ray3<f32>,
        r1: f32, g1: f32, b1: f32,
        r2: f32, g2: f32, b2: f32
    ) {
        self.add_segment(
            ray.origin.clone(), ray.origin.add_v(&ray.direction),
            r1, g1, b1,
            r2, g2, b2
        );
    }
    
    #[allow(dead_code)]
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
    
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.positions.clear();
        self.colors.clear();
        self.indices.clear();
        self.buffer();
    }
    
    #[allow(dead_code)]
    pub fn draw(&self, camera: &Camera) {
        unsafe {
            gl::LineWidth(1.0);
            self.vao.bind();
            self.index_buffer.bind();
            gl::UseProgram(self.program.id);
            gl::UniformMatrix4fv(self.camera_idx, 1, gl::FALSE, mem::transmute(&camera.transform));
            gl::DrawElements(gl::LINES, self.positions.len() as i32, gl::UNSIGNED_SHORT, ptr::null::<c_void>() as *const c_void);
            Vbo::unbind(Indices);
            Vao::unbind();
        }
    }
    
    #[allow(dead_code)]
    fn buffer(&mut self) {
        self.position_buffer.buffer_data(
            // 4 bytes per float, 3 floats per vertex.
            4 * 3 * self.positions.len(),
            &self.positions,
            gl::DYNAMIC_DRAW
        );
    
        self.color_buffer.buffer_data(
            // 4 bytes per float, 3 floats per vertex.
            4 * 3 * self.colors.len(),
            &self.colors,
            gl::DYNAMIC_DRAW
        );
    
        self.index_buffer.buffer_data(
            // 2 bytes per index.
            2 * self.indices.len(),
            &self.indices,
            gl::DYNAMIC_DRAW
        );
    }
}