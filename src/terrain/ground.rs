use std::default::Default;
use std::path::Path;
use gl;
use gl::types::*;

use opengl;
use opengl::{Texture2d, TextureConfig};

pub struct Program {
    pub p:                  opengl::Program,
    
    // Uniform locations.
    pub camera_idx:         GLint,
    pub underwater_idx:     GLint,
    pub flat_idx:           GLint,
    pub slope_idx:          GLint,
    pub cliff_idx:          GLint,
    pub mouse_in_idx:       GLint,
    pub mouse_position_idx: GLint,
    
    // Attribute locations.
    pub position_idx:       GLuint,
    pub normal_idx:         GLuint,
    
    // Textures.
    pub underwater_tex:     Texture2d,
    pub flat_tex:           Texture2d,
    pub slope_tex:          Texture2d,
    pub cliff_tex:          Texture2d
}

impl Program {
    pub fn new() -> Program {
        let tex_cfg: TextureConfig = Default::default();
        let mut program = Program {
            p: opengl::Program::new(
                &Path::new("glsl/terrain.vert.glsl"),
                &Path::new("glsl/terrain.frag.glsl")
            ),
            
            camera_idx: 0, underwater_idx: 0, flat_idx: 0, slope_idx: 0, cliff_idx: 0,
            mouse_in_idx: 0, mouse_position_idx: 0, position_idx: 0, normal_idx: 0,
            
            underwater_tex: Texture2d::from_file(&Path::new("assets/textures/underwater.jpg"), &tex_cfg),
            flat_tex:       Texture2d::from_file(&Path::new("assets/textures/plain.jpg"), &tex_cfg),
            slope_tex:      Texture2d::from_file(&Path::new("assets/textures/slope.jpg"), &tex_cfg),
            cliff_tex:      Texture2d::from_file(&Path::new("assets/textures/cliff.jpg"), &tex_cfg)
        };
        program.configure_indices();
        program
    }
    
    pub fn activate_textures(&self) {
        unsafe {
            self.underwater_tex.activate(0);
            gl::Uniform1i(self.underwater_idx, 0);
            
            self.flat_tex.activate(1);
            gl::Uniform1i(self.flat_idx, 1);
            
            self.slope_tex.activate(2);
            gl::Uniform1i(self.slope_idx, 2);
            
            self.cliff_tex.activate(3);
            gl::Uniform1i(self.cliff_idx, 3);
        }
    }
    
    fn configure_indices(&mut self) {
        self.camera_idx         = self.p.get_uniform_location("camera");
        self.underwater_idx     = self.p.get_uniform_location("underwater");
        self.flat_idx           = self.p.get_uniform_location("plain");
        self.slope_idx          = self.p.get_uniform_location("slope");
        self.cliff_idx          = self.p.get_uniform_location("cliff");
        self.mouse_in_idx       = self.p.get_uniform_location("mouseIn");
        self.mouse_position_idx = self.p.get_uniform_location("mousePosition");
        self.position_idx       = self.p.get_attrib_location( "position");
        self.normal_idx         = self.p.get_attrib_location( "normal");
    }
}