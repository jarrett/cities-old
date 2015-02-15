use std::rc::Rc;
use std::option::Option;

use cgmath::*;
use meta_thing::MetaThing;
use model::Model;

pub struct Thing {
    position: Vector3<f32>,
    meta_thing: Rc<MetaThing>,
    models: Vec<Model>
}

impl Thing {
    pub fn new(meta_thing: &Rc<MetaThing>, position: &Vector3<f32>) -> Thing {
        let models: Vec<Model> = meta_thing.models().iter().map( |model_inclusion| {
            Model::new(position, &model_inclusion.offset, model_inclusion.direction, &model_inclusion.meta_model)
        }).collect();
        Thing {
            position: position.clone(),
            meta_thing: meta_thing.clone(),
            models: models
        }
    }
    
    pub fn models(&self) -> &Vec<Model> {
        &self.models
    }
}
