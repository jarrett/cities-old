use super::widget::Widget;

use opengl::checker;

pub struct Button {
    x: i32, y: i32,
    w: i32, h: i32,
    text: Option<String>
}

impl Button {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Button {
        Button {x: x, y: y, w: w, h: h, text: None}
    }
    
    pub fn text<T: ToString>(mut self, text: T) -> Button {
        self.text = Some(text.to_string());
        self
    }
}

impl Widget for Button {
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn w(&self) -> i32 { self.w }
    fn h(&self) -> i32 { self.h }
    
    fn texture_data(&self) -> Vec<u8> {
        checker(self.w as usize, self.h as usize, 20)
    }
}