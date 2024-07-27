use crate::hitable::{Hitable,HitInfo};
use crate::math::*;
use crate::ray::Ray;


pub struct Sphere {
    pub centre: Vec3f,
    pub color: Vec3f,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitInfo> {
        let Ray {direction, origin, ..} = ray;
        let relative_origin = origin - self.centre;

        let a = direction.dot(direction);
        let b = 2.0*direction.dot(&relative_origin);
        let c = relative_origin.dot(&relative_origin) - self.radius * self.radius;

        let delta = b*b - 4.0*a*c;

        if delta < 0.0 { return None }

        let delta_sqrt = delta.sqrt();

        let t1 = (-b - delta_sqrt) / (2.0*a);
        let t2 = (-b + delta_sqrt) / (2.0*a);

        
        if min_t <= t1 && t1 <= max_t {
            let point = origin + t1*direction;
            let normal = (point - self.centre) / self.radius;
            return Some(HitInfo{
                point,
                normal,
                color: self.color,
                t: t1
            });
        }
        else if min_t <= t2 && t2 <= max_t {
            let point = origin + t2*direction;
            let normal = (point - self.centre) / self.radius;
            return Some(HitInfo{
                point,
                normal,
                color: self.color,
                t: t2
            });
        }

        None
    }
}