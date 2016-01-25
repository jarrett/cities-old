use gl::types::*;

use super::Program;
use super::Widget;

pub struct Element {
    // Position relative to top-left of parent in pixels.
    x: i32,
    y: i32,
    
    // Size in pixels.
    w: i32,
    h: i32,
    
    widget: Option<Box<Widget>>,
    children: Vec<Element>
}

impl Element {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Element {
        Element {x: x, y: y, w: w, h: h, widget: None, children: Vec::new()}
    }
    
    // Be sure to bind the program, Vao, and index buffer first.
    pub unsafe fn draw(&self) {
        //self.widget.map(|w| w.draw());
        for child in self.children.iter() {
            child.draw();
        }
    }
}