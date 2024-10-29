use std::ops::{Index, IndexMut};
use std::path::Path;

use crate::math::*;

#[derive(Debug, Clone)]
pub struct Image {
    pub pixels : Vec<Vec3f>,
    pub width  : u32,
    pub height : u32
}

impl Image {
    pub fn new(color : Vec3f, width : u32, height : u32) -> Image{
        Image {
            pixels: (0..(width as usize)*(height as usize)).into_iter().map(|_| color).collect(),
            width,
            height
        }
    }

    pub fn fill(&mut self, color: Vec3f) {
        self.pixels.fill(color);
    } 

    pub fn get_resolution(&self) -> (u32,u32) {
        (self.width, self.height)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        let bytes: Vec<u8> = self.pixels.iter()
                                        .map(|vec| vec.iter())
                                        .flatten()
                                        .map(|x| (x.powf(1.0/2.2) * 255.0) as u8) //Gamma correction
                                        .collect();

        image::save_buffer(path, &bytes, self.width as u32, self.height as u32, image::ColorType::Rgb8)?;
        Ok(())
    }
}

impl Index<[usize;2]> for Image {
    type Output = Vec3f;
    fn index(&self, [i,j]: [usize;2]) -> &Self::Output {
        &self.pixels[i*(self.width as usize) + j]
    }
}

impl IndexMut<[usize;2]> for Image {
    fn index_mut (&mut self, [i,j]: [usize;2]) -> &mut Self::Output {
        &mut self.pixels[i*(self.width as usize) + j]
    }
}