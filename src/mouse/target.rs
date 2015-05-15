use cgmath::{Aabb3, Point3, Ray3, Triangle, Intersect};

#[derive(Debug)]
pub enum Target {
    Ground(Aabb3<f32>, Triangle<Point3<f32>>, Triangle<Point3<f32>>),
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
    
    pub fn intersects_ray(&self, ray: &Ray3<f32>) -> Option<Hit> {
        let opt_point: Option<Point3<f32>> = match self {
            &Target::Ground(_, ref tri1, ref tri2) => {
                (ray, tri1).intersection().or(
                (ray, tri2).intersection())
            },
            //&Target::Water(ref bb)      => { bb },
            //&Target::Thing(ref bb, _)   => { bb }
        };
        
        opt_point.map(|point| {
            Hit { target: self, at: point }
        })
    }
}