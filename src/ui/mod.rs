mod program;
mod allocator;
mod element;
mod widget;
mod button;

use std::iter::repeat;
use std::mem::size_of;
use std::ptr;
use gl;
use gl::types::*;

use opengl::{Vao, Vbo, Attributes, Indices};

pub use self::button::Button;
use self::program::Program;
use self::widget::Widget;
use self::element::Element;
use self::allocator::Allocator;

// The UI is a tree of Elements. Each Element is a rectangle with position and size. An
// Element may own a Widget, which represents something specific like a button, a bit of
// static text, etc. (Widget is a trait.) An Element need not own a Widget; it can serve
// as an invisible parent to other Elements.
// 
// There's only one attribute buffer and one index for the entire UI. They contain all the
// rectangles for Elements that are currently in scope. I.e. if an Element exists, it's
// in the buffer, regardless of whether it's visible. We maintain an allocator--a map
// of the buffer that tracks which slots are in use. When an Element is constructed, we
// find an available slot in the map and claim it, marking it as as used. When an Element
// is dropped, we mark its slot as free.
// 
// Each Element has its own Texture. (For now. This is inefficient, and we should design
// some kind of allocator for shared textures in the future.)

// The VBO is sized to accomodate this many rectangles.
const BUFFER_SIZE: usize = 256;

pub struct Ui {
    elements: Vec<Element>,
    program: Program,
    vao: Vao,
    position_buffer: Vbo<Attributes>,
    uv_buffer: Vbo<Attributes>,
    index_buffer: Vbo<Indices>,
    //allocator: Rc<RefCell<Allocator>>
    allocator: Allocator
}

impl Ui {
    pub fn new() -> Ui {
        let mut ui = Ui {
            elements: Vec::new(),
            program: Program::new(),
            //allocator: Rc::new(RefCell::new(Allocator::new(BUFFER_SIZE))),
            allocator: Allocator::new(BUFFER_SIZE),
            vao: Vao::new(),
            position_buffer: Vbo::new(),
            uv_buffer: Vbo::new(),
            index_buffer: Vbo::new()
        };
        
        ui.configure_vao();
        
        // We initialize the index and attribute buffer to zeroes.
        
        // Size of the attribute buffer in bytes. Each rectangle has four vertices of
        // the form (x, y, u, v). Each value is a GLfloat.
        let position_buffer_size: usize = 4 * 4 * size_of::<GLfloat>() * BUFFER_SIZE;
        let attr_zeroes: Vec<GLfloat> = repeat(0.0).take(position_buffer_size).collect();
        ui.position_buffer.buffer_data(position_buffer_size, &attr_zeroes, gl::STATIC_DRAW);
        
        // Size of the index buffer in bytes. Each rectangle has two triangles. Each
        // index is a GLushort.
        let index_buffer_size: usize = 2 * 3 * size_of::<GLushort>() * BUFFER_SIZE;
        let index_zeroes: Vec<GLushort> = repeat(0).take(index_buffer_size).collect();
        ui.index_buffer.buffer_data(index_buffer_size, &index_zeroes, gl::STATIC_DRAW);
        
        ui
    }
    
    pub fn add_widget<T: Widget + 'static>(&mut self, widget: T) {
        let element = Element::from_widget(widget, self);
        self.elements.push(element);
    }
    
    pub fn draw(&self, viewport_w: i32, viewport_h: i32) {
        unsafe {
            self.vao.bind();
            self.index_buffer.bind();
            gl::UseProgram(self.program.p.id);
            gl::Uniform2ui(self.program.viewport_size_idx, viewport_w as GLuint, viewport_h as GLuint);
            for element in self.elements.iter() {
                element.draw();
            }
            gl::UseProgram(0);
            Vbo::unbind();
            Vao::unbind();
        }
    }
    
    fn configure_vao(&mut self) {
        unsafe { self.vao.bind(); }
        
        // Position. 32-bit signed int.
        self.vao.attrib(
            &self.position_buffer,
            self.program.position_idx, 2, gl::INT, 0, 0
        );
        
        // UV coord. 32-bit float.
        self.vao.attrib(
            &self.uv_buffer,
            self.program.uv_idx, 2, gl::FLOAT, 0, 0
        );
        
        unsafe { Vao::unbind(); }
    }
}