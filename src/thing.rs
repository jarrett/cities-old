use std::rc::Rc;

use cgmath::*;
use meta_thing::MetaThing;
use model::Model;

pub struct Thing {
    position: Vector3<f32>,
    meta_thing: Rc<MetaThing>,
    models: Vec<Model>
}

impl Thing {
    pub fn new(meta_thing: &Rc<MetaThing>, position: Vector3<f32>) -> Thing {
        Thing {
            position: position,
            meta_thing: meta_thing.clone(),
            models: Vec::new()
        }
    }
    
    pub fn models(&self) -> &Vec<Model> {
        &self.models
    }
}
