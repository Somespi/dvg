use std::{fs::File, io::Write, error::Error};


pub struct DvgImage {
    data: Vec<u8>,
    width: u32,
    height: u32
}

impl DvgImage {
    pub fn new(width: u32, height: u32) -> DvgImage {
        DvgImage {
            data: vec![0u8; (width  * height ) as usize],
            width,
            height
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        self.data[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: u8) {
        self.data[(y * self.width + x) as usize] = pixel
    }

    pub fn save(self, path: &str)  {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.width.to_le_bytes()).unwrap();
        file.write_all(&self.height.to_le_bytes()).unwrap();
        file.write_all(&self.data).unwrap();
        Ok::<(),()>(());
    }
}