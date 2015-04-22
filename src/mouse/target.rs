use std::rc::Rc;
use std::cell::RefCell;
use cgmath::*;

use world::World;
use camera::Camera;
use thing::Thing;
use math::Triangle;

pub enum Target {
    GroundTarget(Aabb3<f32>, Triangle),
    WaterTarget(Aabb3<f32>),
    ThingTarget(Aabb3<f32>, Rc<RefCell<Thing>>)
}

impl Target {
    pub fn bb<'a>(&'a self) -> &'a Aabb3<f32> {
        match self {
            &Target::GroundTarget(ref bb, _) => { bb },
            &Target::WaterTarget(ref bb)     => { bb },
            &Target::ThingTarget(ref bb, _)  => { bb }
        }
    }
}