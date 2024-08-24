use core::f32;

use crate::hitables::*;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Object>
} 

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: Vec::new() }
    }
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene { objects: Vec::new() }
    }
}

impl Optical for Scene {
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