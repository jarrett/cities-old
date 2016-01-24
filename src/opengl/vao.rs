use gl;
use gl::types::{GLuint};

pub struct Vao {id: GLuint}

impl Vao {
    pub fn new() -> Vao {
        let mut vao = Vao {id: 0};
        unsafe { gl::GenVertexArrays(1, &mut vao.id); }
        vao
    }
    
    pub fn unbind() {
        unsafe { gl::BindVertexArray(0); }
    }
    
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id); }
    }
}