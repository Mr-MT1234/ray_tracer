use crate::math::*;
use crate::commun_types::Ray;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin : Vec3f,
    pub direction : Vec3f,
    pub up: Vec3f,
    pub fov: f32
}

pub struct RayIterator {
    camera: Camera,

    width: usize,
    height: usize,
    rays_per_pixel: u16,
    tan_fov: f32,
    aspect_ratio: f32,

    current_pixel_i: usize,
    current_pixel_j: usize,
    current_ray_index: u16,
}


impl<'a> RayIterator {
    fn new(camera: Camera, width: usize, height: usize, rays_per_pixel: u16) -> RayIterator{
        RayIterator {
            camera,
            width,
            height,
            tan_fov: camera.fov.tan(),
            aspect_ratio: height as f32 / width as f32,
            rays_per_pixel,
            current_pixel_i:0,
            current_pixel_j:0,
            current_ray_index: 0,
        }
    }
}

impl<'a> Iterator for RayIterator {
    type Item = (Ray, [usize;2]);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ray_index == self.rays_per_pixel {
            self.current_ray_index = 0;
            self.current_pixel_j += 1;
        }
        if self.current_pixel_j == self.width {
            self.current_pixel_j = 0;
            self.current_pixel_i += 1;
        }
        if self.current_pixel_i == self.height {
            return None;
        }

        let right = self.camera.direction.cross(&self.camera.up);
        let up  = right.cross(&self.camera.direction);

        let relative_x = ((self.current_pixel_j as f32 + rand::random::<f32>()) / self.width as f32  - 0.5) * self.tan_fov;
        let relative_y = (-(self.current_pixel_i as f32 + rand::random::<f32>()) / self.height as f32 + 0.5) * self.tan_fov * self.aspect_ratio;

        let pixel_in_plane = relative_x * right + relative_y * up;
        let direction = self.camera.direction + pixel_in_plane;

        self.current_ray_index += 1;

        Some((
            Ray {
                origin: self.camera.origin,
                direction,
            }, 
            [self.current_pixel_i,self.current_pixel_j]
        ))
    }
}


pub struct RayRandomIterator {
    origin: Vec3f,
    direction: Vec3f,
    up: Vec3f,
    right: Vec3f,

    width: usize,
    height: usize,
    ray_count: u32,
    tan_fov: f32,
    aspect_ratio: f32,

    current_ray_index: u32,
}


impl<'a> RayRandomIterator {
    fn new(camera: &Camera, width: usize, height: usize, ray_count: u32) -> RayRandomIterator{
        let Camera {direction, origin, up, ..} = *camera;
        let right = direction.cross(&up);
        let up  = right.cross(&direction);
        
        RayRandomIterator {
            origin,
            direction,
            up, 
            right,
            width,
            height,
            tan_fov: camera.fov.tan(),
            aspect_ratio: height as f32 / width as f32,
            ray_count,
            current_ray_index: 0,
        }
    }
}

impl<'a> Iterator for RayRandomIterator {
    type Item = (Ray, [usize;2]);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ray_index == self.ray_count {
            return None
        }

        let j = rand::thread_rng().gen_range(0..self.width);
        let i = rand::thread_rng().gen_range(0..self.height);

        let relative_x = ((j as f32 + rand::random::<f32>()) / self.width as f32  - 0.5) * self.tan_fov;
        let relative_y = (-(i as f32 + rand::random::<f32>()) / self.height as f32 + 0.5) * self.tan_fov * self.aspect_ratio;

        let pixel_in_plane = relative_x * self.right + relative_y * self.up;
        let direction = self.direction + pixel_in_plane;

        self.current_ray_index += 1;

        Some((
            Ray {
                origin: self.origin,
                direction,
            }, 
            [i,j]
        ))
    }
}

impl Camera {
    pub fn new(origin: Vec3f, direction : Vec3f, up: Vec3f, fov : f32) -> Camera {
        Camera {
            origin,
            direction: direction.normalize(),
            up: up.normalize(),
            fov
        }
    }

    pub fn shoot_at(&self, window_size: (u16, u16), rays_per_pixel: u16) -> RayIterator {
        let (width, height) = window_size;
        let (width, height) = (width as usize, height as usize);

        RayIterator::new(self.clone(), width, height, rays_per_pixel)
    }

    pub fn shoot_rand(&self, window_size: (u16, u16), ray_count: u32) -> RayRandomIterator {
        let (width, height) = window_size;
        let (width, height) = (width as usize, height as usize);

        RayRandomIterator::new(self, width, height, ray_count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn camera_shoot_rays_count() {
        let camera = Camera::new(Vec3f::zeros(), Vec3f::z(), Vec3f::y(), std::f32::consts::PI/4.0);

        let rays_count = camera.shoot_at((2,2), 2).count();

        assert_eq!(rays_count, 8);
    }
}

