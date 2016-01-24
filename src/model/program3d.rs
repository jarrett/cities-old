use std::path::Path;
use gl;
use gl::types::*;

use opengl;
use opengl::Texture2d;

pub struct Program3d {
    pub p:              opengl::Program,
    
    // Uniform locations.
    pub camera_idx:     GLint,
    pub orbit_idx:      GLint,
    pub direction_idx:  GLint,
    pub origin_idx:     GLint,
    pub sprite_idx:     GLint,
    
    // Attribute locations.
    pub position_idx:   GLuint,
    pub uv_idx:         GLuint
}

impl Program3d {
    pub fn new() -> Program3d {
        let mut program = Program3d {
            p: opengl::Program::new(&Path::new(
                "glsl/model3d.vert.glsl"),
                &Path::new("glsl/model.frag.glsl")
            ),
            camera_idx: 0, orbit_idx: 0, direction_idx: 0, origin_idx: 0, sprite_idx: 0,
            position_idx: 0, uv_idx: 0
        };
        program.configure_indices();
        program
    }
    
    pub fn activate_texture(&self, texture: &Texture2d) {
        unsafe {
            texture.activate(0);
            gl::Uniform1i(self.sprite_idx, 0);
        }
    }
    
    fn configure_indices(&mut self) {
        self.camera_idx    = self.p.get_uniform_location("camera");
        self.orbit_idx     = self.p.get_uniform_location("orbit");
        self.direction_idx = self.p.get_uniform_location("direction");
        self.origin_idx    = self.p.get_uniform_location("origin");
        self.sprite_idx    = self.p.get_uniform_location("sprite");
        self.position_idx  = self.p.get_attrib_location( "position");
        self.uv_idx        = self.p.get_attrib_location( "uv");
    }
}