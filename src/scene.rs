use core::f32;

use crate::hitable::*;
use crate::ray::Ray;
use crate::math::*;

pub struct Scene {
    objects: Vec<Box<dyn Hitable>>
} 

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: Vec::new() }
    }
    pub fn add_object(&mut self, object: Box<dyn Hitable>) {
        self.objects.push(object);
    }
}

impl Hitable for Scene {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;

        for object in &self.objects {
            let old_t = hit_info.as_ref().map(|hit| hit.t).unwrap_or(max_t);
            let new_hit = object.hit(ray, min_t, old_t);
            let new_t = new_hit.as_ref().map(|hit| hit.t).unwrap_or(f32::INFINITY);

            if new_t < old_t {
                hit_info = new_hit;
            }
        }

        hit_info
    }
}