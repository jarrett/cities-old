use cgmath::{Vector, Vector3, Point, Point3};

use super::PLine3;

pub type Triangle = (Point3<f32>, Point3<f32>, Point3<f32>);

// https://www.cs.virginia.edu/~gfx/Courses/2003/ImageSynthesis/papers/Acceleration/Fast%20MinimumStorage%20RayTriangle%20Intersection.pdf
pub fn pline3_intersects_triangle(line: &PLine3, tri: &Triangle) -> Option<Point3<f32>> {
    let &(vert0, vert1, vert2) = tri;
    let edge1: Vector3<f32> = vert1.sub_p(&vert0);
    let edge2: Vector3<f32> = vert2.sub_p(&vert0);
    
    let pvec: Vector3<f32> = line.dir.cross(&edge2);
    let det: f32 = edge1.dot(&pvec);
    
    if det > -0.000001 && det < 0.000001 { return None; }
    let inv_det: f32 = 1.0 / det;
    
    let tvec: Vector3<f32> = line.ori.sub_v(&vert0.to_vec());
    
    let u: f32 = tvec.dot(&pvec) * inv_det;
    if u < 0.0 || u > 1.0 { return None; }
    
    let qvec: Vector3<f32> = tvec.cross(&edge1);
    
    let v: f32 = line.dir.dot(&qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 { return None; }
    
    let t: f32 = edge2.dot(&qvec) * inv_det;
    Some(line.at(t))
}

#[cfg(test)]
mod tests {
    use cgmath::Point3;
    use super::super::PLine3;
    use super::pline3_intersects_triangle;
    
    #[test]
    fn test_pline3_intersects_triangle() {
        let tri = (
            Point3::new( 0.4, -1.8, -1.5),
            Point3::new(-0.1,  1.0,  1.2),
            Point3::new(-2.0, -1.0, -0.2)
        );
        
        let line = PLine3::new(
            &Point3::new( 0.4, -2.3,  2.3),
            &Point3::new(-1.2, -0.2, -1.8)
        ).normalize();
        
        let inter = pline3_intersects_triangle(&line, &tri).unwrap();
        assert_eq_f32!(-0.663337, inter.x);
        assert_eq_f32!(-0.904370, inter.y);
        assert_eq_f32!(-0.424801, inter.z);
        
        let line = PLine3::new(
            &Point3::new( 0.4, -2.3,  2.3),
            &Point3::new( 0.6, -0.2, -1.8)
        ).normalize();
        
        let inter = pline3_intersects_triangle(&line, &tri);
        assert_eq!(None, inter);
    }
}