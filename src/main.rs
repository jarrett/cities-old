extern crate cgmath;
extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate image;
extern crate libc;

mod camera;
mod mode;
mod opengl;
mod ui;

use glfw::Context;
use std::boxed::Box;
use std::time::{Duration, Instant};
use std::thread;

fn main() {
  // Init GLFW.
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  let (mut window, events) = glfw.create_window(1600, 1200, "Cities", glfw::WindowMode::Windowed)
    .expect("Failed to create window.");
  window.set_key_polling(true);
  window.make_current();
  window.maximize();

  // Init OpenGL.
  gl::load_with(|s|
    window.get_proc_address(s) as *const std::os::raw::c_void
  );

  // The current game mode. Can change from one iteration of the main loop to the next.
  let mut mode: Box<mode::Mode> = Box::new(mode::Menu);
  mode.configure_gl();

  // Main loop. Mostly just delegates to the game mode.
  let mut last_frame_time: Instant = Instant::now();
  while !window.should_close() {
    window.swap_buffers();

    glfw.poll_events();
    glfw::flush_messages(&events);

    mode.draw();

    if let Some(new_mode) = mode.transition() {
      mode = new_mode;
      mode.configure_gl();
    }

    // Cap FPS.
    let min_frame_dur = Duration::new(0, 16666666); // 1/60 second.
    let now = Instant::now();
    let frame_dur = now.duration_since(last_frame_time);
    if frame_dur < min_frame_dur {
      thread::sleep(min_frame_dur - frame_dur);
    }
    last_frame_time = now;
  }
}