extern crate glfw;
extern crate gl;
extern crate cgmath;
extern crate image;
extern crate libc;
extern crate num;

mod glutil;
mod camera;
mod axis_indicator;
mod texture;
mod world;
mod chunk;
mod terrain_program;
mod water_program;

use cgmath::*;
use glfw::{Context, Action, Key};

use camera::Camera;
use axis_indicator::AxisIndicator;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).ok().expect("Failed to init glfw");
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));
    
    let (mut window, events) = glfw.create_window(
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
    
    let axis = AxisIndicator::new();
    
    let mut cam = Camera::new(1280, 960, 10f32);
    
    while !window.should_close() {
        let (width, height) = window.get_size();
        cam.resize(width as u16, height as u16);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        axis.draw(&cam, 500.0 / cam.zoom);
        
        window.swap_buffers();
        
        glfw.poll_events();
        glfw::flush_messages(&events);
        
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
            cam.translate(Vector2::new(0.0, -20.0 / zoom));
        }
        if window.get_key(Key::S) == Action::Press {
            let zoom = cam.zoom();
            cam.translate(Vector2::new(0.0, 20.0 / zoom));
        }
        
        // Pan camera with A and D.
        if window.get_key(Key::A) == Action::Press {
            let zoom = cam.zoom();
            cam.translate(Vector2::new(20.0 / zoom, 0.0));
        }
        if window.get_key(Key::D) == Action::Press {
            let zoom = cam.zoom();
            cam.translate(Vector2::new(-20.0 / zoom, 0.0));
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