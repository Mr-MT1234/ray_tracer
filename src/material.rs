use crate::math::*;

pub trait Material {
    fn scater(&self, in_direction: Vec3f, normal: Vec3f) -> Vec3f;
    fn get_color(&self) -> Vec3f;
}

pub struct Lambertian {
    pub color : Vec3f
}

impl Material for Lambertian {
    fn scater(&self, _in_direction: Vec3f, normal: Vec3f) -> Vec3f {
        let direction = random_uniform_unit();
        let direction = (direction + normal).normalize();
        direction
    }
    fn get_color(&self) -> Vec3f {
        self.color
    }
}

