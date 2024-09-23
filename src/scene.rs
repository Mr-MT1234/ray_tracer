use core::f32;

use crate::{hitables::*, CollisionReport};
use crate::commun_types::Ray;

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
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> (Option<HitInfo>, CollisionReport) {
        let mut hit_info: Option<HitInfo> = None;
        let mut report = CollisionReport::default();
        for object in &self.objects {
            let old_t = hit_info.as_ref().map(|hit| hit.t).unwrap_or(max_t);
            let (new_hit, new_report) = object.hit(ray, min_t, old_t);
            let new_t = new_hit.as_ref().map(|hit| hit.t).unwrap_or(f32::INFINITY);
            report.aabb_tests += new_report.aabb_tests;
            report.triangle_tests += new_report.triangle_tests;
            if new_t < old_t {
                hit_info = new_hit;
            }
        }

        (hit_info, report)
    }
}