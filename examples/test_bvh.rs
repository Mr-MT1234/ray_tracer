use commun::InputManager;
use show_image::event::VirtualKeyCode;
use show_image::WindowOptions;
use show_image::{ImageView, ImageInfo, create_window, event::WindowEvent};
use ray_tracer::{self as rt, Lambertian, Optical};

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::f32;
use std::sync::{Arc, Mutex};
use rt::{Vec3f, Vec2f, Mat4f, Object, UVec3f};

mod commun;

fn set_pixel([i,j]: [usize;2], value: rt::Vec3f, data: &mut [u8], width: usize) {
    data[3*(i*width + j) + 0] = (value.x*255.0) as u8;
    data[3*(i*width + j) + 1] = (value.y*255.0) as u8;
    data[3*(i*width + j) + 2] = (value.z*255.0) as u8;
}

fn get_color(report : &rt::CollisionReport) -> Vec3f {
    const MAX_AABB_TEST: f32 = 200.0;
    const MAX_TRIANGLE_TEST: f32 = 100.0;

    let mut r = report.aabb_tests as f32 / MAX_AABB_TEST;
    let mut b = report.triangle_tests as f32/ MAX_TRIANGLE_TEST;

    let g = ((r > 1.0) || (b > 1.0)) as u8 as f32;
    if g > 0.0 {(r,b) = (0.0,0.0)}

    Vec3f::new(r, g, b)
}

static INPUT_MANAGER : Mutex<RefCell<commun::InputManager>> = Mutex::new(RefCell::new(commun::InputManager::new()));

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: u16 = 1000;
    const HEIGHT: u16 = 600;

    let window = create_window("image", WindowOptions {
        size: Some([WIDTH as u32,HEIGHT as u32]),
        ..Default::default()
    })?;
    
    let mut data: Vec<_> = (0..WIDTH as usize * HEIGHT as usize * 3).map(|_| 0u8).collect();

    let _ = window.add_event_handler(|_,event,_| (*INPUT_MANAGER.lock().unwrap()).borrow_mut().handle_event(event));

    let mut model = rt::Object::new(
        rt::Mesh::load_obj("./examples/assets/bunny.obj").unwrap(),
        rt::Mat4f::identity(),
        Box::new(Lambertian {color: Vec3f::zeros(), emission: Vec3f::zeros()})
    );

    println!("BVH constructed");
    println!("\tDepth               : {}", model.mesh.bvh.depth());
    println!("\tMax Triangle Count  : {}", model.mesh.bvh.max_triangle_count());
    println!("\tAvg Triangle Count  : {}", model.mesh.bvh.avg_triangle_count());

    let small_rotation = rt::rotation(&UVec3f::new_normalize(Vec3f::y()), 1e-1);

    let camera = rt::Camera::new(Vec3f::new(0.0,0.0,-8.0), Vec3f::z(), Vec3f::y(), 60.0 / 180. * f32::consts::PI);

    loop {
        for (ray, pixel) in camera.shoot_at((WIDTH, HEIGHT), 1) {
            let (_, report) = model.hit(&ray, 0.0, f32::INFINITY);

            let color = get_color(&report);
            set_pixel(pixel, color, &mut data, WIDTH as usize);
        }

        model.set_transform(model.get_transform()*small_rotation);
    
        let window_image = ImageView::new(ImageInfo::rgb8(WIDTH as u32, HEIGHT as u32), &data);
        window.set_image("image-001", window_image)?;

        let manager =  INPUT_MANAGER.lock().unwrap();
        
        if (*manager).borrow_mut().is_key_pressed(VirtualKeyCode::A) {
            println!("A was pressed");
        }
        
        (*manager).borrow_mut().end_of_frame();
    }

    Ok(())
}
