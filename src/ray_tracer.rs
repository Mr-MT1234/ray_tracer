use itertools::Itertools;

use crate::parallel;
use crate::{Image, image::RenderTraget};

use crate::{CollisionReport, ImageView, RenderReport, ScaterInfo, Vec3f};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::hitables::*;
use crate::commun_types::Ray;
use crate::math::*;
use core::sync;
use std::mem;
use std::num::{NonZeroI16, NonZeroUsize};
use std::ops::DerefMut;
use std::thread;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub max_depth: u32,
    pub rays_per_pixel: u32
}

pub struct RayTracer;

impl RayTracer {
    pub fn render(&self, scene: &Scene, target: &mut Image, options: &RenderOptions) -> RenderReport {
        const TILE_SIZE : u32 = 32;
        
        let render_report = RenderReport::default();
        let camera = &scene.camera;

        let scene_ref = Arc::new(scene);

        let tiles = target.split_tiles(TILE_SIZE, TILE_SIZE);

        let tasks = tiles.map(|tile| (tile, scene_ref.clone()))
            .map(|(mut tile, scene)| { 
                let task = Box::new(move || {
                    for (ray, pixel) in Self::shoot_at_tile(camera.clone(), &tile, options.rays_per_pixel) {
                        
                        let (c, _) = self.trace(&ray, &scene,0, options.max_depth);
                        
                        tile[pixel] += c;
            
                        //TODO: Render report
                    }

                    for i in 0..tile.height as usize {
                        for j in 0..tile.width as usize {
                            tile[[i,j]] /= options.rays_per_pixel as f32;
                        }
                    }
            }) as Box<dyn FnOnce()->()>;
        
            let task : parallel::Task = unsafe {mem::transmute(task)};
            task
        });

        let worker_count = thread::available_parallelism().map(|i| i.get()).unwrap_or(16);
        parallel::parallel_execute(tasks,worker_count);

        render_report
    }

    pub fn render_with_print(&self, scene: &Scene, target: &mut Image, options: &RenderOptions) -> RenderReport {
        const TILE_SIZE : u32 = 32;

        let render_report = RenderReport::default();
        let camera = &scene.camera;

        let scene_ref = Arc::new(scene);

        let tile_count = target.split_tiles(TILE_SIZE, TILE_SIZE).count();
        let tiles = target.split_tiles(TILE_SIZE, TILE_SIZE);

        let tiles_done = Mutex::new(0usize);

        let tasks = tiles.map(|tile| (tile, scene_ref.clone()))
            .map(|(mut tile, scene)| { 
                let tiles_done = &tiles_done;
                let task = Box::new(move || {
                    for (ray, pixel) in Self::shoot_at_tile(camera.clone(), &tile, options.rays_per_pixel) {
                        
                        let (c, _) = self.trace(&ray, &scene,0, options.max_depth);
                        
                        tile[pixel] += c;
            
                        //TODO: Render report
                    }

                    for i in 0..tile.height as usize {
                        for j in 0..tile.width as usize {
                            tile[[i,j]] /= options.rays_per_pixel as f32;
                        }
                    }

                    let mut tiles_done = tiles_done.lock().unwrap();
                    *tiles_done += 1;
                    
                    const PROGRESS_BAR_LENGHT : usize = 100;
                    let percentage = *tiles_done as f32 / tile_count as f32;
                    let progress_bar : String = (0..PROGRESS_BAR_LENGHT).map(|i| if i as f32 <= (percentage * PROGRESS_BAR_LENGHT as f32) {'█'} else {'-'})
                                                                        .collect();
                    print!("\rRendering: [{progress_bar}] {:.2}", percentage*100.);
            }) as Box<dyn FnOnce()->()>;
        
            let task : parallel::Task = unsafe {mem::transmute(task)};
            task
        });


        let worker_count = thread::available_parallelism().map(|i| i.get()).unwrap_or(16);
        parallel::parallel_execute(tasks,worker_count);

        render_report
    }
    
