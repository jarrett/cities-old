extern crate gl;
extern crate glfw;

use glfw::Context;

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

  let (mut window, events) = glfw.create_window(1600, 1200, "Cities", glfw::WindowMode::Windowed)
    .expect("Failed to create window.");

  window.set_key_polling(true);
  window.make_current();
  window.maximize();

  gl::load_with(|s|
    window.get_proc_address(s) as *const std::os::raw::c_void
  );

  unsafe {
    gl::Enable(gl::DEPTH_TEST);
    gl::Enable(gl::BLEND);
    gl::DepthFunc(gl::LEQUAL);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::ClearColor(0.9, 0.94, 1.0, 1.0);
  }

  while !window.should_close() {
    window.swap_buffers();
    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    glfw.poll_events();
    glfw::flush_messages(&events);
  }
}