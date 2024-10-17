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

#[derive(Debug)]
pub struct Object {
    pub mesh: Mesh,
    transform: Mat4f,
    inv_transform: Mat4f,
    normal_mat: Mat3f,
    pub material: Box<dyn Material>,
}

impl Object {
    pub fn new(mesh: Mesh, transform: Mat4f, material: Box<dyn Material>) -> Object {
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

    pub fn get_transform(&self) -> &Mat4f {
        &self.transform
    }

    pub fn set_transform(&mut self, new_transform: Mat4f) {
        let inv_transform = new_transform.try_inverse().unwrap();
        let normal_mat = Mat3f::new(
            inv_transform.m11,inv_transform.m21, inv_transform.m31,
            inv_transform.m12,inv_transform.m22, inv_transform.m32,
            inv_transform.m13,inv_transform.m23, inv_transform.m33
        );

        self.transform = new_transform;
        self.inv_transform = inv_transform;
        self.normal_mat = normal_mat;
    }
}

impl Optical for Object {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> (Option<HitInfo>, CollisionReport) {
        let local_ray = Ray {
            origin: (self.inv_transform * vec3_to_vec4(&ray.origin,1.0)).xyz(),
            direction: (self.inv_transform * vec3_to_vec4(&ray.direction,0.0)).xyz(),
        };
        let (collision, report) = self.mesh.collide(&local_ray, min_t, max_t);
        
        let collision = collision.map(|info| HitInfo {
            point: (self.transform * vec3_to_vec4(&info.point,1.0)).xyz(),
            normal: (self.normal_mat * info.normal).normalize(),
            material: self.material.as_ref(),
            t: info.t,
            inside: info.inside,
            uv: info.uv
        });

        (collision, report)
    }
}

