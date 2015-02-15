use gl;
use gl::types::*;

use glutil;

pub struct Program3d {
    pub id:             GLuint,
    
    // Uniform locations.
    pub model_view_idx: GLint,
    pub projection_idx: GLint,
    pub direction_idx:  GLint,
    pub origin_idx:     GLint,
    
    // Attribute locations.
    pub position_idx:   GLuint,
    pub uv_idx:         GLuint
}

impl Program3d {
    pub fn new() -> Program3d {
        let id = glutil::make_program(&Path::new("glsl/model3d.vert.glsl"), &Path::new("glsl/model.frag.glsl"));
        Program3d {
            id:             id,
            
            model_view_idx: glutil::get_uniform_location(id, "model"),
            projection_idx: glutil::get_uniform_location(id, "projection"),
            direction_idx:  glutil::get_uniform_location(id, "direction"),
            origin_idx:     glutil::get_uniform_location(id, "origin"),
            
            position_idx:   glutil::get_attrib_location( id, "position"),
            uv_idx:         glutil::get_attrib_location( id, "uv")
        }
    }
}