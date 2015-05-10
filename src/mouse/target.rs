use cgmath::{Aabb3, Point3};

use math::{PLine3, Triangle, pline3_intersects_triangle};

#[derive(Debug)]
pub enum Target {
    Ground(Aabb3<f32>, Triangle, Triangle),
    //Water(Aabb3<f32>),
    //Thing(Aabb3<f32>, Rc<RefCell<Thing>>)
}

#[derive(Debug)]
pub struct Hit<'a> {
    pub target: &'a Target,
    pub at: Point3<f32>
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
        let opt_point: Option<Point3<f32>> = match self {
            &Target::Ground(ref bb, ref tri1, ref tri2) => {
                //pline3_intersects_triangle(&line, tri1).or(
                //pline3_intersects_triangle(&line, tri2))
                let (p1, _, _) = tri1.clone();
                Some(p1)
            },
            //&Target::Water(ref bb)      => { bb },
            //&Target::Thing(ref bb, _)   => { bb }
        };
        
        opt_point.map(|point| {
            Hit { target: self, at: point }
        })
    }
}