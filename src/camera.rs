use cgmath::{Rad, rad, Point, Point2, Point3, Vector2, Vector4, Matrix, Matrix3, Matrix4, ToMatrix4, Ortho};

use math::PLine3;

// 28 degrees on the Z axis.
static CAMERA_TILT: Rad<f32> = Rad { s: 3.97935069f32 };

// 228 degrees (or 48 degrees, in Blender terms) on the X axis.
static CAMERA_ORBIT: Rad<f32> = Rad { s: 0.488692191f32 };

pub struct Camera {
    pub z_rotation: Rad<f32>,
    pub orbit: u8,
    pub translation: Vector2<f32>,
    pub zoom: f32,
    pub model_view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
    pub inverse: Matrix4<f32>,
    pub width: u16,
    pub height: u16
}

impl Camera {
    pub fn new(width: u16, height: u16, zoom: f32) -> Camera {
        let mut cam = Camera {
            z_rotation: rad(0f32), orbit: 3, translation: Vector2 {x: 0f32, y: 0f32},
            zoom: zoom, model_view: Matrix4::identity(), projection: Matrix4::identity(),
            inverse: Matrix4::identity(), width: width, height: height
        };
        cam.orbit_to(0); // Rebuilds model-view.
        cam.rebuild_projection();
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
        let mvp = self.projection.mul_m(&self.model_view);
        mvp.mul_v(&point.to_vec().extend(1.0)).z
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
        self.rebuild_model_view();
    }
    
    fn ortho(&self) -> Ortho<f32> {
        Ortho {
            left:   self.width  as f32 / (-1.0 * self.zoom),
            right:  self.width  as f32 /         self.zoom ,
            bottom: self.height as f32 / (-1.0 * self.zoom),
            top:    self.height as f32 /         self.zoom ,
            near:    200.0,
            far:    -200.0
        }
    }
    
    pub fn translate(&mut self, amount: Vector2<f32>) {
        self.translation = self.translation + amount;
        self.rebuild_model_view();
    }
    
    // Converts a point in window space to a line in world space.
    // 
    // p is measured in pixels, where the upper left corner of the window is (0, 0) and
    // the lower right is (self.width, self.height).
    pub fn unproject(&self, p: Point2<f32>) -> PLine3 {
        // Convert to OpenGL clip space, i.e [-1, 1].
        let v1: Vector4<f32> = Vector4::new(
            p.x * 2.0 / self.width as f32 - 1.0,
            p.y * 2.0 / self.width as f32 - 1.0,
            0.0, 0.0
        );
        
        let mut v2: Vector4<f32> = v1.clone();
        v2.z = 1.0;
        
        PLine3::new(
            &Point3::from_vec(&self.inverse.mul_v(&v1).truncate()),
            &Point3::from_vec(&self.inverse.mul_v(&v2).truncate())
        )
    }
    
    pub fn zoom(&self) -> f32 {
        self.zoom
    }
    
    pub fn zoom_by(&mut self, multiplier: f32) {
        self.zoom = self.zoom * multiplier;
        self.rebuild_projection();
    }
    
    
    pub fn resize(&mut self, width: u16, height: u16) {
        if width != self.width || height != self.height {
            self.width = width;
            self.height = height;
            self.rebuild_projection();
        }
    }
    
    fn rebuild_inverse(&mut self) {
        // Build a special version of the projection matrix having a very small near-far
        // interval. This means the viewing volume will be smaller, resulting in a
        // larger determinant.
        // 
        // If we tried to use the real projection matrix, which has a very high volume,
        // the determinant would be so close to zero that the matrix would think itself
        // degenerate. (Due to cgmath's approximate floating point comparison.) It would
        // thus refuse to invert itself.
        let mut tmp_projection: Ortho<f32> = self.ortho();
        tmp_projection.near =  0.001;
        tmp_projection.far  = -0.001;
        let tmp_projection: Matrix4<f32> = tmp_projection.to_matrix4();
        
        let model_view_projection: Matrix4<f32> = tmp_projection.mul_m(&self.model_view);
        
        self.inverse = model_view_projection.invert().unwrap_or_else(|| {
            panic!(
              "Camera matrix was degenerate. Determinant: {} ({}:{})",
              model_view_projection.determinant(),
              file!(), line!()
            )
        });
    }
    
    pub fn rebuild_model_view(&mut self) {
        // Remember that transformations are applied in reverse order.
    
        // Translate model.
        self.model_view = Matrix4::from_translation(&self.translation.extend(0f32));
    
        // X-rotate model.
        self.model_view.mul_self_m(
            &Matrix3::from_angle_x(CAMERA_TILT).to_matrix4()
        );
    
        // Z-rotate model.
        self.model_view.mul_self_m(
            &Matrix3::from_angle_z(self.z_rotation).to_matrix4()
        );
        
        self.rebuild_inverse();
    }
    
    
    pub fn rebuild_projection(&mut self) {
        self.projection = self.ortho().to_matrix4();
        self.rebuild_inverse();
    }
}