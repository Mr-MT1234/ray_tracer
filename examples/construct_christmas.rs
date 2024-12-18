use rand::seq::SliceRandom;
use ray_tracer::{self as rt, Mat4f};

use core::f32;
use std::time::Instant;
use rt::{Vec3f, Vec2f, Object, UVec3f};

fn main() {
    let camera = rt::Camera::new(Vec3f::new(20.0,5.0,1.8), -Vec3f::x() - Vec3f::y() / 5.0, Vec3f::y(), 20.0 / 180. * f32::consts::PI);

    let mut scene = rt::Scene::new(camera, Box::new(rt::SkyEnvironment {
        sun_color: Vec3f::new(0.1,0.1,0.1),
        sun_direction: Vec3f::new(-1.0, -1.0, 1.0),
        sun_size: 0.1,
        up_color: Vec3f::new(0.0,0.0,0.0),
        down_color: Vec3f::new(0.0,0.0,0.0),
    }));


    let background_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(0.3,0.5,1.0), emission: Vec3f::zeros()}));
    let base_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(139.,90.,43.)/255., emission: Vec3f::zeros()}));
    let big_star_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::zeros(), emission: Vec3f::new(100.0, 100.0, 50.0)}));
    let inner_shell_material = scene.add_material(Box::new(rt::Dialectric {refraction_index: 1.33/1.5}));
    let leafs_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(18., 85., 43.)/255., emission: Vec3f::zeros()}));
    let outer_shell_material = scene.add_material(Box::new(rt::Dialectric {refraction_index: 1.5}));
    let spheres_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::zeros(), emission: Vec3f::new(5.0, 2.0, 2.0)}));
    let spirale_material = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(220.0, 150.0, 20.0)/255.0, roughness:0.2}));
    let stars_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::zeros(), emission: Vec3f::new(5.0, 5.0, 2.0)}));
    let sugar_canes_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(196., 35., 62.)/255., emission: Vec3f::zeros()}));
    let support_material = scene.add_material(Box::new(rt::Metal{color: Vec3f::new(220.0, 150.0, 20.0)/255.0, roughness:0.2}));
    let trunk_material = scene.add_material(Box::new(rt::Lambertian {color: Vec3f::new(139.,90.,43.)/255., emission: Vec3f::zeros()}));
    
    let background = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/background.obj").unwrap());
    let base = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/base.obj").unwrap());
    let big_star = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/Big_star.obj").unwrap());
    let inner_shell = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/inner_shell.obj").unwrap());
    let leafs = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/leafs.obj").unwrap());
    let outer_shell = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/outer_shell.obj").unwrap());
    let spheres = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/spheres.obj").unwrap());
    let spirale = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/spirale.obj").unwrap());
    let stars = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/stars.obj").unwrap());
    let sugar_canes = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/sugar_canes.obj").unwrap());
    let support = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/support.obj").unwrap());
    let trunk = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/tree/trunk.obj").unwrap());

    let background =  rt::Object::new(background, Mat4f::identity(), background_material);
    let base =  rt::Object::new(base, Mat4f::identity(), base_material);
    let big_star =  rt::Object::new(big_star, Mat4f::identity(), big_star_material);
    let inner_shell =  rt::Object::new(inner_shell, Mat4f::identity(), inner_shell_material);
    let leafs =  rt::Object::new(leafs, Mat4f::identity(), leafs_material);
    let outer_shell =  rt::Object::new(outer_shell, Mat4f::identity(), outer_shell_material);
    let spheres =  rt::Object::new(spheres, Mat4f::identity(), spheres_material);
    let spirale =  rt::Object::new(spirale, Mat4f::identity(), spirale_material);
    let stars =  rt::Object::new(stars, Mat4f::identity(), stars_material);
    let sugar_canes =  rt::Object::new(sugar_canes, Mat4f::identity(), sugar_canes_material);
    let support =  rt::Object::new(support, Mat4f::identity(), support_material);
    let trunk =  rt::Object::new(trunk, Mat4f::identity(), trunk_material);

    scene.add_object(background);
    scene.add_object(base);
    scene.add_object(big_star);
    scene.add_object(inner_shell);
    scene.add_object(leafs);
    scene.add_object(outer_shell);
    scene.add_object(spheres);
    scene.add_object(spirale);
    scene.add_object(stars);
    scene.add_object(sugar_canes);
    scene.add_object(support);
    scene.add_object(trunk);


    println!("Saving the scene to: ./christmas.json ...");
    scene.save("./christmas.json").expect("Could not save scene");
}