use ray_tracer as rt;

use std::time::Instant;
use rt::Vec3f;

fn main() {

    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("No scene provided");
        return;
    }
    
    const WIDTH: u32 = 1500;
    const HEIGHT: u32 = 1500;
    
    let scene = rt::Scene::load(&args[1]).expect("Counln not load scene");
    let renderer = rt::RayTracer;
    
    let mut target = rt::Image::new(Vec3f::zeros(), WIDTH, HEIGHT);

    let options = rt::RenderOptions { max_depth: 10,  rays_per_pixel: 500};

    let start = Instant::now();
    renderer.render_with_print_single_thread(&scene, &mut target, &options);
    println!("\nRendering finished after {:.2}s", start.elapsed().as_secs_f32());

    target.save("./cornell_parallel_10.png").expect("Failed to save render result");
}
