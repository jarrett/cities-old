use std::path::Path;
use image;
use image::GenericImage;

pub trait Source {
    fn x_verts(&self) -> usize;
    fn y_verts(&self) -> usize;
    
    fn vert_z_at(&self, abs_x: usize, abs_y: usize) -> f32;
}

pub struct ImageSource {
    image: image::DynamicImage,
    z_scale: f32
}

impl ImageSource {
    pub fn new(path: &Path, z_scale: f32) -> ImageSource {
        let dyn_img: image::DynamicImage = image::open(&path).unwrap();       
        ImageSource { image: dyn_img, z_scale: z_scale }
    }
}

impl Source for ImageSource {
    fn x_verts(&self) -> usize {
        let (x_verts, _) = self.image.dimensions();
        x_verts as usize
    }
    
    fn y_verts(&self) -> usize {
        let (_, y_verts) = self.image.dimensions();
        y_verts as usize
    }
    
    fn vert_z_at(&self, abs_x: usize, abs_y: usize) -> f32 {
        self.image.get_pixel(abs_x as u32, abs_y as u32)[0] as f32 * self.z_scale
    }
}