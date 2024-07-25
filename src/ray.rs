use crate::math::*;
use crate::image::Colorf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin : Vec3f,
    pub direction : Vec3f,
    pub color : Colorf
}