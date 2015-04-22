use std::mem;
use std::ptr;
use std::path::Path;
use gl;
use gl::types::*;
use libc::{c_void};

use camera::Camera;
use glutil;

pub struct AxisIndicator {
    pub program: GLuint,
    pub array_buffer: GLuint,
    pub element_array_buffer: GLuint,
    pub vao: GLuint,
    
    pub model_view_idx: GLint,
    pub projection_idx: GLint,
    pub scale_idx: GLint
}

impl AxisIndicator {
    pub fn new() -> AxisIndicator {
        let mut ai = AxisIndicator {
            program: 0, array_buffer: 0, element_array_buffer: 0, vao: 0,
            model_view_idx: 0, projection_idx: 0, scale_idx: 0
        };
        
        ai.program = glutil::make_program(&Path::new("glsl/axis-indicator.vert.glsl"), &Path::new("glsl/axis-indicator.frag.glsl"));
        
        let position_idx  = glutil::get_attrib_location( ai.program, "position");
        let color_idx     = glutil::get_attrib_location( ai.program, "color");
        ai.model_view_idx = glutil::get_uniform_location(ai.program, "modelView");
        ai.projection_idx = glutil::get_uniform_location(ai.program, "projection");
        ai.scale_idx      = glutil::get_uniform_location(ai.program, "scale");
        
        let attrs: [GLfloat; 36] = [
          // X
          0.0, 0.0, 0.0,    0.8, 0.0, 0.0,
          1.0, 0.0, 0.0,    0.8, 0.0, 0.0,
          // Y
          0.0, 0.0, 0.0,    0.0, 0.6, 0.0,
          0.0, 1.0, 0.0,    0.0, 0.6, 0.0,
          // Z
          0.0, 0.0, 0.0,    0.0, 0.0, 1.0,
          0.0, 0.0, 1.0,    0.0, 0.0, 1.0
        ];
        
        let indices: [GLubyte; 6] = [0, 1, 2, 3, 4, 5];
        
        unsafe {
            gl::UseProgram(ai.program);
            gl::GenBuffers(1, &mut ai.array_buffer);
            gl::GenBuffers(1, &mut ai.element_array_buffer);
            gl::GenVertexArrays(1, &mut ai.vao);
            
            gl::BindVertexArray(ai.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, ai.array_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ai.element_array_buffer);
            gl::BufferData(
              gl::ARRAY_BUFFER,
              (mem::size_of::<GLfloat>() * 36) as i64,
              attrs.as_ptr() as *const c_void,
              gl::STATIC_DRAW
            );
            gl::BufferData(
              gl::ELEMENT_ARRAY_BUFFER,
              (mem::size_of::<GLbyte>() * 6) as i64,
              indices.as_ptr() as *const c_void,
              gl::STATIC_DRAW
            );
            gl::EnableVertexAttribArray(position_idx);
            gl::VertexAttribPointer(
              position_idx as GLuint,
              3,
              gl::FLOAT,
              gl::FALSE,
              (mem::size_of::<GLfloat>() * 6) as GLint,
              ptr::null::<c_void>() as *const c_void,
            );
            gl::EnableVertexAttribArray(color_idx);
            gl::VertexAttribPointer(
              color_idx as GLuint,
              3,
              gl::FLOAT,
              gl::FALSE,
              (mem::size_of::<GLfloat>() * 6) as GLint,
              (mem::size_of::<GLfloat>() * 3) as *const c_void
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        
        ai
    }

    pub fn draw(&self, camera: &Camera, scale: f32) {
        unsafe {
            gl::LineWidth(1.0);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.array_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_array_buffer);
            gl::UseProgram(self.program);
            gl::UniformMatrix4fv(self.model_view_idx, 1, gl::FALSE, mem::transmute(&camera.model_view));
            gl::UniformMatrix4fv(self.projection_idx, 1, gl::FALSE, mem::transmute(&camera.projection));
            gl::Uniform1f(self.scale_idx, scale);
            gl::DrawElements(gl::LINES, 6, gl::UNSIGNED_BYTE, ptr::null::<c_void>() as *const c_void);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for AxisIndicator {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.array_buffer);
            gl::DeleteBuffers(1, &self.element_array_buffer);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}