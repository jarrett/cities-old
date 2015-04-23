use cgmath::{Aabb, Aabb3, Point3};

use super::Triangle;

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

pub fn aabb3_from_tris(tri1: &Triangle, tri2: &Triangle) -> Aabb3<f32> {
    let &(p1, p2, p3): &(Point3<f32>, Point3<f32>, Point3<f32>) = tri1;
    let &(p4, p5, p6): &(Point3<f32>, Point3<f32>, Point3<f32>) = tri2;
    let min_x: f32 = p1.x.min(p2.x).min(p3.x).min(p4.x).min(p5.x).min(p6.x);
    let min_y: f32 = p1.y.min(p2.y).min(p3.y).min(p4.y).min(p5.y).min(p6.y);
    let min_z: f32 = p1.z.min(p2.z).min(p3.z).min(p4.z).min(p5.z).min(p6.z);
    
    let max_x: f32 = p1.x.max(p2.x).max(p3.x).max(p4.x).max(p5.x).max(p6.x);
    let max_y: f32 = p1.y.max(p2.y).max(p3.y).max(p4.y).max(p5.y).max(p6.y);
    let max_z: f32 = p1.z.max(p2.z).max(p3.z).max(p4.z).max(p5.z).max(p6.z);
    
    Aabb3::new(Point3::new(min_x, min_y, min_z), Point3::new(max_x, max_y, max_z))
}