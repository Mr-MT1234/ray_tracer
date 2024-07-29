use std::ops::{Index, IndexMut};
use crate::math::*;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Color {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
//     pub a: u8
// }

// impl Color {
//     pub const BLACK : Color = Color {r:0, g:0, b:0, a: 255};

//     pub fn new(r: u8,g: u8,b: u8, a: u8) -> Color{
//         Color {r,g,b, a}
//     }
// }

// impl Into<Color> for Vec3f {
//     fn into(self) -> Color {
//         Color {
//             r: (self.x * 255.) as u8,
//             g: (self.y * 255.) as u8,
//             b: (self.z * 255.) as u8,
//             a: 255,
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Image {
    pub pixels : Vec<Vec3f>,
    pub width  : u16,
    pub height : u16
}


impl Image {
    pub fn new(color : Vec3f, width : u16, height : u16) -> Image{
        Image {
            pixels: (0..(width as usize)*(height as usize)).into_iter().map(|_| color).collect(),
            width,
            height
        }
    }

    pub fn fill(&mut self, color: Vec3f) {
        for p in &mut self.pixels {
            *p = color;
        }
    } 
}

impl Image {
    pub fn get_size(&self) -> (u16,u16) {
        (self.width, self.height)
    }

    pub fn get_pixels(&self) -> &Vec<Vec3f> {
        &self.pixels
    }

    pub fn get_pixels_mut(&mut self) -> &mut Vec<Vec3f> {
        &mut self.pixels
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

#[cfg(test)]
mod test {
}