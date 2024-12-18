use rand::seq::SliceRandom;
use ray_tracer as rt;

use core::f32;
use std::time::Instant;
use rt::{Vec3f, Vec2f, Object, UVec3f};

fn main() {
    let camera = rt::Camera::new(Vec3f::new(0.0,2.0,-3.0), Vec3f::z() -  Vec3f::y() / 3.0, Vec3f::y(), 60.0 / 180. * f32::consts::PI);

    let plane = rt::Mesh::new(
        &[
            rt::Vertex { position: Vec3f::new( 1.0,0.0,-1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new( 1.0,0.0, 1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new(-1.0,0.0, 1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new(-1.0,0.0,-1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
        ],
        &[[0,1,2], [2,3,0]]
    );
    
    let mut scene = rt::Scene::new(camera, Box::new(rt::SkyEnvironment {
            sun_direction: Vec3f::new(1.0, -1.0,1.0).normalize(),
            sun_color: Vec3f::new(10.0,10.0,7.0),
            up_color: Vec3f::new(0.5,0.6,1.5)/2.0,
            down_color: Vec3f::new(0.8,0.9,1.0)/2.0,
            sun_size: 0.01
        }));


    let floor_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(0.9,0.9,0.9), emission: Vec3f::zeros()}));
    let glass_material = scene.add_material(Box::new(rt::Dialectric {refraction_index: 1.5}));
    let gray_material  = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(0.8,0.8,0.8), emission: Vec3f::zeros()}));
    let stone_material  = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(0.5,0.5,0.5), emission: Vec3f::zeros()}));
    let redio_active_material  = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(44., 250., 31.)/255., emission: Vec3f::zeros()}));
    let gold_material  = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(220.0, 150.0, 20.0)/255.0, roughness:0.3}));
    let iron_material  = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(94., 98., 107.) / 255., roughness:0.2}));

    let materials = [gray_material, glass_material, stone_material, gold_material, iron_material, redio_active_material];
    
    let plane_handle = scene.add_mesh(plane);
    let soldier_handle = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/soldier.obj").unwrap());

    let floor=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,-1.1,0.0))*rt::scale(100.0,100.0,100.0), 
        floor_material);
    let floor_shiny=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,-1.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::x()), f32::consts::PI)*rt::scale(100.0,100.0,100.0), 
        glass_material);

    const SOLDER_GRID_WIDTH : u32 = 20;
    const SOLDER_GRID_HEIGHT : u32 = 20;
    
    for i in 1..SOLDER_GRID_WIDTH+1 {
        for j in 1..SOLDER_GRID_HEIGHT+1 {
            let x = i as f32 - SOLDER_GRID_WIDTH as f32 / 2.0;
            let z = j as f32 * 1.2;

            let soldier: Object=  rt::Object::new(soldier_handle, rt::translate(&Vec3f::new(x,-1.0,z))
                *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
                *materials.choose(&mut rand::thread_rng()).unwrap());
            scene.add_object(soldier);
        }
    }
     
    
    scene.add_object(floor);
    scene.add_object(floor_shiny);

    println!("Saving the scene to: ./army.json ...");
    scene.save("./army.json").expect("Could not save scene");
}