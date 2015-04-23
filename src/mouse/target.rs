use cgmath::{Aabb3};

use math::Triangle;

pub enum Target {
    Ground(Aabb3<f32>, Triangle, Triangle),
    //Water(Aabb3<f32>),
    //Thing(Aabb3<f32>, Rc<RefCell<Thing>>)
}

impl Target {
    pub fn bb<'a>(&'a self) -> &'a Aabb3<f32> {
        match self {
            &Target::Ground(ref bb, _, _) => { bb },
            //&Target::Water(ref bb)      => { bb },
            //&Target::Thing(ref bb, _)   => { bb }
        }
    }
}