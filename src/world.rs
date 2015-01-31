use cgmath::*;

pub struct World;

impl World {
  pub fn vert_position_at(&self, abs_x: i32, abs_y: i32) -> Option<Vector3<f32>> {
    // When porting the implementation from C++, verify that both coords are positive.
    // We didn't do that in the old version, preferring to perform such checks elsewhere.
    // But now we prefer to do it here.
    None
  }
}