#![macro_use]

use cgmath::{Point, Point2, Point3, Vector2, Vector3};
use std::fs::File;
use std::io;
use std::string::FromUtf8Error;

#[allow(dead_code)]
pub fn read_string_16(file: &mut File) -> Result<String, io::Error> {
    let length = try!(file.read_be_u16());
    let bytes = try!(file.read_exact(length as usize));
    String::from_utf8(bytes).map_err(from_utf8_error_into_io_error)
}

#[allow(dead_code)]
pub fn write_str_16(file: &mut File, string: &str) -> Result<(), io::Error> {
    try!(file.write_be_u16(string.len() as u16));    
    file.write_str(string)
}

#[allow(dead_code)]
pub fn write_string_16(file: &mut File, string: &String) -> Result<(), io::Error> {
    write_str_16(file, string.as_slice())
}

#[allow(dead_code)]
pub fn read_point_2(file: &mut File) -> Result<Point2<f32>, io::Error> {
    let x = try!(file.read_be_f32());
    let y = try!(file.read_be_f32());
    Ok(Point2 { x: x, y: y })
}

#[allow(dead_code)]
pub fn read_point_3(file: &mut File) -> Result<Point3<f32>, io::Error> {
    let x = try!(file.read_be_f32());
    let y = try!(file.read_be_f32());
    let z = try!(file.read_be_f32());    
    Ok(Point3 { x: x, y: y, z: z })
}

#[allow(dead_code)]
pub fn read_vector_2(file: &mut File) -> Result<Vector2<f32>, io::Error> {
    read_point_2(file).map(|p| { p.to_vec() })
}

#[allow(dead_code)]
pub fn read_vector_3(file: &mut File) -> Result<Vector3<f32>, io::Error> {
    read_point_3(file).map(|p| { p.to_vec() })
}

#[allow(dead_code)]
pub fn write_point_2(file: &mut File, v: &Point2<f32>) -> Result<(), io::Error> {
    try!(file.write_be_f32(v.x));
    file.write_be_f32(v.y)
}

#[allow(dead_code)]
pub fn write_point_3(file: &mut File, v: &Point3<f32>) -> Result<(), io::Error> {
    try!(file.write_be_f32(v.x));
    try!(file.write_be_f32(v.y));
    file.write_be_f32(v.z)
}

// Consumes a FromUtf8Error, returning a new io::Error.
fn from_utf8_error_into_io_error(_: FromUtf8Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, "UTF-8 error");
}

pub type IoErrorLine = (io::Error, &'static str, usize);

macro_rules! tryln {
    ($expr:expr) => (try!(
        ($expr).map_err({ |e|
            (e, file!(), line!())
        })
    ))
}