use crate::{Image, Vec3f};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::hitable::*;
use crate::ray::Ray;
use crate::math::*;

pub struct RayTracer {
    max_depth: u32,
}

impl RayTracer {
    pub fn new() -> RayTracer {
        RayTracer {
            max_depth:10,
        }
    }

    pub fn accumulate(&self, scene: &Scene,camera: &Camera, image: &mut Image) {

        for (ray, pixel) in camera.shoot_at(image.get_size(), 1) {
            
            let c = self.trace(&ray, scene,0);
            
            
            image[pixel] = c.into();
        }
    }
    
    fn trace(&self, ray: &Ray, scene: &Scene, depth: u32) -> Vec3f {
        if depth >= self.max_depth {
            return Vec3f::zeros();
        }

        let hit = scene.hit(&ray, 0.0001, f32::INFINITY);

        if let Some(info) = hit {
            let new_dir = reflect(&ray.direction, &info.normal);
            let new_ray = Ray {
                direction: new_dir,
                origin: info.point,
            };
            mul_element_wise(&self.trace(&new_ray, scene, depth + 1),&info.color)
        } else {
            Vec3f::new(0.1,0.1,0.1)
        }
    }
}