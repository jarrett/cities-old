use std::ops::Neg;
use cgmath::{
  EuclideanSpace, Matrix, Matrix3, Matrix4, Ortho, Point2, Point3,
  Rad, SquareMatrix, Transform, Vector2, Vector3
};
use glfw::{Window, Action, Key};

// 28 degrees on the Z axis.
static CAMERA_TILT: Rad<f32> = Rad(3.97935069f32);

// 228 degrees (or 48 degrees, in Blender terms) on the X axis.
static CAMERA_ORBIT: Rad<f32> = Rad(0.488692191f32);

pub struct Camera {
  pub z_rotation: Rad<f32>,
  pub orbit: u8,
  pub focus: Vector2<f32>,
  pub zoom: f32,
  pub transform: Matrix4<f32>,
  pub inverse: Matrix4<f32>,
  pub width: u16,
  pub height: u16,
  q_down: bool,
  e_down: bool,
}

impl Camera {
  pub fn new(width: u16, height: u16, zoom: f32) -> Camera {
    let mut cam = Camera {
      z_rotation: Rad(0f32), orbit: 3, focus: Vector2 {x: 0f32, y: 0f32},
      zoom: zoom, transform: Matrix4::one(), inverse: Matrix4::one(),
      width: width, height: height, q_down: false, e_down: false
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
    self.transform.transform_point(point.clone()).z
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
    self.z_rotation = CAMERA_ORBIT + Rad(dir as f32 * 1.57079632679f32);
    self.rebuild_matrices();
  }
  
  // amount is in screen space.
  pub fn pan(&mut self, amount: &Vector2<f32>) {
    let amount_3 = amount.extend(0.0);
    let amount_2 = self.inverse.transform_vector(amount_3).truncate();
    self.focus = self.focus + amount_2;
    self.rebuild_matrices();
  }
  
  // TODO: Camera shouldn't know about Window.
  pub fn receive_input(&mut self, window: &Window) {
    // Orbit camera with Q and E.
    if window.get_key(Key::Q) == Action::Press {
      self.q_down = true;
    } else {
      if self.q_down {
        self.decrement_orbit();
        self.q_down = false;
      }
    }
    if window.get_key(Key::E) == Action::Press {
      self.e_down = true;
    } else {
      if self.e_down {
        self.increment_orbit();
        self.e_down = false;
      }
    }
    
    // Pan camera with W and S.
    if window.get_key(Key::W) == Action::Press {
      self.pan(&Vector2::new(0.0, 0.02));
    }
    if window.get_key(Key::S) == Action::Press {
      self.pan(&Vector2::new(0.0, -0.02));
    }
    
    // Pan camera with A and D.
    if window.get_key(Key::A) == Action::Press {
      self.pan(&Vector2::new(-0.02, 0.0));
    }
    if window.get_key(Key::D) == Action::Press {
      self.pan(&Vector2::new(0.02, 0.0));
    }
    
    // Zoom camera with Z and X.
    if window.get_key(Key::Z) == Action::Press {
      self.zoom_by(1.05);
    }
    if window.get_key(Key::X) == Action::Press {
      self.zoom_by(0.9523809524);
    }
  }
  
  // Converts a point in window space to a line in world space. The line is represented as two points.
  // 
  // p is measured in pixels, where the upper left corner of the window is (0, 0) and
  // the lower right is (self.width, self.height).
  pub fn unproject(&self, p: Point2<f32>) -> (Point3<f32>, Point3<f32>) {
    // Convert to OpenGL clip space, i.e. [-1, 1].
    let v1: Vector3<f32> = Vector3::new(
      p.x *  2.0 / self.width  as f32 - 1.0,
      p.y * -2.0 / self.height as f32 + 1.0,
      -1.0
    );
    
    let mut v2: Vector3<f32> = v1.clone();
    v2.z = 1.0;
    
    (
      Point3::from_vec(self.inverse.transform_vector(v1)),
      Point3::from_vec(self.inverse.transform_vector(v2))
    )
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
    self.transform = self.transform *
      Matrix4::from(Matrix3::from_angle_x(CAMERA_TILT));

    // Z-rotate model.
    self.transform = self.transform *
      Matrix4::from(Matrix3::from_angle_z(self.z_rotation));
    
    // Translate model.
    self.transform = self.transform * 
      Matrix4::from_translation(self.focus.neg().extend(0.0));
    
    self.inverse = self.transform.invert().unwrap();
  }
}