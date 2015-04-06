use std::rc::Rc;
use std::cmp::Ordering;

use thing::Thing;
use camera::Camera;

pub struct ZSorted {
    by_orbit: Vec<Vec<Rc<Thing>>>
}

impl ZSorted {
    // Retrieves the appropriate Z-sorted list based on the camera's orbit.
    pub fn get(&self, camera: &Camera) -> &Vec<Rc<Thing>> {
        &self.by_orbit[camera.orbit as usize]
    }
    
    pub fn new(things: &Vec<Rc<Thing>>, camera: &mut Camera) -> ZSorted {
        let mut z_sorted = ZSorted { by_orbit: Vec::with_capacity(4) };
        z_sorted.rebuild(things, camera);
        z_sorted
    }
    
    // Pass in a list of all existing things, and this method will create a Z-sorted
    // list for each viewing direction.
    pub fn rebuild(&mut self, things: &Vec<Rc<Thing>>, camera: &mut Camera) {
        // We temporarily cycle through the camera's possible orbits. We'll restore the
        // original orbit at the end.
        let orig_orbit = camera.orbit;
        self.by_orbit.clear();
        for orbit in 0u8..4u8 {
            let mut new_vec: Vec<Rc<Thing>> = things.iter().map(|thing_rc| {
                thing_rc.clone()
            }).collect();
            camera.orbit_to(orbit);
            new_vec.sort_by(|a, b| {
              let da = camera.distance_to(&a.position);
              let db = camera.distance_to(&b.position);
              db.partial_cmp(&da).unwrap_or(Ordering::Equal)
            });
            self.by_orbit.push(new_vec);
        }
        camera.orbit_to(orig_orbit);
    }
}