use crate::Image;

use crate::{CollisionReport, RenderReport, ScaterInfo, Vec3f};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::hitables::*;
use crate::commun_types::Ray;
use crate::math::*;

#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub max_depth: u32,
    pub rays_per_pixel: u32,
}

pub struct RayTracer;

impl RayTracer {
    pub fn render(&self, scene: &Scene, target: &mut Image, options: &RenderOptions) -> RenderReport {
        let mut render_report = RenderReport::default();
        let camera = &scene.camera;

        for (ray, pixel) in Self::shoot_at(camera.clone(), target.get_resolution(), options.rays_per_pixel) {
            
            let (c, report) = self.trace(&ray, scene,0, options.max_depth);
            
            target[pixel] += c;

            render_report.aabb_tests += report.aabb_tests;
            render_report.triangle_tests += report.triangle_tests;
        }

        render_report
    }

    pub fn render_with_print(&self, scene: &Scene, target: &mut Image, options: &RenderOptions) -> RenderReport {
        let mut render_report = RenderReport::default();
        let camera = &scene.camera;
        let resolution = target.get_resolution();

        let mut current_sample_index = 0;
        let total_sample_count = resolution.0*resolution.1*options.rays_per_pixel;
        let hundredth_of_persantile = total_sample_count / 100000;

        for (ray, pixel) in Self::shoot_at(camera.clone(), resolution, options.rays_per_pixel) {
            
            let (c, report) = self.trace(&ray, scene,0, options.max_depth);
            
            target[pixel] += c;
            
            render_report.aabb_tests += report.aabb_tests;
            render_report.triangle_tests += report.triangle_tests;
            current_sample_index += 1;
            
            if current_sample_index % hundredth_of_persantile == 0 {
                const PROGRESS_BAR_LENGHT : usize = 100;
                let percentage = current_sample_index as f32 / total_sample_count as f32;
                let progress_bar : String = (0..PROGRESS_BAR_LENGHT).map(|i| if i as f32 <= (percentage * PROGRESS_BAR_LENGHT as f32) {'█'} else {'-'})
                                                                    .collect();
                print!("\rRendering: [{progress_bar}] {:.2}", percentage*100.);
            }
        }

        for i in 0..target.pixels.len() {
            target.pixels[i] /= options.rays_per_pixel as f32;
        }

        render_report
    }
    
    fn trace(&self, ray: &Ray, scene: &Scene, depth: u32, max_depth: u32) -> (Vec3f, CollisionReport) {
        if depth >= max_depth {
            return (Vec3f::zeros(), CollisionReport::default());
        }

        let (hit, report1) = scene.hit(&ray, 0.01, f32::INFINITY);

        if let Some(info) = hit {
            let ScaterInfo { ray: new_ray, attenuation, emission} = info.material.scater(ray.direction, &info);

            let (scatered, report2) = self.trace(&new_ray, scene, depth + 1, max_depth);
            (
                mul_element_wise(scatered, attenuation) + emission,
                // info.normal,
                CollisionReport {
                    aabb_tests: report1.aabb_tests + report2.aabb_tests,
                    triangle_tests: report1.triangle_tests + report2.triangle_tests
                }
            )
        } else {
            (Vec3f::new(0.1,0.1,0.1), report1)
            // (Vec3f::new(0.9,0.9,1.0), report1)
        }
    }

    fn shoot_at(camera: Camera, resolution: (u32, u32), rays_per_pixel: u32) -> RayIterator {
        RayIterator::new(camera, resolution.0 as usize, resolution.1 as usize, rays_per_pixel)
    }
}


pub struct RayIterator {
    camera: Camera,

    width: usize,
    height: usize,
    rays_per_pixel: u32,
    tan_fov: f32,
    aspect_ratio: f32,

    current_pixel_i: usize,
    current_pixel_j: usize,
    current_ray_index: u32,
}


impl<'a> RayIterator {
    fn new(camera: Camera, width: usize, height: usize, rays_per_pixel: u32) -> RayIterator{
        RayIterator {
            tan_fov: camera.fov.tan(),
            camera,
            width,
            height,
            aspect_ratio: height as f32 / width as f32,
            rays_per_pixel,
            current_pixel_i:0,
            current_pixel_j:0,
            current_ray_index: 0,
        }
    }
}

impl<'a> Iterator for RayIterator {
    type Item = (Ray, [usize;2]);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ray_index == self.rays_per_pixel {
            self.current_ray_index = 0;
            self.current_pixel_j += 1;
        }
        if self.current_pixel_j == self.width {
            self.current_pixel_j = 0;
            self.current_pixel_i += 1;
        }
        if self.current_pixel_i == self.height {
            return None;
        }

        let right = self.camera.direction.cross(&self.camera.up);
        let up  = right.cross(&self.camera.direction);

        let relative_x = ((self.current_pixel_j as f32 + rand::random::<f32>()) / self.width as f32  - 0.5) * self.tan_fov;
        let relative_y = (-(self.current_pixel_i as f32 + rand::random::<f32>()) / self.height as f32 + 0.5) * self.tan_fov * self.aspect_ratio;

        let pixel_in_plane = relative_x * right + relative_y * up;
        let direction = self.camera.direction + pixel_in_plane;

        self.current_ray_index += 1;

        Some((
            Ray {
                origin: self.camera.origin,
                direction,
            }, 
            [self.current_pixel_i,self.current_pixel_j]
        ))
    }
}
