use crate::{math::*, ray::Ray};


#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub color: Vec3f,
    pub t :f32
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitInfo>;
}