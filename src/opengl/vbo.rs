use std::marker::PhantomData;
use gl;
use gl::types::{GLuint, GLenum, GLintptr, GLsizeiptr, GLvoid};

// The type parameter T can be either Attributes or Indices.
pub struct Vbo<T> {
    id: GLuint,
    initialized: bool,
    target: PhantomData<T>
}

pub struct Attributes;
pub struct Indices;

impl Vbo<T> {
    pub fn new() -> Vbo<T> {
        let mut vbo = Vbo {
            id: 0,
            initialized: false
        };
        unsafe { gl::GenBuffers(1, &mut vbo.id); }
        vbo
    }
    
    pub unsafe fn unbind(target: Target) {
        gl::BindBuffer(Vbo::translate_target(target), 0);
    }
    
    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }
    
    pub fn buffer_data<T>(&mut self, size: usize, data: &Vec<T>, usage: GLenum) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
            gl::BufferData(
                self.target,
                size as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                usage
            );
            gl::BindBuffer(self.target, 0);
        }
        self.initialized = true;
    }
    
    pub fn buffer_sub_data<D>(&mut self, offset: usize, size: usize, data: &Vec<D>) {
        if !self.initialized {
            panic!("VBO not initialized. Must call buffer_data before buffer_sub_data.");
        }
        unsafe {
            gl::BindBuffer(self.target, self.id);
            gl::BufferSubData(
                self.target,
                offset as GLintptr,
                size as GLsizeiptr,
                data.as_ptr() as *const GLvoid
            );
            gl::BindBuffer(self.target, 0);
        }
    }
    
    fn translate_target(target: Target) -> GLenum {
        match target {
            Target::Attributes => gl::ARRAY_BUFFER,
            Target::Indices => gl::ELEMENT_ARRAY_BUFFER
        }
    }
}

impl Drop for Vbo<T> {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}