#![feature(vec_push_all)]
#![feature(fs_walk)]
#![feature(drain)]
#![feature(convert)]

extern crate glfw;
extern crate gl;
extern crate cgmath;
extern crate image;
extern crate freetype;
extern crate libc;
extern crate num;
extern crate byteorder;

mod macros;
mod errors;
mod math;
mod glutil;
mod gldebug;
mod futil;
mod init;
mod camera;
mod world;
mod terrain;
mod overlay;
mod chunk;
mod thing;
mod model;
mod mouse;
mod texture;
mod text;
mod hud;

use cgmath::{Point2, Ray3};

use glfw::Context;

fn main() {
    let (
        mut glfw, mut window, events, mut camera,
        ground_program, water_program, model_program_3d,
        model_buffers, world, z_sorted, mouse_tree
    ) = init::init().unwrap();
    
    while !window.should_close() {
        let (width, height) = window.get_size();
        camera.resize(width as u16, height as u16);
        
        let (mouse_x, mouse_y) = window.get_cursor_pos();
        //println!("{}, {}", mouse_x, mouse_y);
        let mouse_ray: Ray3<f32> = camera.unproject(Point2::new(mouse_x as f32, mouse_y as f32));
        let mouse_hit: Option<mouse::Hit> = mouse_tree.intersects_ray3(&mouse_ray, &camera);
        //println!("mouse line: {:?}", mouse_line);
        //println!("target: {:?}", mouse_hit);
        /*debug_lines.clear();
        debug_lines.add_ray3(
            &mouse_ray,
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0
        );*/
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        //debug_lines.draw(&camera);
        
        world.draw(&camera, &ground_program, &water_program, &mouse_hit);
        
        // For testing only.
        for thing in z_sorted.get(&camera).iter() {
            thing.draw(&model_program_3d, &model_buffers, &camera);
        }
        
        window.swap_buffers();
        
        glfw.poll_events();
        glfw::flush_messages(&events);
        
        camera.receive_input(&window);
    }
}