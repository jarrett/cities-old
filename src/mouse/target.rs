use std::rc::Rc;
use std::cell::RefCell;
use cgmath::*;

use world::World;
use camera::Camera;
use thing::Thing;

enum Target {
    GroundTarget(Aabb3<f32>, Triangle),
    WaterTarget(Vector3<f32>, Vector3<f32>),
    ThingTarget(Aabb3<f32>, Rc<RefCell<Thing>>)
}