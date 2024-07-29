use std::ops::{Index, IndexMut};

use crate::{Image, Vec3f};

pub struct RenderTarget {
    image: Image,
    iteration_count: u32,
}

impl RenderTarget {
    pub fn new(width: u16, height: u16) -> RenderTarget {
        RenderTarget {
            image: Image::new(Vec3f::zeros(), width, height),
            iteration_count: 0,
        }
    }

    pub fn accumulate(&mut self, color: &Vec3f, pixel: [usize;2]) {
        self.image[pixel] += color;
    }

    pub fn get_iterations(&self) -> u32 {
        self.iteration_count
    }

    pub fn get_result(&self,  pixel: [usize;2]) -> Vec3f {
        self.image[pixel] / self.iteration_count as f32
    }

    pub fn next_iteration(&mut self) {
        self.iteration_count += 1;
    }

    pub fn clear(&mut self) {
        self.image.fill(Vec3f::zeros());
        self.iteration_count = 0;
    }

    pub fn get_size(&self) -> (u16,u16) {
        self.image.get_size()
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