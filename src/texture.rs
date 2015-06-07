use gl;
use gl::types::*;

pub struct Config {
    pub min_filter: GLenum,
    pub mag_filter: GLenum,
    pub wrap_s: GLenum,
    pub wrap_t: GLenum,
    pub max_level: GLint
}

impl Default for Config {
    fn default() -> Config {
        Config {
            min_filter: gl::LINEAR_MIPMAP_LINEAR, mag_filter: gl::LINEAR,
            wrap_s: gl::REPEAT, wrap_t: gl::REPEAT, max_level: 4
        }
    }
}