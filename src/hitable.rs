use crate::{material::Material, math::*, ray::Ray};


#[derive(Clone, Copy)]
pub struct HitInfo<'a> {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub material: &'a dyn Material,
    pub t :f32
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitInfo>;
}