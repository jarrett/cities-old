#![feature(core)]
#![feature(collections)]
#![feature(convert)]
#![feature(fs_walk)]
#![feature(std_misc)]

extern crate glfw;
extern crate gl;
extern crate cgmath;
extern crate image;
extern crate libc;
extern crate num;
extern crate byteorder;

mod macros;
mod math;
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
mod mouse;

use std::default::Default;
use std::cmp;
use std::path::Path;
use cgmath::*;
use glfw::{Context, Action, Key};
use gl::types::*;

use camera::Camera;
use axis_indicator::AxisIndicator;
use world::World;
use model::MetaModel;
use thing::{MetaThing, ZSorted};
use texture::Spritesheet;

fn main() {
    println!("Initing GLFW");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).ok().expect("Failed to init glfw");
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    println!("Creating window");
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
    
    println!("Loading shaders");
    let terrain_program = terrain::Program::new();
    let water_program   = water::Program::new();
    let model_program3d   = model::Program3d::new();
    
    println!("Creating spritesheets");
    let mut max_texture_size: GLint = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max_texture_size);
    }
    let texture_size: u32 = cmp::min(max_texture_size as u32, 2048);
    println!("Texture size: {}", texture_size);
    let spritesheet = Spritesheet::load_dir(
        texture_size, texture_size,
        &Path::new("assets/sprites"),
        &Default::default()
    ).unwrap();
    
    println!("Initing model buffers");
    let mut model_buffers = model::Buffers::new();
    
    println!("Loading meta models");
    let meta_models_map = MetaModel::load_dir(
      &Path::new("assets/models"),
      &mut model_buffers,
      &spritesheet
    ).unwrap();
    
    model_buffers.upload(&model_program3d);
    
    println!("Loading meta things");
    let meta_things_map = MetaThing::load_dir(
        &meta_models_map,
        &Path::new("assets/things"),
    ).unwrap();
    
    println!("Loading world");
    /*let mut world: World = World::new(
        String::from_str("river-128x128"),
        terrain::source::ImageSource::new(Path::new("assets/height/river-128x128.png"), 0.1),
        &terrain_program, &water_program,
        16, 16
    );*/
    
    let mut world: World = World::from_file(
        &terrain_program, &water_program, 16, 16, &meta_things_map, &Path::new("saves/test.city")
    ).unwrap();
    
    // For testing only.
    /*let meta_thing: &Rc<MetaThing> = meta_things_map.get("jarrett-test").unwrap();
    for direction in 0u8..8u8 {
        let thing = Rc::new(
          Thing::new(meta_thing, &Vector3::new(5.0 + 3.0 * direction as f32, 5.0, 45.0), direction)
        );
        world.things.push(thing);
    }*/
    
    let z_sorted = ZSorted::new(&world.things, &mut camera);
    
    let mut mouse_tree = mouse::Tree::new(128, Aabb3::new(
        Point3::new(world.min_x(), world.min_y(), 0.0),
        Point3::new(world.max_x(), world.max_y(), 0.0)
    ));
    mouse_tree.build();
    mouse_tree.add_chunks_from_world(&world);
    
    let mut q_down = false;
    let mut e_down = false;
    
    //gldebug::print_vbo::<f32>(model_buffers.uv_buffer, gl::ARRAY_BUFFER, 2);
    //gldebug::print_vbo::<u16>(model_buffers.index_buffer, gl::ELEMENT_ARRAY_BUFFER, 18);
    
    //world.to_file(&Path::new("saves/test.city")).unwrap();
    
    println!("Starting main loop");
    while !window.should_close() {
        let (width, height) = window.get_size();
        camera.resize(width as u16, height as u16);
        
        //let mouse_target = mouse_tree.intersects_line3(&mouse_line);
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        axis.draw(&camera, 500.0 / camera.zoom);
        world.draw(&camera, &terrain_program, &water_program);
        
        // For testing only.
        for thing in z_sorted.get(&camera).iter() {
            thing.draw(&model_program3d, &model_buffers, &camera);
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