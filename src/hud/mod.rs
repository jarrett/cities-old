mod program;
mod allocator;
mod element;

use std::iter::repeat;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem::size_of;
use std::ptr;
use libc::{c_void};
use gl;
use gl::types::*;

pub use self::program::Program;
use self::element::Element;
use self::allocator::Allocator;

// The HUD is a tree of Elements. Each Element is a rectangle with position and size.
// Optionally, a rectangle may have a texture. Or it may just be an invisible parent to
// other Elements.
// 
// There's only one vertex attribute buffer for the entire HUD. It contains all the
// rectangles for Elements that are currently in scope. I.e. if a Rectangle exists, it's
// in the buffer, regardless of whether it's visible. We maintain an allocator--a map
// of the buffer that tracks which slots are in use. When a Rectangle is constructed, we
// find an available slot in the map and claim it, marking it as as used. When a rectangle
// is dropped, we mark its slot as free.

// The VBO is sized to accomodate this many rectangles.
const BUFFER_SIZE: usize = 256;

struct HUD {
    elements: Vec<Element>,
    program: Program,
    vao: GLuint,
    attr_buffer: GLuint,
    index_buffer: GLuint,
    allocator: Rc<RefCell<Allocator>>
}

impl HUD {
    pub fn new() -> HUD {
        unsafe {
            let mut hud = HUD {
                elements: Vec::new(),
                program: Program::new(),
                allocator: Rc::new(RefCell::new(Allocator::new(BUFFER_SIZE))),
                vao: 0,
                index_buffer: 0,
                attr_buffer: 0
            };
            
            gl::GenBuffers(1, &mut hud.index_buffer);
            gl::GenBuffers(1, &mut hud.attr_buffer);
            hud.configure_vao();
            
            // We initialize the index and attribute buffer to zeroes.
            
            // Size of the attribute buffer in bytes. Each rectangle has four vertices of
            // the form (x, y, u, v). Each value is a GLfloat.
            let attr_buffer_size: usize = 4 * 4 * size_of::<GLfloat>() * BUFFER_SIZE;
            let attr_zeroes: Vec<GLfloat> = repeat(0.0).take(attr_buffer_size).collect();
            gl::BindBuffer(gl::ARRAY_BUFFER, hud.attr_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                attr_buffer_size as i64,
                attr_zeroes.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            
            // Size of the index buffer in bytes. Each rectangle has two triangles. Each
            // index is a GLushort.
            let index_buffer_size: usize = 2 * 3 * size_of::<GLushort>() * BUFFER_SIZE;
            let index_zeroes: Vec<GLushort> = repeat(0).take(index_buffer_size).collect();
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, hud.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                index_buffer_size as i64,
                index_zeroes.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            
            hud
        }
    }
    
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.attr_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.attr_buffer);
            gl::UseProgram(self.program.id);
            for element in self.elements.iter() {
                element.draw();
            }
            gl::UseProgram(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
    
    fn configure_vao(&mut self) {
        unsafe {
          gl::GenVertexArrays(1, &mut self.vao);
          gl::BindVertexArray(self.vao);
          gl::BindBuffer(gl::ARRAY_BUFFER, self.attr_buffer);
        
          gl::EnableVertexAttribArray(self.program.position_idx);
          gl::VertexAttribPointer(self.program.position_idx, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        
          gl::EnableVertexAttribArray(self.program.uv_idx);
          gl::VertexAttribPointer(self.program.uv_idx, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        
          gl::BindBuffer(gl::ARRAY_BUFFER, 0);
          gl::BindVertexArray(0);
        }
    }
}