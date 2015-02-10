use image;
use image::GenericImage;

pub trait Source {
    fn x_verts(&self) -> u32;
    fn y_verts(&self) -> u32;
    
    fn vert_z_at(&self, abs_x: u32, abs_y: u32) -> f32;
}

pub struct ImageSource {
    image: image::DynamicImage,
    z_scale: f32
}

impl ImageSource {
    pub fn new(path: &Path, z_scale: f32) -> ImageSource {
        let dyn_img: image::DynamicImage = image::open(path).unwrap();       
        ImageSource { image: dyn_img, z_scale: z_scale }
    }
}

impl Source for ImageSource {
    fn x_verts(&self) -> u32 {
        let (x_verts, _) = self.image.dimensions();
        x_verts
    }
    
    fn y_verts(&self) -> u32 {
        let (_, y_verts) = self.image.dimensions();
        y_verts
    }
    
    fn vert_z_at(&self, abs_x: u32, abs_y: u32) -> f32 {
        self.image.get_pixel(abs_x, abs_y)[0] as f32 * self.z_scale
    }
}