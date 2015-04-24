#![macro_use]

use cgmath::{Point, Point2, Point3, Vector2, Vector3};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

pub type IoErrorLine = (io::Error, &'static str, u32);

#[allow(dead_code)]
pub fn read_string_16(file: &mut File) -> Result<String, io::Error> {
    let length = try!(file.read_u16::<BigEndian>()) as usize;
    let mut string = String::with_capacity(length);
    try!(file.take(length as u64).read_to_string(&mut string));
    Ok(string)
}

#[allow(dead_code)]
pub fn write_str_16(file: &mut File, string: &str) -> Result<(usize), io::Error> {
    try!(file.write_u16::<BigEndian>(string.len() as u16));    
    file.write(string.as_bytes())
}

#[allow(dead_code)]
pub fn write_string_16(file: &mut File, string: &String) -> Result<(usize), io::Error> {
    write_str_16(file, string.as_str())
}

#[allow(dead_code)]
pub fn read_point_2(file: &mut File) -> Result<Point2<f32>, io::Error> {
    let x = try!(file.read_f32::<BigEndian>());
    let y = try!(file.read_f32::<BigEndian>());
    Ok(Point2 { x: x, y: y })
}

#[allow(dead_code)]
pub fn read_point_3(file: &mut File) -> Result<Point3<f32>, io::Error> {
    let x = try!(file.read_f32::<BigEndian>());
    let y = try!(file.read_f32::<BigEndian>());
    let z = try!(file.read_f32::<BigEndian>());    
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
    try!(file.write_f32::<BigEndian>(v.x));
    try!(file.write_f32::<BigEndian>(v.y));
    Ok(())
}

#[allow(dead_code)]
pub fn write_point_3(file: &mut File, v: &Point3<f32>) -> Result<(), io::Error> {
    try!(file.write_f32::<BigEndian>(v.x));
    try!(file.write_f32::<BigEndian>(v.y));
    try!(file.write_f32::<BigEndian>(v.z));
    Ok(())
}