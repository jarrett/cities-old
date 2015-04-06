use std::rc::Rc;
use cgmath::*;

use super::MetaThing;
use model::{Model, Buffers, Program3d};
use camera::Camera;


pub struct Thing {
    pub position: Vector3<f32>,
    pub direction: u8,
    pub meta_thing: Rc<MetaThing>,
    pub models: Vec<Model>
}

impl Thing {
    pub fn new(meta_thing: &Rc<MetaThing>, position: &Vector3<f32>, direction: u8) -> Thing {
        let models: Vec<Model> = meta_thing.models().iter().map( |model_inclusion| {
            // FIXME: Multiply model_inclusion.offset by a rotation matrix representing
            // the thing's direction.
            Model::new(
                position, &model_inclusion.offset,
                (model_inclusion.direction + direction) % 8,
                &model_inclusion.meta_model
            )
        }).collect();
        Thing {
            position: position.clone(),
            direction: direction,
            meta_thing: meta_thing.clone(),
            models: models
        }
    }
    
    pub fn draw(&self, program: &Program3d, buffers: &Buffers, camera: &Camera) {
        for model in self.models.iter() {
            model.draw(program, buffers, camera);
        }
    }
}
