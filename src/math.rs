pub use nalgebra as na;

pub type Vec3f = na::Vector3<f32>;

#[inline]
pub fn reflect(vec: &Vec3f, normal: &Vec3f) -> Vec3f {
    vec - 2.0*normal.dot(vec)*normal
}

#[inline]
pub fn mul_element_wise(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
    Vec3f::new( v1.x*v2.x,v1.y*v2.y,v1.z*v2.z )
}
