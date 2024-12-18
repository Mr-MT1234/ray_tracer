# Ray tracer
A ray tracer for me to learn more about rendering and rust.

![Cornell box scene rendered with 1000 samples per pixel.](cornell_box_1000.png)
Cornell box scene rendered with 1000 samples per pixel. ($\sim$ 15min 17s)
This project does not aim to be at the cutting edge of its domain. However, it utilizes some techniques of optimisation for faster rendre times.

## Creating Scene

Currently, scene are stored in a Json file. The only way to generate a scene file is through code.

``` rust
let camera = rt::Camera::new(Vec3f::new(0.0,0.0,-3.0), Vec3f::z(), Vec3f::y(), 45.0 / 180. * f32::consts::PI);    

let mut scene = rt::Scene::new(camera, Box::new(rt::ConstantEnvironment {color: Vec3f::new(0.1,0.1,0.1)}) );

let plane = rt::Mesh::new(
    &[
        rt::Vertex { position: Vec3f::new( 1.0,0.0,-1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
        rt::Vertex { position: Vec3f::new( 1.0,0.0, 1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
        rt::Vertex { position: Vec3f::new(-1.0,0.0, 1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
        rt::Vertex { position: Vec3f::new(-1.0,0.0,-1.0), normal: -Vec3f::y(), uv_coord: Vec2f::zeros() },
    ],
    &[[0,1,2], [2,3,0]]
);

let white_material = scene.add_material(
    Box::new(rt::Lambertian {color: Vec3f::new(1.0,1.0,1.0), emission: Vec3f::zeros()})
);
let glass_material = scene.add_material(Box::new(rt::Dialectric{refraction_index: 1.5}));

let plane_handle = scene.add_mesh(plane);
let bunny_handle = scene.add_mesh(rt::Mesh::load_obj("./examples/assets/bunny.obj").unwrap());

let floor = rt::Object::new(plane_handle, rt::translate(&Vec3f::new(0.0,-1.0,0.0)), 
            white_material);

let bunny: Object=  rt::Object::new(bunny_handle, rt::translate(&Vec3f::new(0.5,-0.85,-0.2))
*rt::rotation(&UVec3f::new_normalize(Vec3f::y()), -3.0*f32::consts::PI/4.0)
*rt::scale(0.15, 0.15, 0.15),
glass_material);

scene.add_object(floor);
scene.add_object(bunny);

scene.save("./scene.json").expect("Could not save scene");
```

Some examples for scene generation are provided in the examples directory, they follow the naming convention ```construct_{name of the scene}.rs```. These examples can be ran through the command ```cargo run --release --example name_of_the_file```.

## Rendering a scene

A scene can be rendered through the command: ```cargo run --release -p render_scene path/to/scene```

It's also possible to render a scene through code, for more controle over the parameters of the rendering:

```rust    
const WIDTH: u32 = 700;
const HEIGHT: u32 = 700;

let scene = rt::Scene::load("path/to/scene");
let renderer = rt::RayTracer;

let mut target = rt::Image::new(Vec3f::zeros(), WIDTH, HEIGHT);

let options = rt::RenderOptions { max_depth: 10,  rays_per_pixel: 100};

renderer.render_with_print(&scene, &mut target, &options);

target.save("./render.png").expect("Failed to save render result");
```

## Examples

Some examples provided in this reposotory might no longer work as they were created for an older version of the engin.