    pub fn render_with_print_single_thread(&self, scene: &Scene, target: &mut Image, options: &RenderOptions) -> RenderReport {
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
                CollisionReport {
                    aabb_tests: report1.aabb_tests + report2.aabb_tests,
                    triangle_tests: report1.triangle_tests + report2.triangle_tests
                }
            )
        } else {
            (scene.environment.sample(&ray.direction), report1)
        }
    }

    fn shoot_at(camera: Camera, (width, height): (u32, u32), rays_per_pixel: u32) -> impl Iterator<Item = (Ray, [usize;2])> +'static {

        // let right = camera.direction.cross(&camera.up);
        // let up  = right.cross(&camera.direction);
        // let tan_fov = (camera.fov/2.0).tan();
        // let aspect_ratio = height as f32/ width as f32;
        // let direction = camera.direction;
        // let origin = camera.origin;

        // (0..height as usize).cartesian_product(0..width as usize).cartesian_product(0..rays_per_pixel as usize)
        // .map(move |((i,j), _)| {
        //     let relative_x = ( (j as f32 + rand::random::<f32>()) / width as f32  - 0.5) * tan_fov;
        //     let relative_y = (-(i as f32 + rand::random::<f32>()) / height as f32 + 0.5) * tan_fov * aspect_ratio;

        //     let pixel_in_plane = relative_x * right + relative_y * up;
        //     let direction = direction + pixel_in_plane;

        //     (
        //         Ray {
        //             origin: origin,
        //             direction,
        //         }, 
        //         [i,j]
        //     )
        // })

        RayIterator::new(camera, 0, 0, width as usize, height as usize,
            width as usize, height as usize, rays_per_pixel)
    }

    fn shoot_at_tile(camera: Camera, tile: &ImageView, rays_per_pixel: u32) -> impl Iterator<Item = (Ray, [usize;2])> +'static {

        // let right = camera.direction.cross(&camera.up);
        // let up  = right.cross(&camera.direction);
        // let tan_fov = (camera.fov/2.0).tan();
        // let aspect_ratio = height as f32/ width as f32;
        // let direction = camera.direction;
        // let origin = camera.origin;

        // (0..height as usize).cartesian_product(0..width as usize).cartesian_product(0..rays_per_pixel as usize)
        // .map(move |((i,j), _)| {
        //     let relative_x = ( (j as f32 + rand::random::<f32>()) / width as f32  - 0.5) * tan_fov;
        //     let relative_y = (-(i as f32 + rand::random::<f32>()) / height as f32 + 0.5) * tan_fov * aspect_ratio;

        //     let pixel_in_plane = relative_x * right + relative_y * up;
        //     let direction = direction + pixel_in_plane;

        //     (
        //         Ray {
        //             origin: origin,
        //             direction,
        //         }, 
        //         [i,j]
        //     )
        // })

        RayIterator::new(camera, tile.offset_x as usize,  tile.offset_y as usize,
            tile.width as usize, tile.height as usize,  
            tile.source_width as usize, tile.source_height as usize, 
            rays_per_pixel
        )
    }
}


pub struct RayIterator {
    camera: Camera,

    offset_x: usize,
    offset_y: usize,
    width: usize,
    height: usize,
    source_width: usize,
    source_height: usize,
    

    rays_per_pixel: u32,
    tan_fov: f32,
    aspect_ratio: f32,

    current_pixel_i: usize,
    current_pixel_j: usize,
    current_ray_index: u32,
}


impl<'a> RayIterator {
    fn new(camera: Camera, offset_x: usize, offset_y: usize, width: usize, height: usize, source_width: usize, source_height: usize, rays_per_pixel: u32) -> RayIterator{
        RayIterator {
            tan_fov: camera.fov.tan(),
            camera,
            offset_x,
            offset_y,
            width,
            height,
            aspect_ratio: source_height as f32 / source_width as f32,
            rays_per_pixel,
            current_pixel_i: offset_y,
            current_pixel_j: offset_x,
            current_ray_index: 0,
            source_width,
            source_height
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
        if self.current_pixel_j == self.width + self.offset_x {
            self.current_pixel_j = self.offset_x;
            self.current_pixel_i += 1;
        }
        if self.current_pixel_i == self.height + self.offset_y{
            return None;
        }

        let right = self.camera.direction.cross(&self.camera.up);
        let up  = right.cross(&self.camera.direction);

        let relative_x = ((self.current_pixel_j as f32 + rand::random::<f32>()) / self.source_width as f32  - 0.5) * self.tan_fov;
        let relative_y = (-(self.current_pixel_i as f32 + rand::random::<f32>()) / self.source_height as f32 + 0.5) * self.tan_fov * self.aspect_ratio;

        let pixel_in_plane = relative_x * right + relative_y * up;
        let direction = self.camera.direction + pixel_in_plane;

        self.current_ray_index += 1;

        Some((
            Ray {
                origin: self.camera.origin,
                direction,
            }, 
            [self.current_pixel_i - self.offset_y,self.current_pixel_j - self.offset_x]
        ))
    }
}
