use cgmath::Vector2;
use cgmath::Vector3;
use std::old_io::{File, IoError};
use std::old_io::IoErrorKind::OtherIoError;
use std::string::FromUtf8Error;

#[allow(dead_code)]
pub fn read_string_16(file: &mut File) -> Result<String, IoError> {
    let length = try!(file.read_be_u16());
    let bytes = try!(file.read_exact(length as usize));
    String::from_utf8(bytes).map_err(from_utf8_error_into_io_error)
}

#[allow(dead_code)]
pub fn write_str_16(file: &mut File, string: &str) -> Result<(), IoError> {
    try!(file.write_be_u16(string.len() as u16));    
    file.write_str(string)
}

#[allow(dead_code)]
pub fn write_string_16(file: &mut File, string: &String) -> Result<(), IoError> {
    write_str_16(file, string.as_slice())
}

#[allow(dead_code)]
pub fn read_vector_2(file: &mut File) -> Result<Vector2<f32>, IoError> {
    let x = try!(file.read_be_f32());
    let y = try!(file.read_be_f32());
    Ok(Vector2 { x: x, y: y })
}

#[allow(dead_code)]
pub fn read_vector_3(file: &mut File) -> Result<Vector3<f32>, IoError> {
    let x = try!(file.read_be_f32());
    let y = try!(file.read_be_f32());
    let z = try!(file.read_be_f32());    
    Ok(Vector3 { x: x, y: y, z: z })
}

#[allow(dead_code)]
pub fn write_vector_2(file: &mut File, v: &Vector2<f32>) -> Result<(), IoError> {
    try!(file.write_be_f32(v.x));
    file.write_be_f32(v.y)
}

#[allow(dead_code)]
pub fn write_vector_3(file: &mut File, v: &Vector3<f32>) -> Result<(), IoError> {
    try!(file.write_be_f32(v.x));
    try!(file.write_be_f32(v.y));
    file.write_be_f32(v.z)
}

// Consumes a FromUtf8Error, returning a new IoError.
fn from_utf8_error_into_io_error(_: FromUtf8Error) -> IoError {
    IoError { kind: OtherIoError, desc: "UTF error", detail: None }
}