use std::io::Cursor;
use std::ops::{Index, IndexMut};
use std::path::Path;

use crate::{Image, Vec3f};

pub struct RenderTarget {
    image: Image,
    accumulation_count: u32,
}

impl RenderTarget {
    pub fn new(width: u16, height: u16) -> RenderTarget {
        RenderTarget {
            image: Image::new(Vec3f::zeros(), width, height),
            accumulation_count: 0,
        }
    }

    pub fn accumulate(&mut self, color: &Vec3f, pixel: [usize;2]) {
        self.image[pixel] += color;
        self.accumulation_count += 1;
    }

    pub fn get_result(&self,  pixel: [usize;2]) -> Vec3f {
        let pixel_count = self.image.get_pixels().len() as f32;
        self.image[pixel] / (self.accumulation_count as f32 / pixel_count)
    }

    pub fn clear(&mut self) {
        self.image.fill(Vec3f::zeros());
        self.accumulation_count = 0;
    }

    pub fn get_size(&self) -> (u16,u16) {
        self.image.get_size()
    }

    pub fn get_image_mut(&mut self) -> &mut Image {
        &mut self.image
    }

    pub fn save(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        let pixel_count = self.image.get_pixels().len() as f32;
        let bytes: Vec<u8> = self.image.pixels.iter().map(|vec| vec.iter())
                                                     .flatten()
                                                     .map(|x| (x / (self.accumulation_count as f32 / pixel_count) * 255.0) as u8)
                                                     .collect();

        image::save_buffer(path, &bytes, self.image.width as u32, self.image.height as u32, image::ColorType::Rgb8)?;

        Ok(())
    }
}

impl Index<[usize;2]> for RenderTarget{
    type Output = Vec3f;
    fn index(&self, index: [usize;2]) -> &Self::Output {
        &self.image[index] 
    }
}

impl IndexMut<[usize;2]> for RenderTarget{
    fn index_mut(&mut self, index: [usize;2]) -> &mut Self::Output {
        &mut self.image[index] 
    }
}

