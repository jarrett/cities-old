use cgmath::Vector2;
use cgmath::Vector3;
use std::old_io::{File, IoError};
use std::old_io::IoErrorKind::OtherIoError;
use std::string::FromUtf8Error;

#[allow(dead_code)]
pub fn read_string_16(file: &mut File) -> Result<String, IoError> {
    file.read_be_u16().and_then({ |length|
        file.read_exact(length as usize).and_then({ |bytes|
            String::from_utf8(bytes).map_err(from_utf8_error_into_io_error)
        })
    })
}

#[allow(dead_code)]
pub fn read_string_32(file: &mut File) -> Result<String, IoError> {
    file.read_be_u32().and_then({ |length|
        file.read_exact(length as usize).and_then({ |bytes|
            String::from_utf8(bytes).map_err(from_utf8_error_into_io_error)
        })
    })
}

#[allow(dead_code)]
pub fn read_string_64(file: &mut File) -> Result<String, IoError> {
    file.read_be_u64().and_then({ |length|
        file.read_exact(length as usize).and_then({ |bytes|
            String::from_utf8(bytes).map_err(from_utf8_error_into_io_error)
        })
    })
}

#[allow(dead_code)]
pub fn write_string_16(file: &mut File, string: &String) -> Result<(), IoError> {
    file.write_be_u16(string.len() as u16).and_then({ |_|
        file.write_str(string.as_slice())
    })
}

#[allow(dead_code)]
pub fn read_vector_2(file: &mut File) -> Result<Vector2<f32>, IoError> {
    Ok(Vector2 {
        x: file.read_be_f32().unwrap(),
        y: file.read_be_f32().unwrap()
    })
}

#[allow(dead_code)]
pub fn read_vector_3(file: &mut File) -> Result<Vector3<f32>, IoError> {
    file.read_be_f32().and_then({ |x: f32|
        file.read_be_f32().map({ |y: f32| -> (f32, f32) (x, y) })
    }).and_then({ |(x, y)|
        file.read_be_f32().map({ |z: f32| -> (f32, f32, f32) (x, y, z) })
    }).map({ |(x, y, z)|
        Vector3 { x: x, y: y, z: z }
    })
}

#[allow(dead_code)]
pub fn write_vector_2(file: &mut File, v: &Vector2<f32>) -> Result<(), IoError> {
    file.write_be_f32(v.x).and_then({ |()|
        file.write_be_f32(v.y)
    })
}

#[allow(dead_code)]
pub fn write_vector_3(file: &mut File, v: &Vector3<f32>) -> Result<(), IoError> {
    file.write_be_f32(v.x).and_then({ |()|
        file.write_be_f32(v.y)
    }).and_then({ |()|
        file.write_be_f32(v.z)
    })
}

// Consumes a FromUtf8Error, returning a new IoError.
fn from_utf8_error_into_io_error(_: FromUtf8Error) -> IoError {
    IoError { kind: OtherIoError, desc: "UTF error", detail: None }
}