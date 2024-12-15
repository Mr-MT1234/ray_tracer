use std::ops::{Index, IndexMut};
use std::path::Path;
use std::slice::from_raw_parts_mut;
use itertools::Itertools;

use crate::math::*;


pub trait RenderTraget : Index<[usize;2]> + IndexMut<[usize;2]> {
    fn get_resolution(&self) -> (u32,u32);
}

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

    pub fn save(&self, path: impl AsRef<Path>) -> image::ImageResult<()> {
        let bytes: Vec<u8> = self.pixels.iter()
                                        .map(|vec| vec.iter())
                                        .flatten()
                                        .map(|x| (x.powf(1.0/2.2) * 255.0) as u8) //Gamma correction
                                        .collect();

        image::save_buffer(path, &bytes, self.width as u32, self.height as u32, image::ColorType::Rgb8)?;
        Ok(())
    }

    pub fn view(&mut self, offset_x: u32, offset_y: u32, width: u32, height: u32) -> ImageView {
        ImageView {
            source: &mut self.pixels,
            offset_x,
            offset_y,
            width,
            height,
            source_width: self.width,
            source_height: self.height,
        }
    }

    
    
    pub fn split_tiles(& mut self, width: u32, height: u32) -> TileIterator {
        TileIterator {
            tile_count_h: (self.height as f32 / height as f32).ceil() as u32,
            tile_count_w: (self.width as f32 / width as f32).ceil() as u32,
            width,
            height,
            i:0,
            j:0,
            source: self,
        }
    }
}

impl RenderTraget for Image {
    fn get_resolution(&self) -> (u32,u32) {
        (self.width, self.height)
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

#[derive(Debug)]
pub struct ImageView<'a> {
    pub source : &'a mut [Vec3f],
    pub source_width: u32,
    pub source_height: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width  : u32,
    pub height : u32
}

impl<'a> RenderTraget for ImageView<'a> {
    fn get_resolution(&self) -> (u32,u32) {
        (self.width, self.height)
    }
}

impl<'a> Index<[usize;2]> for ImageView<'a> {
    type Output = Vec3f;
    fn index(&self, [i,j]: [usize;2]) -> &Self::Output {
        assert!(i < self.height  as usize && j < self.width as usize, "index out of bound");
        let x = self.offset_x as usize + j;
        let y = (self.offset_y as usize + i)*(self.source_width as usize);
        &self.source[x + y]
    }
}

impl<'a> IndexMut<[usize;2]> for ImageView<'a> {
    fn index_mut (&mut self, [i,j]: [usize;2]) -> &mut Self::Output {
        assert!(i < self.height  as usize && j < self.width as usize, "index out of bound");
        let x = self.offset_x as usize + j;
        let y = (self.offset_y as usize + i)*(self.source_width as usize);
        &mut self.source[x + y]
    }
}

impl<'a> ImageView<'a> {
    pub fn fill(&mut self, color: Vec3f) {
        for i in 0..self.height as usize{
            for j in 0..self.width as usize {
                self[[i,j]] = color;
            }
        }
    } 
}

#[derive(Debug)]
pub struct TileIterator<'a> {
    source: &'a mut Image,
    width: u32, 
    height: u32,
    tile_count_w: u32,
    tile_count_h: u32,
    i: u32,
    j: u32
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = ImageView<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.tile_count_w {
            self.j = 0;
            self.i += 1;
        }
        if self.i >= self.tile_count_h {
            return None;
        }
        let ptr = self.source.pixels.as_mut_ptr();
        let len = self.source.pixels.len();
        let tile = unsafe {
            let offset_x = self.j*self.width;
            let offset_y = self.i*self.height;
            Some(ImageView {
                source: from_raw_parts_mut(ptr, len),
                offset_x,
                offset_y ,
                width: self.width.min(self.source.width - offset_x),
                height: self.height.min(self.source.height - offset_y),
                source_width: self.source.width,
                source_height: self.source.height,
            })
        };
        self.j += 1;
        tile
    }
}


mod test {
    use super::*;
    #[test]
    fn test_shit() {
        let mut image = Image::new(Vec3f::zeros(), 2, 2);
        let mut tiles: Vec<_> = image.split_tiles(1, 1).collect();
        

        assert_eq!(tiles.len(), 4);
        dbg!(&tiles);
        tiles[0].fill(Vec3f::new(1.0, 0.0, 0.0));
        tiles[1].fill(Vec3f::new(0.0, 1.0, 0.0));
        tiles[2].fill(Vec3f::new(1.0, 1.0, 0.0));
        tiles[3].fill(Vec3f::new(0.0, 0.0, 1.0));

        assert_eq!(image.pixels, [Vec3f::new(1.0, 0.0, 0.0),
        Vec3f::new(0.0, 1.0, 0.0),
        Vec3f::new(1.0, 1.0, 0.0),
        Vec3f::new(0.0, 0.0, 1.0),]);
    }
}