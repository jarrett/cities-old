extern crate cgmath;

use self::cgmath::*;

// 28 degrees on the Z axis.
static CAMERA_TILT: Rad<f32> = Rad { s: 3.97935069f32 };

// 228 degrees (or 48 degrees, in Blender terms) on the X axis.
static CAMERA_ORBIT: Rad<f32> = Rad { s: 0.488692191f32 };

pub struct Camera {
    z_rotation: Rad<f32>,
    orbit: u8,
    translation: Vector2<f32>,
    zoom: f32,
    model_view: Matrix4<f32>,
    projection: Matrix4<f32>,
    width: u16,
    height: u16
}

impl Camera {
    pub fn new(width: u16, height: u16, zoom: f32) -> Camera {
        let mut cam = Camera {
            z_rotation: rad(0f32), orbit: 3, translation: Vector2 {x: 0f32, y: 0f32},
            zoom: zoom, model_view: Matrix4::identity(), projection: Matrix4::identity(),
            width: width, height: height
        };
        cam.rebuild_model_view();
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
    
    
    pub fn translate(&mut self, amount: Vector2<f32>) {
        self.translation = self.translation + amount;
        self.rebuild_model_view();
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
    }
    
    
    pub fn rebuild_projection(&mut self) {
        self.projection = ortho(
            self.width  as f32 / (-1f32 * self.zoom),    // Left.
            self.width  as f32 /          self.zoom ,    // Right.
            self.height as f32 / (-1f32 * self.zoom),    // Bottom.
            self.height as f32 /          self.zoom ,    // Top.
            1000f32, -1000f32                            // Near, far.
        );
    }
}