use std::ops::Neg;
use cgmath::{Rad, rad, Point, Point2, Point3, Vector2, Vector4, Matrix, Matrix3, Matrix4, Ortho, Ray3};

// 28 degrees on the Z axis.
static CAMERA_TILT: Rad<f32> = Rad { s: 3.97935069f32 };

// 228 degrees (or 48 degrees, in Blender terms) on the X axis.
static CAMERA_ORBIT: Rad<f32> = Rad { s: 0.488692191f32 };

pub struct Camera {
    pub z_rotation: Rad<f32>,
    pub orbit: u8,
    pub focus: Vector2<f32>,
    pub zoom: f32,
    pub transform: Matrix4<f32>,
    pub inverse: Matrix4<f32>,
    pub width: u16,
    pub height: u16
}

impl Camera {
    pub fn new(width: u16, height: u16, zoom: f32) -> Camera {
        let mut cam = Camera {
            z_rotation: rad(0f32), orbit: 3, focus: Vector2 {x: 0f32, y: 0f32},
            zoom: zoom, transform: Matrix4::identity(), inverse: Matrix4::identity(),
            width: width, height: height
        };
        cam.orbit_to(0); // Rebuilds model-view.
        cam.rebuild_matrices();
        cam
    }
    
    pub fn decrement_orbit(&mut self) {
        if self.orbit == 0 {
            self.orbit_to(3);
        } else {
            let orbit = self.orbit;
            self.orbit_to(orbit - 1);
        }
    }
    
    pub fn distance_to(&self, point: &Point3<f32>) -> f32 {
        self.transform.mul_v(&point.to_vec().extend(1.0)).z
    }
    
    pub fn increment_orbit(&mut self) {
        if self.orbit == 3 {
            self.orbit_to(0);
        } else {
            let orbit = self.orbit;
            self.orbit_to(orbit + 1);
        }
    }
    
    pub fn orbit_to(&mut self, dir: u8) {
        self.orbit = dir;
        self.z_rotation = CAMERA_ORBIT + rad(dir as f32 * 1.57079632679f32);
        self.rebuild_matrices();
    }
    
    // amount is in screen space.
    pub fn pan(&mut self, amount: &Vector2<f32>) {
        let amount_4 = amount.extend(0.0).extend(0.0);
        let amount_2 = self.inverse.mul_v(&amount_4).truncate().truncate();
        self.focus = self.focus + amount_2;
        self.rebuild_matrices();
    }
    
    // Converts a point in window space to a line in world space.
    // 
    // p is measured in pixels, where the upper left corner of the window is (0, 0) and
    // the lower right is (self.width, self.height).
    pub fn unproject(&self, p: Point2<f32>) -> Ray3<f32> {
        // Convert to OpenGL clip space, i.e. [-1, 1].
        let v1: Vector4<f32> = Vector4::new(
            p.x *  2.0 / self.width  as f32 - 1.0,
            p.y * -2.0 / self.height as f32 + 1.0,
            -1.0,
            1.0
        );
        
        let mut v2: Vector4<f32> = v1.clone();
        v2.z = 1.0;
        // Just temporary so we can see the debug line.
        v2.x = v1.x + 0.1;
        
        Ray3::from_points(
            Point3::from_vec(&self.inverse.mul_v(&v1).truncate()),
            Point3::from_vec(&self.inverse.mul_v(&v2).truncate())
        )
    }
    
    pub fn zoom(&self) -> f32 {
        self.zoom
    }
    
    pub fn zoom_by(&mut self, multiplier: f32) {
        self.zoom = self.zoom * multiplier;
        self.rebuild_matrices();
    }
    
    
    pub fn resize(&mut self, width: u16, height: u16) {
        if width != self.width || height != self.height {
            self.width = width;
            self.height = height;
            self.rebuild_matrices();
        }
    }
    
    fn rebuild_matrices(&mut self) {
        // Remember that transformations are applied in reverse order.
        
        // Orthographic projection.
        self.transform = Matrix4::from(Ortho {
            left:   self.width  as f32 / (-1.0 * self.zoom),
            right:  self.width  as f32 /         self.zoom ,
            bottom: self.height as f32 / (-1.0 * self.zoom),
            top:    self.height as f32 /         self.zoom ,
            near:    200.0,
            far:    -200.0
        });
        
        // X-rotate model.
        self.transform.mul_self_m(
          &Matrix4::from(Matrix3::from_angle_x(CAMERA_TILT))
        );
    
        // Z-rotate model.
        self.transform.mul_self_m(
            &Matrix4::from(Matrix3::from_angle_z(self.z_rotation))
        );
        
        // Translate model.
        self.transform.mul_self_m(
            &Matrix4::from_translation(&self.focus.neg().extend(0.0))
        );
        
        unsafe {
            self.inverse = self.transform.invert_unsafe();
        }
    }
}