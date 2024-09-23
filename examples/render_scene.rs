use show_image::WindowOptions;
use show_image::{ImageView, ImageInfo, create_window};
use ray_tracer as rt;

use std::f32;
use rt::{Vec3f, Vec2f, Mat4f, Object, UVec3f};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: u16 = 1500;
    const HEIGHT: u16 = 900;
    
    // Create a window with default options and display the image.
    let window = create_window("image", WindowOptions {
        size: Some([WIDTH as u32,HEIGHT as u32]),
        ..Default::default()
    })?;
    let mut data: Vec<_> = (0..WIDTH as usize * HEIGHT as usize * 3).map(|_| 0u8).collect();
    
    let camera = rt::Camera::new(Vec3f::new(0.0,0.0,-10.0), Vec3f::z(), Vec3f::y(), 60.0 / 180. * f32::consts::PI);
    let renderer = rt::RayTracer::new();
    
    
    let mut scene = rt::Scene::new();
    let cube_mesh = ray_tracer::Mesh::load_obj("./examples/assets/cube.obj").unwrap();
    
    let cube_light = Object::new(
        cube_mesh,
        rt::translate(&Vec3f::new(0.0,3.0,0.0))*rt::scale(0.1, 0.1, 0.1),
        Box::new(rt::Lambertian{color:Vec3f::new(0.0,0.0,0.0), emission: Vec3f::new(100.0,100.,100.0)}),
    );

    let bunny = Object::new(
        rt::Mesh::load_obj("./examples/assets/bunny.obj").unwrap(),
        rt::translate(&Vec3f::new(2.0,-0.5,0.0))*
        rt::rotation(&UVec3f::new_normalize(Vec3f::y()), std::f32::consts::PI*16.0/12.0),
        // Box::new(rt::Lambertian{color:Vec3f::new(0.2,0.4,0.7), emission: Vec3f::new(0.0,0.,0.0)}),
        Box::new(rt::Dialectric{refraction_index:1.5}),
    );

    let monkey = Object::new(
        rt::Mesh::load_obj("./examples/assets/monkey.obj").unwrap(),
        rt::translate(&Vec3f::new(1.0,0.0,3.0))*
        rt::rotation(&UVec3f::new_normalize(Vec3f::y()), std::f32::consts::PI*4.0/4.0),
        Box::new(rt::Lambertian{color:Vec3f::new(0.2,0.4,0.7), emission: Vec3f::new(0.0,0.,0.0)}),
    );


    
    let ground = Object::new(
        rt::Mesh::new(
        &[
            rt::Vertex { position: Vec3f::new( 1000.0,-1.0,-1000.0), normal: Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new( 1000.0,-1.0, 1000.0), normal: Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new(-1000.0,-1.0, 1000.0), normal: Vec3f::y(), uv_coord: Vec2f::zeros() },
            rt::Vertex { position: Vec3f::new(-1000.0,-1.0,-1000.0), normal: Vec3f::y(), uv_coord: Vec2f::zeros() },
            ],
            &[[0,1,2], [2,3,0]]
        ),
        rt::Mat4f::identity(),
        Box::new(rt::Lambertian{color:Vec3f::new(0.8,0.8,0.8), emission:Vec3f::zeros()}),
    );
    
    scene.add_object(monkey);
    scene.add_object(bunny);
    // scene.add_object(cube_light);
    scene.add_object(ground);
    
    let mut r_image = rt::RenderTarget::new(WIDTH, HEIGHT);
    
    loop {
        let report = renderer.accumulate(&scene, &camera, &mut r_image);
        copy_result(&r_image, &mut data);
        let window_image = ImageView::new(ImageInfo::rgb8(WIDTH as u32, HEIGHT as u32), &data);
        dbg!(report);
        
        window.set_image("image-001", window_image)?;
    }

}

fn copy_result(source: &rt::RenderTarget, destination: &mut [u8]) {
    let (width, height) = source.get_size();
    for i in 0..height as usize {
        for j in 0..width as usize {
            let color = source.get_result([i,j]);
            destination[3*(i*width as usize + j) + 0] = (color.x * 255.0) as u8;
            destination[3*(i*width as usize + j) + 1] = (color.y * 255.0) as u8;
            destination[3*(i*width as usize + j) + 2] = (color.z * 255.0) as u8;
        }
    }
}

// struct CameraMan {
//     pub camera: rt::Camera,
//     speed: f32,
//     angular_speed: f32,
// }
// impl CameraMan {
    //     fn new(camera: rt::Camera) -> CameraMan{
        //         CameraMan {camera, speed: 2.0, angular_speed: 1.0}
        //     }
        
        //     fn update(&mut self, dt: f32) -> bool{
//         use macroquad::input::*;

//         let direction = &mut self.camera.direction; 
//         let right = direction.cross(&self.camera.up);
//         let up = right.cross(direction);

//         let mut did_move = false;

//         if is_key_down(KeyCode::W) {
    //             self.camera.origin += *direction*dt*self.speed;
//             did_move = true;
//         }
//         if is_key_down(KeyCode::S) {
    //             self.camera.origin -= *direction*dt*self.speed;
    //             did_move = true;
//         }

//         if is_key_down(KeyCode::D) {
//             self.camera.origin += right*dt*self.speed;
//             did_move = true;
//         }
//         if is_key_down(KeyCode::A) {
    //             self.camera.origin -= right*dt*self.speed;
    //             did_move = true;
    //         }

//         if is_key_down(KeyCode::LeftShift) {
//             self.camera.origin += up*dt*self.speed;
//             did_move = true;
//         }
//         if is_key_down(KeyCode::LeftControl) {
//             self.camera.origin -= up*dt*self.speed;
//             did_move = true;
//         }

//         if is_key_down(KeyCode::Left) {
//             let axis = rt::UVec3f::new_normalize(Vec3f::y());
//             let rot = rt::na::Rotation::from_axis_angle(&axis, self.angular_speed*dt);
//             *direction = rot**direction;
//             did_move = true;
//         }
//         if is_key_down(KeyCode::Right) {
//             let axis = rt::UVec3f::new_normalize(Vec3f::y());
//             let rot = rt::na::Rotation::from_axis_angle(&axis, -self.angular_speed*dt);
//             *direction = rot**direction;
//             did_move = true;
//         }

//         if is_key_down(KeyCode::Up) {
//             let axis = rt::UVec3f::new_normalize(right);
//             let rot = rt::na::Rotation::from_axis_angle(&axis, self.angular_speed*dt);
//             *direction = rot**direction;
//             did_move = true;
//         }
//         if is_key_down(KeyCode::Down) {
//             let axis = rt::UVec3f::new_normalize(right);
//             let rot = rt::na::Rotation::from_axis_angle(&axis, -self.angular_speed*dt);
//             *direction = rot**direction;
//             did_move = true;
//         }

//         did_move
//     }
// }
