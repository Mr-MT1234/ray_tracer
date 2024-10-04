use core::f32;
use std::default;

use eframe::egui;

use egui_plot::{BarChart, Plot, PlotBounds};
use ray_tracer as rt;
use rt::{Vec3f, UVec3f, Optical};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui example: custom font",
        options,
        Box::new(|_| Ok(Box::new(MyApp::new()))),
    )
}

struct Histogram{
    buckets_bounds: Vec<u32>,
    buckets: Vec<u32>,
}

impl Histogram {
    fn new(bounds: impl Into<Vec<u32>>) -> Self {
        let buckets_bounds = bounds.into();
        let buckets = (0..buckets_bounds.len() + 1).map(|_| 0).collect();

        Self {
            buckets_bounds,
            buckets
        }
    }

    fn add_bound(&mut self, bound: u32) {
        for (i,&b) in self.buckets_bounds.iter().enumerate() {
            if b > bound {
                self.buckets_bounds.insert(i, b);
                break;
            }
        }
    }

    fn add_point(&mut self, data: u32) {
        let elongated_bounds = self.buckets_bounds.iter().chain(&[u32::MAX]);
        for (i, &bound) in elongated_bounds.enumerate() {
            if data <= bound {
                self.buckets[i] += 1;
                break;
            }
        }
    }

    fn clear(&mut self) {
        self.buckets.fill(0);
    }

    fn get_plot(&self) -> egui_plot::BarChart{
        let bar_count = self.buckets.len();
        let data_count: f64 = self.buckets.iter().map(|i| *i as f64).sum();

        if bar_count == 1 { return BarChart::new(vec![])}

        let bars = self.buckets.iter().enumerate().map(|(i, &bucket)| {
            let name =if i == 0 {
                format!("]-inf, {}]", self.buckets_bounds[0])
            }
            else if i == self.buckets_bounds.len() {
                format!("]{}, +inf[", self.buckets_bounds.last().unwrap())
            }
            else {
                format!("]{}, {}]", self.buckets_bounds[i-1], self.buckets_bounds[i])
            };
            
            egui_plot::Bar {
                argument: i as f64,
                name: name,
                value: bucket as f64 / data_count,
                bar_width: 0.5,
                orientation: egui_plot::Orientation::Vertical,
                base_offset: None,
                stroke: egui::Stroke::new(2.0, egui::Color32::BLUE),
                fill: egui::Color32::BLUE
            }
        }).collect();

        let bar_chart = egui_plot::BarChart::new(bars);

        bar_chart
    }
}

struct MyApp {
    image_data: Vec<u8>,
    width: u16, 
    height: u16,

    max_aabb: f32,
    max_triangles: f32,
    rotation: f32,
    texture: Option<egui::TextureHandle>,
    object: rt::Object,

    aabb_histogram: Histogram,
    triangles_histogram: Histogram
}

impl MyApp {
    fn new() -> Self {
        const WIDTH: u16 = 700;
        const HEIGHT: u16 = 400;
        Self {
            image_data: (0..WIDTH as usize * HEIGHT as usize * 4).map(|_| 255u8).collect(),
            width: WIDTH,
            height: HEIGHT,
            max_aabb: 300.0,
            max_triangles: 100.0,
            texture: None,
            object: rt::Object::new(
                rt::Mesh::load_obj("./examples/assets/bunny.obj").unwrap(),
                rt::Mat4f::identity(),
                Box::new(rt::Lambertian {color: Vec3f::zeros(), emission: Vec3f::zeros()})
            ),
            rotation: 0.0,
            aabb_histogram: Histogram::new((0..=300).collect::<Vec<_>>()),
            triangles_histogram: Histogram::new((0..=100).collect::<Vec<_>>()),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // println!("BVH constructed");
        // println!("\tDepth               : {}", model.mesh.bvh.depth());
        // println!("\tMax Triangle Count  : {}", model.mesh.bvh.max_triangle_count());
        // println!("\tAvg Triangle Count  : {}", model.mesh.bvh.avg_triangle_count());        
        
        egui::SidePanel::left("Control panel").show(ctx, |ui| {
            let aabb_slider = ui.add(egui::widgets::Slider::new(&mut self.max_aabb, 0.0..=300.0).text("Max AABB tests"));
            let triangle_slider = ui.add(egui::widgets::Slider::new(&mut self.max_triangles, 0.0..=100.0).text("Max triangle tests"));
            let rotation_slider = ui.add(egui::widgets::Slider::new(&mut self.rotation, 0.0..=2.0*f32::consts::PI).text("Rotation"));

            if aabb_slider.changed() || triangle_slider.changed() || rotation_slider.changed() {
                self.update_render_image();
                let image = egui::ColorImage::from_rgba_unmultiplied(
                    [self.width as usize, self.height as usize],
                    &self.image_data.as_slice(),
                );
                self.texture = Some(ctx.load_texture("render", image, egui::TextureOptions::default()));
            }

            let aabb_plot = self.aabb_histogram.get_plot();
            let triangles_plot = self.triangles_histogram.get_plot();

            Plot::new("id_source").view_aspect(2.0).x_axis_label("Number of AABB tests"    ).show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [300.0, 0.1]));
                plot_ui.bar_chart(aabb_plot)
            });
            Plot::new("id_source").view_aspect(2.0).x_axis_label("Number of triangle tests").show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [100.0, 0.1]));
                plot_ui.bar_chart(triangles_plot)
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("Plot of sin");
            // let sin: PlotPoints = (0..1000).map(|i| {
            //     let x = i as f64 * 0.01;
            //     [x, x.sin()]
            // }).collect();
            // let line = Line::new(sin);
            // Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
            ui.heading("Heat map");
            if let Some(texture) = &self.texture {
                ui.image(texture);
            }
        });

    }
}
impl MyApp {
    fn update_render_image(&mut self) {
        let camera = rt::Camera::new(Vec3f::new(0.0,1.0,-8.0), Vec3f::z(), Vec3f::y(), 40.0 / 180. * f32::consts::PI);
        self.aabb_histogram.clear();
        self.triangles_histogram.clear();
        self.object.set_transform(rt::rotation(&UVec3f::new_normalize(Vec3f::y()), self.rotation));

        for (ray, pixel) in camera.shoot_at((self.width, self.height), 1) {
            let (hit , report) = self.object.hit(&ray, 0.0, f32::INFINITY);
            
            let color = get_color(&report, self.max_aabb, self.max_triangles);
            set_pixel(pixel, color, &mut self.image_data, self.width as usize);
            if let Some(_) = hit {
                self.aabb_histogram.add_point(report.aabb_tests as u32);
                self.triangles_histogram.add_point(report.triangle_tests as u32);
            }
        
        }
    }
}

fn set_pixel([i,j]: [usize;2], value: rt::Vec3f, data: &mut [u8], width: usize) {
    data[4*(i*width + j) + 0] = (value.x*255.0) as u8;
    data[4*(i*width + j) + 1] = (value.y*255.0) as u8;
    data[4*(i*width + j) + 2] = (value.z*255.0) as u8;
}

fn get_color(report : &rt::CollisionReport, max_aabb: f32, max_triangles: f32) -> Vec3f {

    let r = report.aabb_tests as f32 / max_aabb;
    let b = report.triangle_tests as f32/ max_triangles;

    if (r > 1.0) || (b > 1.0){
        Vec3f::new(1.0,1.0,1.0)
    }
    else {
        Vec3f::new(r, 0.0, b)
    }
}
