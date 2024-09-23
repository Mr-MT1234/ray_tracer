use crate::math::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin : Vec3f,
    pub direction : Vec3f,
}
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3f,
    pub normal: Vec3f,
    pub uv_coord: Vec2f,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RenderReport {
    pub aabb_tests: u64,
    pub triangle_tests: u64,
}
