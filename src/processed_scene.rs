use crate::{math::*, Material, Scene, BVH::BVH};

pub struct Vertex {
    pub position: Vec3f,
    pub normal: Vec3f,
    pub uv_coord: Vec2f,
}

pub struct ProssecedTriangle {
    a: Vertex,
    b: Vertex,
    c: Vertex,
    material_index: usize,  
}

pub struct ProssecedScene {
    triangles: Vec<ProssecedTriangle>,
    materials: Vec<Box<dyn Material>>,
    bvh: BVH
}

impl ProssecedScene {
    fn process(scene: Scene) -> ProssecedScene {
        
    }
} 