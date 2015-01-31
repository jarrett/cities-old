extern crate glfw;
extern crate gl;
extern crate cgmath;
extern crate libc;

mod camera;
mod texture;
mod world;
mod chunk;

//use gl::types::*;
use glfw::*;
use camera::*;
use cgmath::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).ok().expect("Failed to init glfw");
    
    let (mut window, _) = glfw.create_window(
        1280, 960, "Cities", glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window.");
    
    window.set_key_polling(true);
    window.make_current();
    
    // Load the external functions. From the gl-rs crate.
    gl::load_with(|s| window.get_proc_address(s));
    
    unsafe {
        // Basic OpenGL configs.
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::DepthFunc(gl::LEQUAL);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::ClearColor(0.9, 0.94, 1.0, 1.0);
    }
    
    let mut cam = Camera::new(1280, 960, 10f32);
    
    while !window.should_close() {
        let (width, height) = window.get_size();
        cam.resize(width as u16, height as u16);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        window.swap_buffers();
        
        glfw.poll_events();
        
        // Orbit camera with Q and E.
        if window.get_key(Key::Q) == Action::Press {
            cam.decrement_orbit();
        }
        if window.get_key(Key::E) == Action::Press {
            cam.increment_orbit();
        }
        
        // Pan camera with W and S.
        if window.get_key(Key::W) == Action::Press {
            let zoom = cam.zoom();
            cam.translate(Vector2::new(0f32, -20f32 / zoom));
        }
        if window.get_key(Key::S) == Action::Press {
            let zoom = cam.zoom();
            cam.translate(Vector2::new(0f32, -20f32 / zoom));
        }
        
        // Pan camera with A and D.
        if window.get_key(Key::A) == Action::Press {
            let zoom = cam.zoom();
            cam.translate(Vector2::new(20f32 / zoom, 0f32));
        }
        if window.get_key(Key::D) == Action::Press {
            let zoom = cam.zoom();
            cam.translate(Vector2::new(-20f32 / zoom, 0f32));
        }
        
        // Zoom camera with Z and X.
        if window.get_key(Key::Z) == Action::Press {
            cam.zoom_by(1.05);
        }
        if window.get_key(Key::X) == Action::Press {
            cam.zoom_by(0.9523809524);
        }
    }
}