use libc::c_void;
use gl;
use gl::types::{GLuint, GLenum};

pub struct Vbo {id: GLuint, target: GLenum}

pub enum Target {Attributes, Indices}

impl Vbo {
    pub fn new(target: Target) -> Vbo {
        let mut vbo = Vbo {
            id: 0,
            target: Vbo::translate_target(target)
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
                size as i64,
                data.as_ptr() as *const c_void,
                usage
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

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}