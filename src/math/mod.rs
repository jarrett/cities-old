use std::cmp::PartialOrd;
use std::cmp::Ordering::*;
use cgmath::{Point3, Triangle};

mod aabb;

pub use self::aabb::{
    split_aabb3_for_quadtree, aabb3_contains_aabb3_xy, aabb3_from_tris,
    ray3_intersects_aabb3
};

pub type Quad = (Point3<f32>, Point3<f32>, Point3<f32>, Point3<f32>);

// Tests whether v is in the closed interval [min, max].
pub fn in_interval<T: PartialOrd>(v: T, min: T, max: T) -> bool {
    v >= min && v <= max
}

// Returns the minimum Some value. Returns None if both values are None.
pub fn min_opts<T: PartialOrd>(opt_a: Option<T>, opt_b: Option<T>) -> Option<T> {
    match (opt_a, opt_b) {
        (Some(a), Some(b)) => match a.partial_cmp(&b) {
            Some(Less)    => Some(a),
            Some(Greater) => Some(b),
            Some(Equal)   => Some(a),
            None => None
        },
        (Some(a), None)    => Some(a),
        (None,    Some(b)) => Some(b),
        (None,    None)    => None
    }
}

// Returns the maximum Some value. Returns None if both values are None.
pub fn max_opts<T: PartialOrd>(opt_a: Option<T>, opt_b: Option<T>) -> Option<T> {
    match (opt_a, opt_b) {
        (Some(a), Some(b)) => match a.partial_cmp(&b) {
            Some(Less)    => Some(b),
            Some(Greater) => Some(a),
            Some(Equal)   => Some(a),
            None => None
        },
        (Some(a), None)    => Some(a),
        (None,    Some(b)) => Some(b),
        (None,    None)    => None
    }
}

pub fn quad_to_tris(quad: Quad) -> (Triangle<Point3<f32>>, Triangle<Point3<f32>>) {
    let (p0, p1, p2, p3) = quad;
    (
        Triangle::new(p0, p1, p3),
        Triangle::new(p1, p2, p3)
    )
}

#[cfg(test)]
mod tests {
    use std::f32;
    use super::{in_interval, min_opts, max_opts};
    
    #[test]
    fn test_in_interval() {
        // v is in range.
        assert!(in_interval(2.0, 1.0, 3.0));
        
        // v is greater than max.
        assert!(!in_interval(4.0, 1.0, 3.0));
        
        // v is less than min.
        assert!(!in_interval(0.0, 1.0, 3.0));
        
        // min is greater than max.
        assert!(!in_interval(2.0, 3.0, 1.0));
        
        // v is NaN.
        assert!(!in_interval(f32::NAN, 1.0, 2.0));
        
        // min is NaN.
        assert!(!in_interval(1.0, f32::NAN, 2.0));
        
        // max is Nan.
        assert!(!in_interval(2.0, 1.0, f32::NAN));
    }
    
    #[test]
    fn test_min_opts() {
        // a < b.
        assert_eq!(Some(1.0), min_opts(Some(1.0), Some(2.0)));
        
        // b < a.
        assert_eq!(Some(1.0), min_opts(Some(2.0), Some(1.0)));
        
        // a is None.
        assert_eq!(Some(1.0), min_opts(None, Some(1.0)));
        
        // b is None.
        assert_eq!(Some(1.0), min_opts(Some(1.0), None));
        
        // a and b are None.
        assert_eq!(None, min_opts::<f32>(None, None));
    }
    
    #[test]
    fn test_max_opts() {
        // a < b.
        assert_eq!(Some(2.0), max_opts(Some(1.0), Some(2.0)));
        
        // b < a.
        assert_eq!(Some(2.0), max_opts(Some(2.0), Some(1.0)));
        
        // a is None.
        assert_eq!(Some(1.0), max_opts(None, Some(1.0)));
        
        // b is None.
        assert_eq!(Some(1.0), max_opts(Some(1.0), None));
        
        // a and b are None.
        assert_eq!(None, max_opts::<f32>(None, None));
    }
}