use std::fmt::Debug;
use std::mem;
use gl;
use gl::types::*;

mod lines;

pub use self::lines::DebugLines;

#[allow(dead_code)]
pub fn print_vbo<T: Debug>(id: GLuint, target: GLenum, group: usize) {
    unsafe {
        gl::BindBuffer(target, id);
        
        let mut size: GLint = 0; // Size in bytes.
        gl::GetBufferParameteriv(target, gl::BUFFER_SIZE, &mut size);
        let size: usize = size as usize;
        let count: usize = size / mem::size_of::<T>();
        
        let ptr = gl::MapBuffer(target, gl::READ_ONLY) as *mut T;
        let values: Vec<T> = Vec::from_raw_parts(ptr, count, count);        
        
        print!("{} elements", count);
        if group > 1 {
            print!(" ({} groups)", count / group);
        }
        println!("");
        for (i, val) in values.iter().enumerate() {
            if group > 1 && i % group == 0 {
                // Start of group.
                print!("Group {}: (", i / group);
            }
            print!("{:?}", val);
            if group > 1 && i % group == group - 1 {
                // End of group.
                print!(")\n");
            } else if i != values.len() - 1 {
                // Delimiter between values.
                print!(", ");
            }
        }
        println!("");
        
        gl::UnmapBuffer(target);
        gl::BindBuffer(target, 0);
    }
}