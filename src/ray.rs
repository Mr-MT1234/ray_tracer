use crate::math::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin : Vec3f,
    pub direction : Vec3f,
}