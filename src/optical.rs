use crate::bvhs::BVH;

use crate::{material::Material, math::*, commun_types::Ray, colliders::*};


#[derive(Clone, Copy)]
pub struct HitInfo<'a> {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub uv: Vec2f,
    pub material: &'a dyn Material,
    pub t :f32,
    pub inside: bool,
}

pub trait Optical {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> (Option<HitInfo>, CollisionReport);
}