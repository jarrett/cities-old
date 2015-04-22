use cgmath::{Aabb, Aabb3, Point3};

// Splits into equal quadrants.
pub fn split_aabb3_for_quadtree(bb: &Aabb3<f32>) -> (Aabb3<f32>, Aabb3<f32>, Aabb3<f32>, Aabb3<f32>) {
    let mut center = bb.center();
    center.z = bb.min.z;
    (
        Aabb3::new(Point3::new(bb.max.x, bb.max.y, bb.max.z), center.clone()), // +x, +y
        Aabb3::new(Point3::new(bb.min.x, bb.max.y, bb.max.z), center.clone()), // -x, +y
        Aabb3::new(Point3::new(bb.min.x, bb.min.y, bb.max.z), center.clone()), // -x, -y
        Aabb3::new(Point3::new(bb.max.x, bb.min.y, bb.max.z), center        )  // +x, -y
    )
}

pub fn aabb3_contains_aabb3(outer: &Aabb3<f32>, inner: &Aabb3<f32>) -> bool {
    if outer.min.x > inner.min.x { return false; }
    if outer.min.y > inner.min.y { return false; }
    if outer.min.z > inner.min.z { return false; }
    
    if outer.max.x < inner.max.x { return false; }
    if outer.max.y < inner.max.y { return false; }
    if outer.max.z < inner.max.z { return false; }
    
    true
}