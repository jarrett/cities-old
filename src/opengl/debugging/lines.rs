use std::ptr;
use std::mem;
use std::path::Path;
use gl;
use gl::types::*;
use cgmath::Point3;

use opengl::{Program, Vbo, Vao, Attributes, Indices};
use camera::Camera;

#[allow(dead_code)]
pub struct DebugLines {
  pub positions:       Vec<Point3<f32>>,
  pub colors:          Vec<Point3<f32>>,
  pub indices:         Vec<u16>,

  pub position_buffer: Vbo<Attributes>,
  pub color_buffer:    Vbo<Attributes>,
  pub index_buffer:    Vbo<Indices>,
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
    let program = Program::new(
      &Path::new("glsl/debug-lines.vert.glsl"),
      &Path::new("glsl/debug-lines.frag.glsl")
    );
    
    let position_idx = program.get_attrib_location("position");
    let color_idx    = program.get_attrib_location("color");
    let camera_idx   = program.get_uniform_location("camera");
    
    let mut vao = Vao::new();
    let mut position_buffer: Vbo<Attributes> = Vbo::new();
    let mut color_buffer: Vbo<Attributes> = Vbo::new();
    
    vao.attrib(
      &position_buffer,
      position_idx as GLuint, // Index in program.
      3, // Number of vector components.
      gl::FLOAT, // Data type.
      0, // Stride.
      0 // Offset.
    );

    vao.attrib(
      &color_buffer,
      color_idx as GLuint, // Index in program.
      3, // Number of vector components.
      gl::FLOAT, // Data type.
      0, // Stride.
      0 // Offset.
    );
    
    let mut lines = DebugLines {
      positions: Vec::new(),
      colors: Vec::new(),
      indices: Vec::new(),
      position_buffer: position_buffer,
      color_buffer: color_buffer,
      index_buffer: Vbo::new(),
      vao: vao,
      program: program,
      position_idx: position_idx,
      color_idx: color_idx,
      camera_idx: camera_idx,
      next_attr: 0,
      next_index: 0
    };
    
    lines
  }
  
  #[allow(dead_code)]
  pub fn add_segment(
    &mut self, p1: Point3<f32>, p2: Point3<f32>,
    r1: f32, g1: f32, b1: f32,
    r2: f32, g2: f32, b2: f32
  ) {
    let index_offset = self.positions.len() as u16;
    self.indices.extend(&[index_offset, index_offset + 1]);
    
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
      gl::DrawElements(
        gl::LINES,
        self.positions.len() as i32, // Number of indices to draw.
        gl::UNSIGNED_SHORT, // Format of index buffer.
        ptr::null() // Offset into index buffer.
      );
      self.index_buffer.unbind();
      self.vao.unbind();
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