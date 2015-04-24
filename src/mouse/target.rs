use cgmath::{Aabb3, Point3};

use math::{PLine3, Triangle};

pub enum Target {
    Ground(Aabb3<f32>, Triangle, Triangle),
    //Water(Aabb3<f32>),
    //Thing(Aabb3<f32>, Rc<RefCell<Thing>>)
}

pub struct Hit<'a> {
    target: &'a Target,
    at: Point3<f32>
}

impl Target {
    pub fn bb<'a>(&'a self) -> &'a Aabb3<f32> {
        match self {
            &Target::Ground(ref bb, _, _) => { bb },
            //&Target::Water(ref bb)      => { bb },
            //&Target::Thing(ref bb, _)   => { bb }
        }
    }
    
    pub fn intersects_pline3(&self, line: &PLine3) -> Option<Hit> {
        match self {
            &Target::Ground(ref bb, ref tri1, ref tri2) => {
                None
            },
            //&Target::Water(ref bb)      => { bb },
            //&Target::Thing(ref bb, _)   => { bb }
        }
    }
}