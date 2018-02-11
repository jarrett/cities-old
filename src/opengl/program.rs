use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::str;
use std::ffi::CString;
use std::iter;
use std::ptr;
use std::path::Path;
use gl;
use gl::types::{GLuint, GLint, GLenum};
use libc;

pub struct Program {pub id: GLuint}

struct Shader {id: GLuint}

impl Program {
  pub fn new(vert_path: &Path, frag_path: &Path) -> Program {
    unsafe {
      let id: GLuint = gl::CreateProgram();
      let vert_shader: Shader = Shader::new(vert_path, gl::VERTEX_SHADER);
      let frag_shader: Shader = Shader::new(frag_path, gl::FRAGMENT_SHADER);
      gl::AttachShader(id, vert_shader.id);
      gl::AttachShader(id, frag_shader.id);
      gl::LinkProgram(id);
      let mut link_status: GLint = gl::FALSE as GLint;
      gl::GetProgramiv(id, gl::LINK_STATUS, &mut link_status);
      if link_status != gl::TRUE as GLint {
        let mut log_length: GLint = 0;
        gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut log_length);
        let s: String = iter::repeat(' ').take(log_length as usize).collect();
        let c_str: CString = CString::new(s).unwrap();
        gl::GetProgramInfoLog(
          id,
          log_length,
          ptr::null::<GLint>() as *mut GLint,
          c_str.as_bytes().as_ptr() as *mut libc::c_char
        );
        panic!("GLSL program failed to link:\n\n{}", str::from_utf8(c_str.as_bytes()).unwrap());
      }
      Program {id: id}
    }
  }
  
  pub fn get_attrib_location(&self, name: &str) -> GLuint {
    unsafe {
      let c_str = CString::new(name).unwrap();
      let loc: GLint = gl::GetAttribLocation(self.id, c_str.as_ptr());
      if loc == -1 {
        panic!("Could not find attribute \"{}\"", name);
      }
      loc as GLuint
    }
  }

  // The OpenGL API uses signed ints for uniform attribute locations.
  pub fn get_uniform_location(&self, name: &str) -> GLint {
    unsafe {
      let c_str = CString::new(name).unwrap();
      let loc: GLint = gl::GetUniformLocation(self.id, c_str.as_ptr());
      if loc == -1 {
        panic!("Could not find uniform \"{}\"", name);
      }
      loc
    }
  }
}

impl Shader {
  fn new(path: &Path, shader_type: GLenum) -> Shader {
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
      let src_c_str: CString = CString::new(source).unwrap();

      // Create and compile the shader.
      let id: GLuint = gl::CreateShader(shader_type);
      gl::ShaderSource(
        id,
        1,
        &src_c_str.as_ptr(),
        ptr::null()
      );
      gl::CompileShader(id);

      // Check for compile errors.
      let mut status: GLint = gl::FALSE as GLint;
      gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
      if status != gl::TRUE as GLint {
        let mut log_length: GLint = 0;
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length);
        let s: String = iter::repeat(' ').take(log_length as usize).collect();
        let c_str: CString = CString::new(s).unwrap();
        gl::GetShaderInfoLog(
          id,
          log_length,
          ptr::null::<GLint>() as *mut GLint,
          c_str.as_bytes().as_ptr() as *mut libc::c_char
        );
        panic!("GLSL shader {} failed to compile:\n\n{}", path.display(), str::from_utf8(c_str.as_bytes()).unwrap());
      }

      Shader {id: id}
    }
  }
}

impl Drop for Program {
  fn drop(&mut self) {
    unsafe { gl::DeleteProgram(self.id); }
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe { gl::DeleteShader(self.id); }
  }
}