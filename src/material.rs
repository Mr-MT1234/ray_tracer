use serde::Deserialize;
use serde::Serialize;

use crate::math::*;
use crate::HitInfo;
use crate::Ray;

pub struct ScaterInfo {
    pub ray: Ray,
    pub attenuation: Vec3f,
    pub emission: Vec3f
} 

#[typetag::serde(tag="type")]
pub trait Material : core::fmt::Debug {
    fn scater(&self, in_direction: Vec3f, hit_info: &HitInfo) -> ScaterInfo;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lambertian {
    pub color : Vec3f,
    pub emission: Vec3f
}
#[typetag::serde]
impl Material for Lambertian {
    fn scater(&self, _in_direction: Vec3f, hit_info: &HitInfo) -> ScaterInfo {
        let direction = random_uniform_unit();
        let direction = (direction + hit_info.normal).normalize();
        let ray = Ray {direction, origin: hit_info.point};
        ScaterInfo {
            ray,
            attenuation: self.color,
            emission: self.emission
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialectric {
    pub refraction_index: f32,
}

#[typetag::serde]
impl Material for Dialectric {
    fn scater(&self,mut  in_direction: Vec3f, hit_info: &HitInfo) -> ScaterInfo {

        let n = if hit_info.inside {1.0/self.refraction_index} else {self.refraction_index};
        let normal = &hit_info.normal;
        in_direction = in_direction.normalize();

        let cos = -in_direction.dot(&normal);

        let out_tangential = (in_direction + cos * normal) / n;
        let a = 1.0 - out_tangential.norm_squared();

        let direction = if  a < 0.0 || Dialectric::reflectance(cos,n) > rand::random() {
            reflect(&in_direction, &normal)
        }
        else {
            out_tangential - a.sqrt()*normal
        };

        let ray = Ray {direction, origin: hit_info.point};
        ScaterInfo {
            ray, 
            attenuation: Vec3f::new(1.0,1.0,1.0),
            emission: Vec3f::zeros(),
        }
    }
}

impl Dialectric {
    fn reflectance(cosine: f32, n: f32) -> f32{
        // Schlick's approximation.
        let mut r0 = (1.0 - n) / (1.0 + n);
        r0 = r0*r0;
        return r0 + (1.0-r0)*((1.0 - cosine).powf(5.0));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metal {
    pub color: Vec3f,
    pub roughness: f32,
}

#[typetag::serde]
impl Material for Metal {
    fn scater(&self, in_direction: Vec3f, hit_info: &HitInfo) -> ScaterInfo {
        let direction = reflect(&in_direction, &hit_info.normal) + self.roughness*random_uniform_unit();
        let ray = Ray {direction, origin: hit_info.point};
        ScaterInfo {
            ray, 
            attenuation: Vec3f::new(1.0,1.0,1.0),
            emission: Vec3f::zeros(),
        }    
    }
}
