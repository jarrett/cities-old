use cgmath::{Point, Point3, Line3, Vector, EuclideanVector, Vector3};

// A line in parametric form.
#[derive(Debug, PartialEq)]
pub struct PLine3 {
    pub ori: Vector3<f32>,
    pub dir: Vector3<f32>
}

impl PLine3 {
    pub fn new(p1: &Point3<f32>, p2: &Point3<f32>) -> PLine3 {
        PLine3 {
            ori: p1.to_vec(),
            dir: Vector3::new(
                p2.x - p1.x,
                p2.y - p1.y,
                p2.z - p1.z
            )
        }
    }
    
    #[allow(dead_code)]
    pub fn at(&self, t: f32) -> Point3<f32> {
        Point3 {x: self.x(t), y: self.y(t), z: self.z(t)}
    }
    
    #[allow(dead_code)]
    pub fn x(&self, t: f32) -> f32 {
        self.dir.x * t + self.ori.x
    }
    
    #[allow(dead_code)]
    pub fn y(&self, t: f32) -> f32 {
        self.dir.y * t + self.ori.y
    }
    
    #[allow(dead_code)]
    pub fn z(&self, t: f32) -> f32 {
        self.dir.z * t + self.ori.z
    }
    
    #[allow(dead_code)]
    pub fn normalize(&self) -> PLine3 {
        PLine3 { ori: self.ori, dir: self.dir.normalize() }
    }
    
    // Returns a t value.
    pub fn where_x_eq(&self, x: f32) -> Option<f32> {
        if self.dir.x == 0.0 {
            None
        } else {
            Some((x - self.ori.x) / self.dir.x)
        }
    }
    
    // Returns a t value.
    pub fn where_y_eq(&self, y: f32) -> Option<f32> {
        if self.dir.y == 0.0 {
            None
        } else {
            Some((y - self.ori.y) / self.dir.y)
        }
    }
    
    // Returns a t value.
    pub fn where_z_eq(&self, z: f32) -> Option<f32> {
        if self.dir.z == 0.0 {
            None
        } else {
            Some((z - self.ori.z) / self.dir.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Point3;
    use super::PLine3;
    
    #[test]
    fn test_from_line3() {
        let l = PLine3::new(
            &Point3::new(-1.0,  2.5,  3.2),
            &Point3::new( 1.3, -8.4,  7.8),
        );
        assert_eq_f32!(  2.3, l.dir.x);
        assert_eq_f32!(-10.9, l.dir.y);
        assert_eq_f32!(  4.6, l.dir.z);
        assert_eq_f32!( -1.0, l.ori.x);
        assert_eq_f32!(  2.5, l.ori.y);
        assert_eq_f32!(  3.2, l.ori.z);
    }
    
    #[test]
    fn test_at() {
        let l = PLine3::new(
            &Point3::new(-1.0,  2.5,  3.2),
            &Point3::new( 1.3, -8.4,  7.8),
        );
        let p: Point3<f32> = l.at(2.7);
        assert_eq_f32!(  5.21, p.x);
        assert_eq_f32!(-26.93, p.y);
        assert_eq_f32!( 15.62, p.z);
    }
    
    #[test]
    fn test_where_x_eq() {
        let l = PLine3::new(
            &Point3::new(2.0, 0.0, 0.0),
            &Point3::new(3.5, 1.0, 2.0),
        );
        assert_eq!(14.0, l.where_x_eq(23.0).unwrap());
        
        // This line never intersects x = 2.
        let l = PLine3::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(0.0, 1.0, 2.0),
        );
        assert_eq!(None, l.where_x_eq(1.0));
    }
    
    #[test]
    fn test_where_y_eq() {
        let l = PLine3::new(
            &Point3::new(0.0, 2.0, 0.0),
            &Point3::new(1.0, 3.5, 2.0),
        );
        assert_eq!(14.0, l.where_y_eq(23.0).unwrap());
        
        // This line never intersects x = 2.
        let l = PLine3::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(1.0, 0.0, 2.0),
        );
        assert_eq!(None, l.where_y_eq(1.0));
    }
    
    #[test]
    fn test_where_z_eq() {
        let l = PLine3::new(
            &Point3::new(0.0, 0.0, 2.0),
            &Point3::new(2.0, 1.0, 3.5),
        );
        assert_eq!(14.0, l.where_z_eq(23.0).unwrap());
        
        // This line never intersects x = 2.
        let l = PLine3::new(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(2.0, 1.0, 0.0),
        );
        assert_eq!(None, l.where_z_eq(1.0));
    }
}