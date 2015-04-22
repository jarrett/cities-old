use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::str;
use std::ffi::CString;
use std::iter;
use std::ptr;
use std::path::Path;
use gl;
use gl::types::*;
use libc;

pub fn get_attrib_location(program: GLuint, name: &str) -> GLuint {
    unsafe {
        let c_str = CString::new(name).unwrap();
        let loc: GLint = gl::GetAttribLocation(program, c_str.as_ptr());
        if loc == -1 {
          panic!("Could not find attribute \"{}\"", name);
        }
        loc as GLuint
    }
}

// The OpenGL API uses signed ints for uniform attribute locations.
// See e.g. glUniform.
pub fn get_uniform_location(program: GLuint, name: &str) -> GLint {
    unsafe {
        let c_str = CString::new(name).unwrap();
        let loc: GLint = gl::GetUniformLocation(program, c_str.as_ptr());
        if loc == -1 {
          panic!("Could not find uniform \"{}\"", name);
        }
        loc
    }
}

pub fn make_program(vert_path: &Path, frag_path: &Path) -> GLuint {
    unsafe {
        let program: GLuint = gl::CreateProgram();
        let vert_shader: GLuint = make_shader(vert_path, gl::VERTEX_SHADER);
        let frag_shader: GLuint = make_shader(frag_path, gl::FRAGMENT_SHADER);
        gl::AttachShader(program, vert_shader);
        gl::AttachShader(program, frag_shader);
        gl::LinkProgram(program);
        let mut link_status: GLint = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_status);
        if link_status != gl::TRUE as GLint {
            let mut log_length: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
            let s: String = iter::repeat(' ').take(log_length as usize).collect();
            let c_str: CString = CString::new(s.as_str()).unwrap();
            gl::GetProgramInfoLog(
                program,
                log_length,
                ptr::null::<GLint>() as *mut GLint,
                c_str.as_bytes().as_ptr() as *mut libc::c_char
            );
            panic!("GLSL program failed to link:\n\n{}", str::from_utf8(c_str.as_bytes()).unwrap());
        }
        program
    }
}


pub fn make_shader(path: &Path, shader_type: GLenum) -> GLuint {
    unsafe {
        let display = path.display();
    
        // Open the file for reading.
        let mut file: File = match File::open(path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
    
        // Read the file into a C string.
        let mut source: String = String::new();
        match file.read_to_string(&mut source) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            _ => {}
        };
        let src_c_str: CString = CString::new(source.as_str()).unwrap();
    
        // Create and compile the shader.
        let shader: GLuint = gl::CreateShader(shader_type);
        gl::ShaderSource(
            shader,
            1,
            &src_c_str.as_ptr(),
            ptr::null()
        );
        gl::CompileShader(shader);
    
        // Check for compile errors.
        let mut status: GLint = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != gl::TRUE as GLint {
          let mut log_length: GLint = 0;
          gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
          let s: String = iter::repeat(' ').take(log_length as usize).collect();
          let c_str: CString = CString::new(s.as_str()).unwrap();
          gl::GetShaderInfoLog(
            shader,
            log_length,
            ptr::null::<GLint>() as *mut GLint,
            c_str.as_bytes().as_ptr() as *mut libc::c_char
          );
          panic!("GLSL shader {} failed to compile:\n\n{}", path.display(), str::from_utf8(c_str.as_bytes()).unwrap());
        }
    
        shader
    }
}