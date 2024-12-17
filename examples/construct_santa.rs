use ray_tracer as rt;

use core::f32;
use std::time::Instant;
use rt::{Vec3f, Vec2f, Object, UVec3f};

fn main() {
    let camera = rt::Camera::new(Vec3f::new(0.0,1.0,-3.0), Vec3f::z(), Vec3f::y(), 50.0 / 180. * f32::consts::PI);
    
    
    let mut scene = rt::Scene::new(camera, Box::new(rt::SkyEnvironment {
            sun_direction: Vec3f::new(1.0, -1.0,1.0).normalize(),
            sun_color: Vec3f::new(10.0,10.0,9.0),
            up_color: Vec3f::new(0.6,0.7,1.0)/2.0,
            down_color: Vec3f::new(1.0,1.0,1.0)/2.0,
            sun_size: 0.01
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
    let anti_glass_material = scene.add_material(Box::new(rt::Dialectric{refraction_index: 1.0/1.5}));
    let gold_material = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(220.0, 150.0, 20.0)/255.0, roughness:0.3}));
    let mirror_material = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(0.9, 1.0, 0.9), roughness:0.0}));
    
    let plane_handle = scene.add_mesh(plane);
    let santa_red = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/red.obj").unwrap());
    let santa_black = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/black.obj").unwrap());
    let santa_white = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/white.obj").unwrap());
    let santa_yellow = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/yellow.obj").unwrap());
    let santa_bleu = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/blue.obj").unwrap());
    let santa_green = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/green.obj").unwrap());
    let santa_gold = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/gold.obj").unwrap());
    let santa_skin = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/skin.obj").unwrap());
    let santa_eyes = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/santa/eyes.obj").unwrap());

    let floor=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,-1.0,0.0))*rt::scale(10.0,10.0,10.0), 
                gray_material);


    let santa_red: Object=  rt::Object::new(santa_red, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        pink_material);
    let santa_black: Object=  rt::Object::new(santa_black, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        bleu_material);
    let santa_white: Object=  rt::Object::new(santa_white, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        white_material);
    let santa_yellow: Object=  rt::Object::new(santa_yellow, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        white_material);
    let santa_bleu: Object=  rt::Object::new(santa_bleu, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        white_material);
    let santa_green: Object=  rt::Object::new(santa_green, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        white_material);
    let santa_gold: Object=  rt::Object::new(santa_gold, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        white_material);
    let santa_skin: Object=  rt::Object::new(santa_skin, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        white_material);
    let santa_eyes: Object=  rt::Object::new(santa_eyes, rt::translate(&Vec3f::new(0.0,0.0,0.0))
        *rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::PI)*rt::scale(1.0,1.0,1.0), 
        white_material);
    
    scene.add_object(floor);
    scene.add_object(santa_red);
    scene.add_object(santa_black);
    scene.add_object(santa_white);
    scene.add_object(santa_yellow);
    scene.add_object(santa_bleu);
    scene.add_object(santa_green);
    scene.add_object(santa_gold);
    scene.add_object(santa_skin);
    scene.add_object(santa_eyes);

    println!("Saving the scene to: ./santa.json ...");
    scene.save("./santa.json").expect("Could not save scene");
}