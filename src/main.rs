extern crate glfw;

use glfw::Context;

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

  let (mut window, events) = glfw.create_window(800, 600, "Cities", glfw::WindowMode::Windowed)
    .expect("Failed to create window.");

  window.set_key_polling(true);
  window.make_current();

  while !window.should_close() {
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      // Do something with the event, which is a glfw::WindowEvent.
    }
  }
}