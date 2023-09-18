use std::{fs::File, io::Write};

use image::Luma;


pub struct DvgImage {
    data: Vec<Luma<u8>>,
    width: u32,
    height: u32
}

impl DvgImage {
    pub fn new(width: u32, height: u32) -> DvgImage {
        DvgImage {
            data: vec![Luma([0u8]); (width * height) as usize],
            width,
            height
        }
    }

    pub fn put_pixel(&mut self, x: &u32, y: &u32, pixel: Luma<u8>) {
        self.data[(x + y * self.width) as usize] = pixel;
    }

    pub fn get_pixel(&self, x: &u32, y: &u32) -> Luma<u8> {
        self.data[(x + y * self.width) as usize]
    }


    pub fn save(&self, path: &str)  {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.width.to_le_bytes()).unwrap();
        file.write_all(&self.height.to_le_bytes()).unwrap();
        let data = &self.data.iter().map(|pixel| pixel[0]).collect::<Vec<u8>>();
        file.write_all(&data).unwrap();
        let _ = Ok::<(),()>(());
    }
}