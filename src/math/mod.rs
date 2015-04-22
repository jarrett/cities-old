use cgmath::Point3;

mod aabb;

pub use self::aabb::{split_aabb3_for_quadtree, aabb3_contains_aabb3};

pub type Triangle = (Point3<f32>, Point3<f32>, Point3<f32>);

pub type Quad = (Point3<f32>, Point3<f32>, Point3<f32>, Point3<f32>);

pub fn quad_to_tris(quad: Quad) -> (Triangle, Triangle) {
    let (p0, p1, p2, p3) = quad;
    (
        (p0, p1, p3),
        (p1, p2, p3)
    )
}