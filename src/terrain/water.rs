use std::default::Default;
use std::path::Path;
use gl;
use gl::types::*;

use opengl;
use opengl::Texture2d;

pub struct Program {
    pub p:              opengl::Program,
    
    // Uniform locations.
    pub camera_idx:     GLint,
    pub foam_idx:       GLint,
    
    // Attribute locations.
    pub position_idx:   GLuint,
    pub depth_idx:      GLuint,
    
    // Textures.
    pub foam_tex:       Texture2d
}

impl Program {
    pub fn new() -> Program {
        let mut program = Program {
            p: opengl::Program::new(
                &Path::new("glsl/water.vert.glsl"),
                &Path::new("glsl/water.frag.glsl")
            ),
            camera_idx: 0, foam_idx: 0, position_idx: 0, depth_idx: 0,
            
            foam_tex:       Texture2d::from_file(&Path::new("assets/textures/foam.jpg"), &Default::default())
        };
        program.configure_indices();
        program
    }
    
    pub fn bind_textures(&self) {
        unsafe {
            self.foam_tex.bind(self.foam_idx, 0);
        }
    }
    
    fn configure_indices(&mut self) {
        self.camera_idx   = self.p.get_uniform_location("camera");
        self.foam_idx     = self.p.get_uniform_location("foam");
        self.position_idx = self.p.get_attrib_location( "position");
        self.depth_idx    = self.p.get_attrib_location( "depth");
    }
}