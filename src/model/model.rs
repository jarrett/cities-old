use std::rc::Rc;
use cgmath::*;

use camera::Camera;
use super::{MetaModel, Buffers, Program3d};

pub struct Model {
    pub abs_position: Vector3<f32>, // World coords.
    pub direction: u8,
    pub meta_model: Rc<MetaModel>
}

impl Model {
    pub fn new(thing_origin: &Vector3<f32>, offset: &Vector3<f32>, direction: u8, meta_model: &Rc<MetaModel>) -> Model {
        Model {
            abs_position: thing_origin.add_v(offset),
            direction: direction,
            meta_model: meta_model.clone()
        }
    }
    
    
    pub fn draw(&self, program: &Program3d, buffers: &Buffers, camera: &Camera) {
        self.meta_model.draw(program, buffers, camera, &self.abs_position, self.direction);
    }
}