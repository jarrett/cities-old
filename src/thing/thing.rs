use std::rc::Rc;
use cgmath::*;

use super::MetaThing;
use model::Model;

pub struct Thing {
    position: Vector3<f32>,
    direction: u8,
    meta_thing: Rc<MetaThing>,
    models: Vec<Model>
}

impl Thing {
    pub fn new(meta_thing: &Rc<MetaThing>, position: &Vector3<f32>, direction: u8) -> Thing {
        let models: Vec<Model> = meta_thing.models().iter().map( |model_inclusion| {
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
    
    pub fn models(&self) -> &Vec<Model> {
        &self.models
    }
}
