use std::boxed::Box;

pub use self::menu::Menu;

mod menu;

pub trait Mode {
  // Configure OpenGL with functions such as glClearColor.
  fn configure_gl(&self);

  fn draw(&self);

  // Optionally tell the main loop to switch to a new mode at the next iteration.
  fn transition(&self) -> Option<Box<Mode>>;
}