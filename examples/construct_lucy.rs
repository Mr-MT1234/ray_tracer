use ray_tracer as rt;

use core::f32;
use std::time::Instant;
use rt::{Vec3f, Vec2f, Object, UVec3f};

fn main() {
    let camera = rt::Camera::new(Vec3f::new(0.0,0.0,-3.4), Vec3f::z(), Vec3f::y(), 45.0 / 180. * f32::consts::PI);
    
    let mut scene = rt::Scene::new(camera, Box::new(rt::ConstantEnvironment {color: Vec3f::new(0.001,0.001,0.001)}) );

    let plane = rt::Mesh::new(
        &[
            rt::Vertex { position: Vec3f::new( 1.0,0.0,-1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new( 1.0,0.0, 1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new(-1.0,0.0, 1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new(-1.0,0.0,-1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
        ],
        &[[0,1,2], [2,3,0]]
    );

    let faint_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(1.0,1.0,1.0), emission: Vec3f::new(0.04,0.05,0.1)}));
    let white_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(1.0,1.0,1.0), emission: Vec3f::zeros()}));
    let glowing_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::zeros(), emission: Vec3f::new(30., 30., 30.)}));
    let mirror_material = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(0.9, 0.95, 1.0), roughness:0.0}));
    let bleu_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(68., 66., 219.) / 255., emission: Vec3f::zeros()}));
    
    let plane_handle = scene.add_mesh(plane);
    let lucy = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/lucy.obj").unwrap());
    let lucy_flame = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/lucy_flame.obj").unwrap());
    let lucy_shell = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/lucy_shell_outer.obj").unwrap());

    let floor = rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,-1.0,0.0))*rt::scale(3.5,3.5,3.5), faint_material);

    let ceiling =  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,1.0,0.0))*rt::scale(3.5,3.5,3.5), bleu_material);

    let front_wall: Object=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,0.0,1.0))*rt::rotation(&UVec3f::new_normalize(Vec3f::x()), f32::consts::FRAC_PI_2)*rt::scale(3.5,3.5,3.5), 
    mirror_material);

    let back_wall: Object=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,0.0,-3.5))*rt::rotation(&UVec3f::new_normalize(Vec3f::x()), f32::consts::FRAC_PI_2)*rt::scale(3.5,3.5,3.5), 
    mirror_material);

    let left_wall: Object=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(1.0,0.0,0.0))*rt::rotation(&UVec3f::new_normalize(Vec3f::z()), f32::consts::FRAC_PI_2)*rt::scale(3.5,3.5,3.5), 
        mirror_material
    );

    let right_wall: Object=  rt::Object::new(plane_handle, rt::translate(&Vec3f::new(-1.0,0.0,0.0))*rt::rotation(&UVec3f::new_normalize(Vec3f::z()), f32::consts::FRAC_PI_2)*rt::scale(3.5,3.5,3.5), 
        mirror_material
    );

    let lucy: Object=  rt::Object::new(lucy, rt::translate(&Vec3f::new(0.0,-0.2,0.0))*rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::FRAC_PI_2)*rt::scale(0.8,0.8,0.8), 
        white_material
    );
    let lucy_flame: Object=  rt::Object::new(lucy_flame, rt::translate(&Vec3f::new(0.0,-0.2,0.0))*rt::rotation(&UVec3f::new_normalize(Vec3f::y()), f32::consts::FRAC_PI_2)*rt::scale(0.8,0.8,0.8), 
        glowing_material
    );
    
    scene.add_object(floor);
    scene.add_object(ceiling);
    scene.add_object(left_wall);
    scene.add_object(right_wall);
    scene.add_object(back_wall);
    scene.add_object(front_wall);
    scene.add_object(lucy);
    scene.add_object(lucy_flame);
    // scene.add_object(lucy_shell);

    println!("Saving the scene to: ./lucy.json ...");
    scene.save("./lucy.json").expect("Could not save scene");
}