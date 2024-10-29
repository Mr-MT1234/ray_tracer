use serde::{Deserialize, Serialize};

use crate::math::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub origin : Vec3f,
    pub direction : Vec3f,
    pub up: Vec3f,
    pub fov: f32,
}

impl Camera {
    pub fn new(origin: Vec3f, direction : Vec3f, up: Vec3f, fov : f32) -> Camera {
        Camera {
            origin,
            direction: direction.normalize(),
            up: up.normalize(),
            fov,
        }
    }
}
