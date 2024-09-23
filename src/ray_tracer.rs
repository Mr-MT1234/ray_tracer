use crate::render_target::RenderTarget;
use crate::{CollisionReport, RenderReport, ScaterInfo, Vec3f};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::hitables::*;
use crate::commun_types::Ray;
use crate::math::*;

pub struct RayTracer {
    max_depth: u32,
}

impl RayTracer {
    pub fn new() -> RayTracer {
        RayTracer {
            max_depth:10,
        }
    }

    pub fn accumulate(&self, scene: &Scene,camera: &Camera, target: &mut RenderTarget) -> RenderReport {

        let mut render_report = RenderReport::default();

        for (ray, pixel) in camera.shoot_at(target.get_size(), 1) {
            
            let (c, report) = self.trace(&ray, scene,0);
            
            target.accumulate(&c, pixel);

            render_report.aabb_tests += report.aabb_tests;
            render_report.triangle_tests += report.triangle_tests;
        }

        render_report
    }
    
    fn trace(&self, ray: &Ray, scene: &Scene, depth: u32) -> (Vec3f, CollisionReport) {
        if depth >= self.max_depth {
            return (Vec3f::zeros(), CollisionReport::default());
        }

        let (hit, report1) = scene.hit(&ray, 0.01, f32::INFINITY);

        if let Some(info) = hit {
            let ScaterInfo { ray: new_ray, attenuation, emission} = info.material.scater(ray.direction, &info);

            let (scatered, report2) = self.trace(&new_ray, scene, depth + 1);
            (
                mul_element_wise(scatered, attenuation) + emission,
                CollisionReport {
                    aabb_tests: report1.aabb_tests + report2.aabb_tests,
                    triangle_tests: report1.triangle_tests + report2.triangle_tests
                }
            )
        } else {
            (Vec3f::new(0.8,0.8,0.9), report1)
        }
    }
}