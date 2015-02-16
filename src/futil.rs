use cgmath::Vector2;
use cgmath::Vector3;
use std::old_io::File;

pub fn read_string_16(file: &mut File) -> String {
    let length: u16 = file.read_be_u16().unwrap();
    String::from_utf8(
        file.read_exact(length as usize).unwrap()
    ).unwrap()
}

/*
pub fn read_string_32(file: &mut File) -> String {
    let length: u32 = file.read_be_u32().unwrap();
    String::from_utf8(
        file.read_exact(length as usize).unwrap()
    ).unwrap()
}

pub fn read_string_64(file: &mut File) -> String {
    let length: u64 = file.read_be_u64().unwrap();
    String::from_utf8(
        file.read_exact(length as usize).unwrap()
    ).unwrap()
}*/

pub fn read_vector_2(file: &mut File) -> Vector2<f32> {
    Vector2 {
        x: file.read_be_f32().unwrap(),
        y: file.read_be_f32().unwrap()
    }
}

pub fn read_vector_3(file: &mut File) -> Vector3<f32> {
    Vector3 {
        x: file.read_be_f32().unwrap(),
        y: file.read_be_f32().unwrap(),
        z: file.read_be_f32().unwrap()
    }
}