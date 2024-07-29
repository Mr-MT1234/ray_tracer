use core::f32;

pub use nalgebra as na;
use rand::Rng;

pub type Vec3f = na::Vector3<f32>;

#[inline]
pub fn reflect(vec: &Vec3f, normal: &Vec3f) -> Vec3f {
    vec - 2.0*normal.dot(vec)*normal
}

#[inline]
pub fn mul_element_wise(v1: Vec3f, v2: Vec3f) -> Vec3f {
    Vec3f::new( v1.x*v2.x,v1.y*v2.y,v1.z*v2.z )
}

#[inline]
pub fn random_uniform_unit() -> Vec3f {
    let mut rng = rand::thread_rng();

    let u : f32= rng.gen::<f32>() * 2.0 - 1.0;
    let theta :f32 = rng.gen::<f32>() * 2.0 * f32::consts::PI;

    let ring_radius = (1.0-u*u).sqrt();

    Vec3f::new(ring_radius*theta.cos(), ring_radius*theta.sin(), u)

    
}