use std::old_io::File;
use cgmath::*;

use futil::read_vector_2;

pub struct UvsForDirection {
    // See doc/model-rendering.md for a diagram of these vertices.
    pub tb: Vector2<f32>,
    pub tr: Vector2<f32>,
    pub tf: Vector2<f32>,
    pub tl: Vector2<f32>,
    pub bl: Vector2<f32>,
    pub bf: Vector2<f32>,
    pub br: Vector2<f32>
}

impl UvsForDirection {
    pub fn from_file(file: &mut File) -> UvsForDirection {
        UvsForDirection {
            tb: read_vector_2(file),
            tr: read_vector_2(file),
            tf: read_vector_2(file),
            tl: read_vector_2(file),
            bl: read_vector_2(file),
            bf: read_vector_2(file),
            br: read_vector_2(file),
        }
    }
}