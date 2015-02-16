#![feature(path)]
#![feature(std_misc)]
#![feature(io)]
#![feature(libc)]
#![feature(collections)]
#![feature(core)]

extern crate glfw;
extern crate gl;
extern crate cgmath;
extern crate image;
extern crate libc;
extern crate num;

mod assertions;
mod glutil;
mod gldebug;
mod futil;
mod camera;
mod axis_indicator;
mod texture;
mod world;
mod terrain;
mod chunk;
mod water;
mod thing;
mod model;
mod meta_thing;
mod meta_model;

use std::rc::Rc;
use cgmath::*;
use glfw::{Context, Action, Key};

use camera::Camera;
use axis_indicator::AxisIndicator;
use world::World;
use meta_model::MetaModel;
use meta_thing::MetaThing;
use thing::Thing;

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
    
    let mut camera = Camera::new(1280, 960, 10f32);
    
    let terrain_program = terrain::Program::new();
    let water_program   = water::Program::new();
    let model_program3d   = model::Program3d::new();
    
    let mut model_buffers = model::Buffers::new();
    
    let meta_models_map = MetaModel::load_dir(
      &Path::new("assets/models"),
      &mut model_buffers
    );
    
    model_buffers.upload(&model_program3d);
    
    let meta_things_map = MetaThing::load_dir(
        &meta_models_map,
        &Path::new("assets/things")
    ).unwrap();
    
    let world = World::new(
        terrain::source::ImageSource::new(&Path::new("assets/height/river-128x128.png"), 0.1),
        &terrain_program, &water_program,
        16, 16
    );
    
    // For testing only.
    let meta_thing: &Rc<MetaThing> = meta_things_map.get("jarrett-test").unwrap();
    let thing = Thing::new(meta_thing, &Vector3::new(10.0, 10.0, 20.0));
    
    let mut q_down = false;
    let mut e_down = false;
    
    //gldebug::print_vbo::<u16>(model_buffers.index_buffer, gl::ELEMENT_ARRAY_BUFFER, 3);
    
    while !window.should_close() {
        let (width, height) = window.get_size();
        camera.resize(width as u16, height as u16);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        axis.draw(&camera, 500.0 / camera.zoom);
        world.draw(&camera, &terrain_program, &water_program);
        
        // For testing only.
        for model in thing.models().iter() {
            model.draw(&model_program3d, &model_buffers, &camera);
        }
        
        window.swap_buffers();
        
        glfw.poll_events();
        glfw::flush_messages(&events);
        
        // Orbit camera with Q and E.
        if window.get_key(Key::Q) == Action::Press {
            q_down = true;
        } else {
            if q_down {
                camera.decrement_orbit();
                q_down = false;
            }
        }
        if window.get_key(Key::E) == Action::Press {
            e_down = true;
            
        } else {
            if e_down {
                camera.increment_orbit();
                e_down = false;
            }
        }
        
        // Pan camera with W and S.
        if window.get_key(Key::W) == Action::Press {
            let zoom = camera.zoom();
            camera.translate(Vector2::new(0.0, -20.0 / zoom));
        }
        if window.get_key(Key::S) == Action::Press {
            let zoom = camera.zoom();
            camera.translate(Vector2::new(0.0, 20.0 / zoom));
        }
        
        // Pan camera with A and D.
        if window.get_key(Key::A) == Action::Press {
            let zoom = camera.zoom();
            camera.translate(Vector2::new(20.0 / zoom, 0.0));
        }
        if window.get_key(Key::D) == Action::Press {
            let zoom = camera.zoom();
            camera.translate(Vector2::new(-20.0 / zoom, 0.0));
        }
        
        // Zoom camera with Z and X.
        if window.get_key(Key::Z) == Action::Press {
            camera.zoom_by(1.05);
        }
        if window.get_key(Key::X) == Action::Press {
            camera.zoom_by(0.9523809524);
        }
    }
}