use gl;
use gl::types::*;

use opengl::{Vbo, Texture2d, TextureConfig};
use super::Ui;
use super::program::Program;
use super::widget::Widget;
use super::allocator::Allocator;

// A single rectangle in the UI (similar to a DOM element in HTML). An Element
// may wrap a struct implemnting the Widget trait. The inner Widget could
// be anything, e.g. a button or a label.
// 
// Each Element can own any number of child Elements. However, an Element
// lacks a reference back to its parent. (Should we change this?)
// 
// See mod.rs for how the UI structs fit together.
pub struct Element {
    // Position relative to top-left of parent in pixels.
    x: i32,
    y: i32,
    
    // Size in pixels.
    w: i32,
    h: i32,
    
    // Slot index in VBOs. Byte and vertex indices can be derived from this.
    slot: usize,
    
    widget: Option<(
      Box<Widget + 'static>,
      Texture2d
    )>,
    children: Vec<Element>
}

impl Element {
    pub fn from_widget<T: Widget + 'static>(
        widget: T,
        ui: &mut Ui,
    ) -> Element {
        let slot = ui.allocator.alloc_slot();
        
        let mut texture = Texture2d::new(
            &TextureConfig {
                min_filter: gl::NEAREST, mag_filter: gl::NEAREST,
                wrap_s: gl::CLAMP_TO_EDGE, wrap_t: gl::CLAMP_TO_EDGE, max_level: 1
            },
            widget.w() as usize, widget.h() as usize
        );
        texture.upload(
            0, // Mipmap level.
            gl::RGBA, // Internal_format.
            gl::RGBA, // Input format.
            gl::UNSIGNED_BYTE, // Input type.
            &widget.texture_data(), // Buffer.
            false // Generate mipmaps.
        );
        
        let mut element = Element {
            x: widget.x(), y: widget.y(), w: widget.w(), h: widget.h(), slot: slot,
            widget: Some((
              Box::new(widget),
              texture
            )),
            children: Vec::new()
        };
        
        element.buffer(ui);
        element
    }
    
    pub fn add_widget<T: Widget + 'static>(&mut self, child: T, ui: &mut Ui) {
        self.children.push(
            Element::from_widget(child, ui)
        );
    }
    
    // Be sure to bind the program, Vao, and index buffer first.
    pub unsafe fn draw(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                6, // Number of indices to draw.
                gl::UNSIGNED_SHORT, // Format of index buffer.
                (self.slot * 6) as *const GLvoid // Offset into index buffer.
            );
        }
        for child in self.children.iter() {
            child.draw();
        }
    }
    
    fn buffer(&self, ui: &mut Ui) {
        let attr_data: Vec<GLint> = vec![
            self.x,           self.y,           // Top left.
            self.x + self.w,  self.y,           // Top right.
            self.x + self.w,  self.y + self.h,  // Bottom right.
            self.x,           self.y + self.h   // Bottom left.
        ];
        
        /*let attr_data: Vec<GLint> = vec![
            -1, -1, // Top left.
             1, -1, // Top right.
             1,  1, // Bottom right.
            -1,  1  // Bottom left.
        ];*/
        
        // 6 indices per slot.
        let offset: GLushort = self.slot as GLushort * 6;
        let index_data: Vec<GLushort> = vec![
            offset,     // Top left.
            offset + 3, // Bottom left.
            offset + 1, // Top right.
            
            offset + 3, // Bottom left.
            offset + 2, // Bottom right.
            offset + 1  // Top right.
        ];
        
        ui.attr_buffer.buffer_sub_data(
            // Offset into attributes buffer. 4 bytes per int x ints per vertex x
            // 4 vertices per slot = 32 bytes per slot.
            self.slot * 32,
            // Size of attribute data. 32 bytes per slot as explained above.
            32,
            &attr_data
        );
        
        ui.index_buffer.buffer_sub_data(
            // Offset into index buffer. 2 bytes per index x 6 indices per slot =
            // 12 bytes per slot.
            self.slot * 12,
            // Size of index data. 12 bytes per slot as explained above.
            12,
            &index_data
        );
    }
}