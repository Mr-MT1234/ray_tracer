use core::f32;

pub use nalgebra as na;
use rand::Rng;

pub type Vec2f = na::Vector2<f32>;
pub type Vec3f = na::Vector3<f32>;
pub type Vec4f = na::Vector4<f32>;
pub type UVec3f = na::UnitVector3<f32>;
pub type Mat4f = na::Matrix4<f32>;
pub type Mat3f = na::Matrix3<f32>;

#[inline]
pub fn reflect(vec: &Vec3f, normal: &Vec3f) -> Vec3f {
    vec - 2.0*normal.dot(vec)*normal
}

#[inline]
pub fn vec3_to_vec4(vec3: &Vec3f, w: f32) -> Vec4f {
    Vec4f::new(vec3.x,vec3.y,vec3.z,w)
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

#[inline] 
pub fn rotation(axis: &UVec3f, angle: f32) -> Mat4f {
    na::Rotation::from_axis_angle(axis, angle).to_homogeneous()
}

#[inline] 
pub fn translate(translation: &Vec3f) -> Mat4f {
    let mut identity = Mat4f::identity();
    identity.set_column(3, &vec3_to_vec4(translation, 1.0));
    identity
}

#[inline] 
pub fn scale(x:f32, y:f32, z:f32) -> Mat4f {
    Mat4f::from_diagonal(&Vec4f::new(x,y,z,1.0))
}