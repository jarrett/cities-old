mod program;
mod buffers;

use std::rc::Rc;
use cgmath::*;

use meta_model::MetaModel;
pub use self::program::Program;
pub use self::buffers::Buffers;

pub struct Model {
    abs_position: Vector3<f32>,
    rel_position: Vector3<f32>,
    meta_model: Rc<MetaModel>
}

impl Model {
    pub fn draw(&self, model_program: &Program) {
        self.meta_model.draw(model_program, self.abs_position);
    }
}