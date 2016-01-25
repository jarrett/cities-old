mod program;
mod allocator;
mod element;
mod widget;
mod button;

use std::iter::repeat;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem::size_of;
use std::ptr;
use libc::{c_void};
use gl;
use gl::types::*;

use opengl::{Vao, Vbo, Attributes, Indices};

pub use self::program::Program;
pub use self::widget::Widget;
pub use self::button::Button;
use self::element::Element;
use self::allocator::Allocator;

// The UI is a tree of Elements. Each Element is a rectangle with position and size. An
// Element may own a Widget, which represents something specific like a button, a bit of
// static text, etc. (Widget is a trait.) An Element need not own a Widget; it can serve
// as an invisible parent to other Elements.
// 
// From calling code, we don't usually construct an Element directly. Instead, we use the
// element() method on Widget classes, e.g. Button::element(). This constructs the Element
// and the Widget
// 
// There's only one vertex attribute buffer for the entire UI. It contains all the
// rectangles for Elements that are currently in scope. I.e. if a Rectangle exists, it's
// in the buffer, regardless of whether it's visible. We maintain an allocator--a map
// of the buffer that tracks which slots are in use. When a Rectangle is constructed, we
// find an available slot in the map and claim it, marking it as as used. When a rectangle
// is dropped, we mark its slot as free.

// The Vbo is sized to accomodate this many rectangles.
const BUFFER_SIZE: usize = 256;

pub struct Ui {
    elements: Vec<Element>,
    program: Program,
    vao: Vao,
    attr_buffer: Vbo,
    index_buffer: Vbo,
    allocator: Rc<RefCell<Allocator>>
}

impl Ui {
    pub fn new() -> Ui {
        unsafe {
            let mut ui = Ui {
                elements: Vec::new(),
                program: Program::new(),
                allocator: Rc::new(RefCell::new(Allocator::new(BUFFER_SIZE))),
                vao: Vao::new(),
                index_buffer: Vbo::new(Indices),
                attr_buffer: Vbo::new(Attributes)
            };
            
            ui.configure_vao();
            
            // We initialize the index and attribute buffer to zeroes.
            
            // Size of the attribute buffer in bytes. Each rectangle has four vertices of
            // the form (x, y, u, v). Each value is a GLfloat.
            let attr_buffer_size: usize = 4 * 4 * size_of::<GLfloat>() * BUFFER_SIZE;
            let attr_zeroes: Vec<GLfloat> = repeat(0.0).take(attr_buffer_size).collect();
            ui.attr_buffer.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                attr_buffer_size as i64,
                attr_zeroes.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            Vbo::unbind(Attributes);
            
            // Size of the index buffer in bytes. Each rectangle has two triangles. Each
            // index is a GLushort.
            let index_buffer_size: usize = 2 * 3 * size_of::<GLushort>() * BUFFER_SIZE;
            let index_zeroes: Vec<GLushort> = repeat(0).take(index_buffer_size).collect();
            ui.index_buffer.bind();
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                index_buffer_size as i64,
                index_zeroes.as_ptr() as *const c_void,
                gl::STATIC_DRAW
            );
            Vbo::unbind(Indices);
            
            ui
        }
    }
    
    pub fn draw(&self) {
        unsafe {
            self.vao.bind();
            self.index_buffer.bind();
            gl::UseProgram(self.program.p.id);
            for element in self.elements.iter() {
                element.draw();
            }
            gl::UseProgram(0);
            Vbo::unbind(Indices);
            Vao::unbind();
        }
    }
    
    fn configure_vao(&mut self) {
        unsafe {
            self.vao.bind();
            
            self.vao.attrib(
                &self.attr_buffer,
                self.program.position_idx, 2, gl::FLOAT, 0, 0
            );
            
            self.vao.attrib(
                &self.attr_buffer,
                self.program.uv_idx, 2, gl::FLOAT, 12, 12
            );
            
            Vao::unbind();
        }
    }
}