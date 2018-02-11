//use std::marker::PhantomData;
use gl;
use gl::types::{GLuint, GLenum, GLintptr, GLsizeiptr, GLvoid};

// The type parameter T can be either Attributes or Indices.
pub struct Vbo<T> {
  id: GLuint,
  initialized: bool,
  target: T
}

pub struct Attributes;
pub struct Indices;
pub trait Target {
  fn new() -> Self;
  fn as_gl_enum(&self) -> GLenum;
}
impl Target for Attributes {
  fn new() -> Attributes { Attributes }
  fn as_gl_enum(&self) -> GLenum { gl::ARRAY_BUFFER }
}
impl Target for Indices {
  fn new() -> Indices { Indices }
  fn as_gl_enum(&self) -> GLenum { gl::ELEMENT_ARRAY_BUFFER }
}

impl <T: Target> Vbo<T> {
  fn new() -> Vbo<T> {
    let mut vbo = Vbo {
      id: 0,
      initialized: false,
      target: T::new()
    };
    unsafe { gl::GenBuffers(1, &mut vbo.id); }
    vbo
  }
  
  fn translate_target(&self) -> GLenum {
    self.target.as_gl_enum()
  }

  pub unsafe fn bind(&self) {
    gl::BindBuffer(self.translate_target(), self.id);
  }

  pub unsafe fn unbind(&self) {
    gl::BindBuffer(self.translate_target(), 0);
  }
  
  pub fn buffer_data<D>(&mut self, size: usize, data: &Vec<D>, usage: GLenum) {
    unsafe {
      self.bind();
      gl::BufferData(
        self.translate_target(),
        size as GLsizeiptr,
        data.as_ptr() as *const GLvoid,
        usage
      );
      self.unbind();
    }
    self.initialized = true;
  }
  
  pub fn buffer_sub_data<D>(&mut self, offset: usize, size: usize, data: &Vec<D>) {
    if !self.initialized {
      panic!("VBO not initialized. Must call buffer_data before buffer_sub_data.");
    }
    unsafe {
      self.bind();
      gl::BufferSubData(
        self.translate_target(),
        offset as GLintptr,
        size as GLsizeiptr,
        data.as_ptr() as *const GLvoid
      );
      self.unbind();
    }
  }
}

impl <T> Drop for Vbo<T> {
  fn drop(&mut self) {
    unsafe { gl::DeleteBuffers(1, &self.id); }
  }
}