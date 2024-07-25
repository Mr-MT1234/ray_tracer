use crate::{Colorf, Image, Color};
use crate::scene::Scene;
use crate::camera::Camera;

pub struct RayTracer {
    
}

impl RayTracer {
    pub fn new() -> RayTracer {
        RayTracer {}
    }

    pub fn accumulate(&self, _scene: &Scene,camera: &Camera, image: &mut Image) {
        for (ray, pixel) in camera.shoot_at(image.get_size(), 1) {
            let [x,y,z] : [f32;3] = ray.direction.into();

            
            let c = Colorf::new(x.max(0.),y.max(0.),z.max(0.));
            
            image[pixel] = c.into();
        }
    }
}