use std;
use std::path::Path;
use std::cmp::min;
use gl;
use gl::types::GLint;
use glfw;
use glfw::{Glfw, Window, Context};
use cgmath::{Point3, Aabb3};

use errors::GameError;
use futil::IoErrorLine;
use camera::Camera;
use ui;
use ui::Ui;
use world::World;
use model;
use model::{MetaModel, Program3d};
use thing::{MetaThing, ZSorted};
use sprite;
use terrain;
use mouse;

// http://www.rust-ci.org/bjz/glfw-rs/doc/glfw/struct.Glfw.html#method.create_window
// http://smallcultfollowing.com/rust-int-variations/imem-umem/sync/comm/struct.Receiver.html
pub type Events = std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>;

pub fn init() -> Result<(
    Glfw,
    Window,
    Events,
    Ui,
    Camera,
    terrain::ground::Program,
    terrain::water::Program,
    model::Program3d,
    model::Buffers,
    World,
    ZSorted,
    mouse::Tree
), GameError> {
    let glfw = init_glfw();
    
    let (window, events) = init_window(&glfw);
    
    let mut camera = Camera::new(1280, 960, 10f32);
    
    let ui = init_ui();
    
    let (ground_program, water_program, model_program_3d) = init_programs();
    
    let sprite_sheet = try!(init_sprite_sheet());
    
    let mut model_buffers = model::Buffers::new();
    
    let world = try!(init_world(&ground_program, &water_program, &mut model_buffers, &sprite_sheet));
    
    // Must call after init_world because that's where the meta models are buffered.
    model_buffers.upload(&model_program_3d);
    
    let z_sorted = ZSorted::new(&world.things, &mut camera);
    
    let mut mouse_tree = mouse::Tree::new(128, Aabb3::new(
        Point3::new(world.min_x(), world.min_y(), 0.0),
        Point3::new(world.max_x(), world.max_y(), 0.0)
    ));
    mouse_tree.build();
    mouse_tree.add_chunks_from_world(&world);
    
    Ok((
        glfw, window, events, ui, camera, ground_program, water_program, model_program_3d,
        model_buffers, world, z_sorted, mouse_tree
    ))
}

fn init_glfw() -> Glfw {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).ok().expect("Failed to init glfw");
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    glfw
}

fn init_window(glfw: &Glfw) -> (Window, Events) {
    let (mut window, events) = glfw.create_window(
        1280, 960, "Cities", glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window.");
    
    window.set_key_polling(true);
    window.make_current();
    
    // Load the external functions. From the gl-rs crate.
    gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);
    
    unsafe {
        // Basic OpenGL configs.
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::DepthFunc(gl::LEQUAL);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::ClearColor(0.9, 0.94, 1.0, 1.0);
    }
    
    (window, events)
}

fn init_ui() -> Ui {
    let mut ui = Ui::new();
    ui.add_widget(ui::Button::new(40, 20, 200, 50).text("Test Button"));
    ui
}

fn init_programs() -> (terrain::ground::Program, terrain::water::Program, model::Program3d) {
    (   terrain::ground::Program::new(),
        terrain::water::Program::new(),
        model::Program3d::new())
}

fn init_sprite_sheet() -> Result<sprite::Sheet, IoErrorLine> {
    let mut max_texture_size: GLint = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max_texture_size);
    }
    let texture_size: usize = min(max_texture_size as usize, 2048);
    sprite::Sheet::load_dir(
        texture_size, texture_size,
        &Path::new("assets/sprites"),
        &Default::default()
    )
}

fn init_world(
    ground_program: &terrain::ground::Program,
    water_program: &terrain::water::Program,
    model_buffers: &mut model::Buffers,
    sprite_sheet: &sprite::Sheet
) -> Result<World, IoErrorLine> {
    let meta_models_map = MetaModel::load_dir(
      &Path::new("assets/models"),
      // Each meta model will be buffered.
      model_buffers,
      sprite_sheet
    ).unwrap();
    
    let meta_things_map = try!(MetaThing::load_dir(
        &meta_models_map,
        &Path::new("assets/things"),
    ));
    
    World::from_file(
        &ground_program, &water_program,
        16, 16, // Chunk size.
        &meta_things_map,
        &Path::new("saves/test.city")
    )
}