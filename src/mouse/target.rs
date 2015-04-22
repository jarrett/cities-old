use std::rc::Rc;
use std::cell::RefCell;
use cgmath::*;

use world::World;
use camera::Camera;
use thing::Thing;
use math::Triangle;

enum Target {
    GroundTarget(Aabb3<f32>, Triangle),
    WaterTarget(Aabb3<f32>),
    ThingTarget(Aabb3<f32>, Rc<RefCell<Thing>>)
}

impl Target {
    pub fn bb(&self) -> &Aabb3<f32> {
        match *self {
            Target::GroundTarget(bb, _) => { &bb },
            Target::WaterTarget(bb)     => { &bb },
            Target::ThingTarget(bb, _)  => { &bb }
        }
    }
}