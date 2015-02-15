use std::rc::Rc;
use cgmath::*;

use camera::Camera;
use meta_model::MetaModel;
use super::program3d::Program3d;
use super::buffers::Buffers;

pub struct Model {
    abs_position: Vector3<f32>, // World coords.
    rel_position: Vector3<f32>, // Relative to the owning Thing's origin.
    direction: u8,
    meta_model: Rc<MetaModel>
}

impl Model {
    pub fn new(thing_origin: &Vector3<f32>, offset: &Vector3<f32>, direction: u8, meta_model: &Rc<MetaModel>) -> Model {
        Model {
            abs_position: thing_origin.add_v(offset),
            rel_position: offset.clone(),
            direction: direction,
            meta_model: meta_model.clone()
        }
    }
    
    
    pub fn draw(&self, program: &Program3d, buffers: &Buffers, camera: &Camera) {
        self.meta_model.draw(program, buffers, camera, &self.abs_position, self.direction);
    }
}