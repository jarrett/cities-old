use gl;
use super::Mode;

// The game mode for the main menu system.
pub struct Menu;

impl Mode for Menu {
  fn configure_gl(&self) {
    unsafe {
      gl::Enable(gl::DEPTH_TEST);
      gl::Enable(gl::BLEND);
      gl::DepthFunc(gl::LEQUAL);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
      gl::ClearColor(0.9, 0.94, 1.0, 1.0);
    }
  }

  fn draw(&self) {
    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
  }

  fn transition(&self) -> Option<Box<Mode>> {
    None
  }
}