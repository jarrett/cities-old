use std::iter::repeat;
use image::{DynamicImage, GenericImage, ImageFormat, Rgba};

pub trait Adapter {
    fn put_pixel(&mut self, x: u32, y: u32, opacity: u8);
    fn w(&self) -> u32;
    fn h(&self) -> u32;
    
    fn safe_put_pixel(&mut self, x: u32, y: u32, opacity: u8) {
        let w = self.w();
        let h = self.h();
        if x >= w || y >= h {
            panic!(
                "Tried to put pixel out-of-bounds at {}, {}. Adapter dimensions: {}, {}.",
                x, y, w, h
            );
        }
        self.put_pixel(x, y, opacity);
    }
}

pub struct DynamicImageAdapter {
    pub img: DynamicImage
}

pub struct RGBA8Adapter {
    pub w: u32,
    pub h: u32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub buffer: Vec<u8>
}

impl DynamicImageAdapter {
    pub fn new(img: DynamicImage) -> DynamicImageAdapter {
        DynamicImageAdapter {img: img}
    }
}

impl Adapter for DynamicImageAdapter {
    fn put_pixel(&mut self, x: u32, y: u32, opacity: u8) {
        let pixel: Rgba<u8> = Rgba([0, 0, 0, opacity]);
        self.img.put_pixel(x, y, pixel);
    }
    
    fn w(&self) -> u32 { self.img.dimensions().0 }
    fn h(&self) -> u32 { self.img.dimensions().1 }
}

impl RGBA8Adapter {
    pub fn new(w: u32, h: u32, r: u8, g: u8, b: u8) {
        RGBA8Adapter {
            w: w, h: h, r: r, g: g, b: b,
            buffer: repeat(0).take((w * h * 4) as usize).collect()
        };
    }
}

impl Adapter for RGBA8Adapter {
    fn put_pixel(&mut self, x: u32, y: u32, opacity: u8) {
        let offset = ((y * self.w + x) * 4) as usize;
        self.buffer[offset + 0] = self.r;
        self.buffer[offset + 1] = self.g;
        self.buffer[offset + 2] = self.b;
        self.buffer[offset + 3] = opacity;
    }
    
    fn w(&self) -> u32 { self.w }
    fn h(&self) -> u32 { self.h }
}