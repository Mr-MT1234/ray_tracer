use crate::math::*;
use crate::Ray;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: Vec3f,
    max: Vec3f
}

impl AABB {
    pub fn zeros() -> AABB {
        AABB {
            min: Vec3f::zeros(),
            max: Vec3f::zeros(),
        }
    }
    pub fn new(c1: Vec3f, c2: Vec3f) -> AABB {
        AABB {
            min: c1.zip_map(&c2, |a,b| a.min(b)),
            max: c1.zip_map(&c2, |a,b| a.max(b)),
        }
    }

    pub fn intersects(&self, ray: &Ray, min_t: f32, max_t: f32) -> bool {

        let inv_dir = ray.direction.map(|a| 1.0/a);
        let t0s = mul_element_wise(self.min - ray.origin,inv_dir);
        let t1s = mul_element_wise(self.max - ray.origin,inv_dir);

        let tsmaller = t0s.zip_map(&t1s, |a,b| a.min(b));
        let tbigger  = t0s.zip_map(&t1s, |a,b| a.max(b));

        let tmin = tsmaller.max().max(min_t);
        let tmax = tbigger.min().min(max_t);

        tmin < tmax
    }

    pub fn expand(&mut self, point: &Vec3f) {
        self.min = self.min.zip_map(point, |a,b| a.min(b));
        self.max = self.max.zip_map(point, |a,b| a.max(b));
    }

    pub fn get_min(&self) -> &Vec3f {
        &self.min
    }

    pub fn get_max(&self) -> &Vec3f {
        &self.max
    }
}

#[derive(Debug, Clone, Copy)]
enum SubNode {
    Children((usize, usize)),
    Triangles((usize, usize)),
}

#[derive(Debug, Clone, Copy)]
struct BVHNode {
    aabb: AABB,
    sub_node: SubNode
}

#[derive(Debug, Clone)]
pub struct BVH {
    nodes: Vec<BVHNode>,
    root: usize,
}

impl BVH {
    pub fn new(vertices: &[Vec3f], triangles: &mut [[usize;3]]) -> BVH {
        let mut root = AABB::new(Vec3f::new(-1e-3, -1e-3, -1e-3), Vec3f::new(1e-3, 1e-3, 1e-3));
        for point in vertices {
            root.expand(point);
        }

        let root_node = BVHNode {
            aabb: root,
            sub_node: SubNode::Triangles((0, triangles.len()))
        };

        BVH {
            nodes: vec![root_node],
            root: 0
        }
    }

    pub fn intersects(&self, ray: &Ray, min_t: f32, max_t: f32) -> (usize,usize) {
        if self.nodes[self.root].aabb.intersects(ray, min_t, max_t) {
            match self.nodes[self.root].sub_node {
                SubNode::Children(_) => todo!(),
                SubNode::Triangles(triangles) => triangles
            }
        }
        else {
            (0,0)
        }

    }
}