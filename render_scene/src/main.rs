use ray_tracer as rt;

use core::f32;
use std::time::Instant;
use rt::{Vec3f, Vec2f, Object, UVec3f};

fn main() {
    const WIDTH: u32 = 1500;
    const HEIGHT: u32 = 1500;
    
    let camera = rt::Camera::new(Vec3f::new(0.0,0.0,-3.0), Vec3f::z(), Vec3f::y(), 45.0 / 180. * f32::consts::PI);
    let renderer = rt::RayTracer;
    
    
    let mut scene = rt::Scene::new(camera);

    let plane = rt::Mesh::new(
        &[
            rt::Vertex { position: Vec3f::new( 1.0,0.0,-1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new( 1.0,0.0, 1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new(-1.0,0.0, 1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new(-1.0,0.0,-1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
        ],
        &[[0,1,2], [2,3,0]]
    );

    let white_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(1.0,1.0,1.0), emission: Vec3f::zeros()}));
    let bleu_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(68., 66., 219.) / 255., emission: Vec3f::zeros()}));
    let pink_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(209., 56., 125.) / 255., emission: Vec3f::zeros()}));
    let glowing_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::zeros(), emission: Vec3f::new(10., 10., 10.)}));
    let glass_material = scene.add_material(Box::new(rt::Dialectric{refraction_index: 1.5}));
    
    let plane_handle = scene.add_mesh(plane);
    let sphere_handle = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/sphere.obj").unwrap());
    let cube_handle = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/cube.obj").unwrap());

    let floor=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,-1.0,0.0)), 
                white_material);

    let ceiling=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,1.0,0.0)), 
                white_material);

    let front_wall: Object=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,0.0,1.0))*rt::rotation(&UVec3f::new_normalize(Vec3f::x()), f32::consts::FRAC_PI_2), 
        white_material);

    let left_wall: Object=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(1.0,0.0,0.0))*rt::rotation(&UVec3f::new_normalize(Vec3f::z()), f32::consts::FRAC_PI_2), 
        bleu_material
    );

    let right_wall: Object=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(-1.0,0.0,0.0))*rt::rotation(&UVec3f::new_normalize(Vec3f::z()), f32::consts::FRAC_PI_2), 
        pink_material
    );

    let lamp: Object=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,0.99,0.0))*rt::scale(0.2, 0.2, 0.2), 
        glowing_material
    );

    let sphere: Object=  rt::Object::new(sphere_handle, rt::translate(&Vec3f::new(0.5,-0.7,0.0))*rt::scale(0.3, 0.3, 0.3), 
        glass_material);

    let cube: Object=  rt::Object::new(cube_handle, rt::translate(&Vec3f::new(-0.5,-0.7,0.0))*rt::scale(0.3, 0.3, 0.3)
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI / 12.0), 
        white_material);
    
    scene.add_object(floor);
    scene.add_object(ceiling);
    scene.add_object(left_wall);
    scene.add_object(right_wall);
    scene.add_object(front_wall);
    scene.add_object(lamp);
    scene.add_object(sphere);
    scene.add_object(cube);

    println!("saving cornell box scene at cornell_box.json");
    scene.save("./cornell_box.json").expect("failed to save scene");
    
    let mut target = rt::Image::new(Vec3f::zeros(), WIDTH, HEIGHT);

    let options = rt::RenderOptions { max_depth: 10, rays_per_pixel: 100};

    let start = Instant::now();
    renderer.render_with_print(&scene, &mut target, &options);
    println!("Rendering finished after {}s", start.elapsed().as_secs_f32());

    target.save("./cornell_box 100.png").expect("Failed to save render result");
}
