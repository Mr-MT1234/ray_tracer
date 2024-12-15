use ray_tracer as rt;

use core::f32;
use std::time::Instant;
use rt::{Vec3f, Vec2f, Object, UVec3f};

fn main() {
    let camera = rt::Camera::new(Vec3f::new(0.0,0.0,-3.0), Vec3f::z(), Vec3f::y(), 50.0 / 180. * f32::consts::PI);
    
    
    let mut scene = rt::Scene::new(camera, Box::new(rt::SkyEnvironment {
            sun_direction: Vec3f::new(1.0, -1.0,1.0).normalize(),
            sun_color: Vec3f::new(100.0,100.0,90.0),
            up_color: Vec3f::new(0.6,0.7,1.0)/2.0,
            down_color: Vec3f::new(1.0,1.0,1.0)/2.0,
            sun_size: 0.001
        }));

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
    let gray_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(0.8,0.8,0.8), emission: Vec3f::zeros()}));
    let bleu_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(68., 66., 219.) / 255., emission: Vec3f::zeros()}));
    let pink_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(209., 56., 125.) / 255., emission: Vec3f::zeros()}));
    let glowing_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::zeros(), emission: Vec3f::new(10., 10., 10.)}));
    let glass_material = scene.add_material(Box::new(rt::Dialectric{refraction_index: 1.5}));
    let gold_material = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(220.0, 150.0, 20.0)/255.0, roughness:0.3}));
    let mirror_material = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(0.9, 1.0, 0.9), roughness:0.0}));
    
    let plane_handle = scene.add_mesh(plane);
    let sphere_handle = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/sphere.obj").unwrap());
    let dragon_handle = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/dragon.obj").unwrap());
    let bunny_handle = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/bunny.obj").unwrap());
    let cube_handle = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/cube.obj").unwrap());

    let floor=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,-1.0,0.0))*rt::scale(10.0,10.0,10.0), 
                gray_material);

    let bunny: Object=  rt::Object::new(bunny_handle, rt::translate(&Vec3f::new(0.5,-0.85,-0.2))*rt::rotation(&UVec3f::new_normalize(Vec3f::y()), -3.0*f32::consts::PI/4.0)*rt::scale(0.15, 0.15, 0.15), 
        glass_material);

    let dragon: Object=  rt::Object::new(dragon_handle, rt::translate(&Vec3f::new(-0.3,-1.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), -f32::consts::PI/4.0)*rt::scale(1.0,1.0,1.0), 
        gold_material);
    
    scene.add_object(floor);
    scene.add_object(bunny);
    scene.add_object(dragon);

    println!("Saving the scene to: ./outside.json ...");
    scene.save("./outside.json").expect("Could not save scene");
}