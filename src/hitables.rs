use crate::BVH::BVH;

use crate::{material::Material, math::*, ray::Ray, colliders::*};


#[derive(Clone, Copy)]
pub struct HitInfo<'a> {
    pub point: Vec3f,
    pub normal: Vec3f,
    pub material: &'a dyn Material,
    pub t :f32,
    pub inside: bool,
}

pub trait Optical {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitInfo>;
}

#[derive(Debug)]
pub struct Object {
    mesh: Mesh,
    transform: Mat4f,
    inv_transform: Mat4f,
    normal_mat: Mat3f,
    material: Box<dyn Material + Send + Sync>,
}

impl Object {
    pub fn new(mut mesh: Mesh, transform: Mat4f, material: Box<dyn Material + Send + Sync>) -> Object {
        let inv_transform = transform.try_inverse().unwrap();
        let normal_mat = Mat3f::new(
            inv_transform.m11,inv_transform.m21, inv_transform.m31,
            inv_transform.m12,inv_transform.m22, inv_transform.m32,
            inv_transform.m13,inv_transform.m23, inv_transform.m33
        );
        
        Object {
            mesh,
            material,
            transform,
            inv_transform,
            normal_mat,
        }
    }
}

impl Optical for Object {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitInfo> {
        let local_ray = Ray {
            origin: (self.inv_transform * vec3_to_vec4(&ray.origin,1.0)).xyz(),
            direction: (self.inv_transform * vec3_to_vec4(&ray.direction,0.0)).xyz(),
        };
        let collision = self.mesh.collide(&local_ray, min_t, max_t);
        
        collision.map(|info| HitInfo {
            point: (self.transform * vec3_to_vec4(&info.point,1.0)).xyz(),
            normal: self.normal_mat * info.normal,
            material: self.material.as_ref(),
            t: info.t,
            inside: info.inside,
        })
    }
}

