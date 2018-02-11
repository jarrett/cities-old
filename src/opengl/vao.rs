use std::ptr;
use gl;
use gl::types::{GLuint, GLint, GLsizei, GLenum, GLvoid};

use super::{Vbo, Attributes, Indices};

// The type parameter is for the attributes buffers. It could be, e.g, (Vbo, Vbo) or
// MyCustomVbos {positions: Vbo, uvs: Vbo}.
pub struct Vao {
  id: GLuint
}

impl Vao {
  pub fn new() -> Vao {
    let mut vao = Vao {id: 0};
    unsafe { gl::GenVertexArrays(1, &mut vao.id); }
    vao
  }
  
  pub unsafe fn unbind(&self) {
    gl::BindVertexArray(0);
  }
  
  pub unsafe fn bind(&self) {
    gl::BindVertexArray(self.id);
  }
  
  pub fn attrib(
    &self,
    vbo: &Vbo<Attributes>,
    attrib_idx: GLuint,
    size: usize,
    data_type: GLenum,
    stride: usize,
    pointer: usize
  ) {
    unsafe {
      vbo.bind();
      gl::EnableVertexAttribArray(attrib_idx);
      gl::VertexAttribPointer(
        attrib_idx,
        size as GLint,
        data_type,
        gl::FALSE,
        stride as GLsizei,
        pointer as *const GLvoid
      );
      vbo.unbind();
    }
  }
}

impl Drop for Vao {
  fn drop(&mut self) {
    unsafe { gl::DeleteVertexArrays(1, &self.id); }
  }
}