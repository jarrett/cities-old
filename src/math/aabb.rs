use cgmath::{Aabb, Aabb3, Point3, Triangle, Ray3};

use super::{in_interval, min_opts, max_opts};

pub fn aabb3_contains_aabb3(outer: &Aabb3<f32>, inner: &Aabb3<f32>) -> bool {
    if outer.min.x > inner.min.x { return false; }
    if outer.min.y > inner.min.y { return false; }
    if outer.min.z > inner.min.z { return false; }
    
    if outer.max.x < inner.max.x { return false; }
    if outer.max.y < inner.max.y { return false; }
    if outer.max.z < inner.max.z { return false; }
    
    true
}

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

pub fn aabb3_from_tris(tri1: &Triangle<Point3<f32>>, tri2: &Triangle<Point3<f32>>) -> Aabb3<f32> {
    let min_x: f32 = tri1.p0.x.min(tri1.p1.x).min(tri1.p2.x).min(tri2.p0.x).min(tri2.p1.x).min(tri2.p2.x);
    let min_y: f32 = tri1.p0.y.min(tri1.p1.y).min(tri1.p2.y).min(tri2.p0.y).min(tri2.p1.y).min(tri2.p2.y);
    let min_z: f32 = tri1.p0.z.min(tri1.p1.z).min(tri1.p2.z).min(tri2.p0.z).min(tri2.p1.z).min(tri2.p2.z);
    
    let max_x: f32 = tri1.p0.x.max(tri1.p1.x).max(tri1.p2.x).max(tri2.p0.x).max(tri2.p1.x).max(tri2.p2.x);
    let max_y: f32 = tri1.p0.y.max(tri1.p1.y).max(tri1.p2.y).max(tri2.p0.y).max(tri2.p1.y).max(tri2.p2.y);
    let max_z: f32 = tri1.p0.z.max(tri1.p1.z).max(tri1.p2.z).max(tri2.p0.z).max(tri2.p1.z).max(tri2.p2.z);
    
    Aabb3::new(Point3::new(min_x, min_y, min_z), Point3::new(max_x, max_y, max_z))
}

pub fn ray3_intersects_aabb3(ray: &Ray3<f32>, bb: &Aabb3<f32>) -> bool {
    // Special case: The ray is parallel to one of the axes. In that case, the question
    // is whether the ray's constant value on that axis is inside or outside the slab.
    // (The slab is the set of points bounded by the box's min and max values
    // on that axis.)
    if ray.direction.x == 0.0 && !in_interval(ray.origin.x, bb.min.x, bb.max.x) { return false; }
    if ray.direction.y == 0.0 && !in_interval(ray.origin.y, bb.min.y, bb.max.y) { return false; }
    if ray.direction.z == 0.0 && !in_interval(ray.origin.z, bb.min.z, bb.max.z) { return false; }
    
    // We couldn't rule out an intersection based on the ray being parallel to an axis.
    // Find the t values where the ray intersects each plane of the bounding box. These
    // pairs of values define axis-aligned slabs.
    
    // x planes.
    let t1x: Option<f32> = ray.where_x_eq(bb.min.x);
    let t2x: Option<f32> = ray.where_x_eq(bb.max.x);
    
    // y planes.
    let t1y: Option<f32> = ray.where_y_eq(bb.min.z);
    let t2y: Option<f32> = ray.where_y_eq(bb.max.z);
    
    // z planes.
    let t1z: Option<f32> = ray.where_z_eq(bb.min.z);
    let t2z: Option<f32> = ray.where_z_eq(bb.max.z);
    
    // For each axis-aligned slab, find the interval of t values where the ray is within
    // the slab. We already computed the t values above. Now we just have to decide which
    // is the min and which is the max.
    
    // x slab (set of points where box.min.x <= x <= box.max.x).
    let t_min_x: Option<f32> = min_opts(t1x, t2x);
    let t_max_x: Option<f32> = max_opts(t1x, t2x);
    
    // y slab (set of points where box.min.y <= y <= box.max.y).
    let t_min_y: Option<f32> = min_opts(t1y, t2y);
    let t_max_y: Option<f32> = max_opts(t1y, t2y);
    
    // z slab (set of points where box.min.y <= y <= box.max.y).
    let t_min_z: Option<f32> = min_opts(t1z, t2z);
    let t_max_z: Option<f32> = max_opts(t1z, t2z);
    
    // We now have an interval of t values for each axis-aligned slab. If these intervals
    // overlap, then the ray intersects the box.
    //    
    // Find the min and max t values for the overlapping interval. We can unwrap because
    // the ray can't be parallel to more than one axis, and therefore at least 2 of the 3
    // values are Some.
    let t_min: f32 = max_opts(max_opts(t_min_x, t_min_y), t_min_z).unwrap();
    let t_max: f32 = min_opts(min_opts(t_max_x, t_max_y), t_max_z).unwrap();
    
    // If there is some overlap between all three intervals, then the overall t_min will
    // be less than or equal to the overall t_max. If so, then we have an intersection.
    t_min <= t_max
}

#[cfg(test)]
mod tests {
    use cgmath::{Point3, Aabb3, Ray3};
    use super::ray3_intersects_aabb3;
    
    #[test]
    fn test_ray3_intersects_aabb3() {
        let bb = Aabb3::new(
            Point3::new(-1.0, -2.0, -3.0),
            Point3::new( 4.0,  5.0,  6.0)
        );
        
        // Ray is not parallel to any of the three axes. Does intersect.
        let ray = Ray3::from_points(
            Point3::new(-1.0, -4.0, -3.0),
            Point3::new( 3.5,  5.0,  8.0)
        );
        assert!(ray3_intersects_aabb3(&ray, &bb));
        
        // Ray is not parallel to any of the three axes. Does not intersect.
        let ray = Ray3::from_points(
            Point3::new(-4.0, -5.0, -3.0),
            Point3::new(-1.0,  7.0,  8.0)
        );
        assert!(!ray3_intersects_aabb3(&ray, &bb));
        
        // Ray is parallel to the x axis, and its x value is within the box's x interval.
        // Does intersect.
        let ray = Ray3::from_points(
            Point3::new( 0.0, -5.0, -3.0),
            Point3::new( 0.0,  7.0,  8.0)
        );
        assert!(ray3_intersects_aabb3(&ray, &bb));
        
        // Ray is parallel to the x axis, and its x value is outside the box's x interval.
        // Does not intersect.
        let ray = Ray3::from_points(
            Point3::new( 5.0, -5.0, -3.0),
            Point3::new( 5.0,  7.0,  8.0)
        );
        assert!(!ray3_intersects_aabb3(&ray, &bb));
    }
}