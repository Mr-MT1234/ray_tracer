use core::f32;
use std::usize;

use crate::{math::*, commun_types::*};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec3f,
    pub max: Vec3f
}

impl AABB {
    pub const EMPTY: AABB = AABB {
        max: Vec3f::new(-f32::INFINITY,-f32::INFINITY,-f32::INFINITY),
        min: Vec3f::new(f32::INFINITY,f32::INFINITY,f32::INFINITY),
    };
    pub const UNIVERSE: AABB = AABB {
        max: Vec3f::new(f32::INFINITY,f32::INFINITY,f32::INFINITY),
        min: Vec3f::new(-f32::INFINITY,-f32::INFINITY,-f32::INFINITY),
    };

    pub fn new(c1: Vec3f, c2: Vec3f) -> AABB {
        AABB {
            min: c1.zip_map(&c2, |a,b| a.min(b)),
            max: c1.zip_map(&c2, |a,b| a.max(b)),
        }
    }

    pub fn union(left: &AABB, right: &AABB) -> AABB {
        return AABB {
            min:  left.min.zip_map(&right.min, |a,b| a.min(b)),
            max:  left.max.zip_map(&right.max, |a,b| a.max(b)),
        }
    }

    pub fn union_many(aabbs: &[AABB]) -> AABB {
        let mut union = AABB::EMPTY;
        for aabb in aabbs {
            union = AABB::union(&union, aabb);
        }

        union
    }

    pub fn enclose(vertices: &[Vertex], triangles: &[ [usize;3] ]) -> AABB {
        let mut aabb = AABB::EMPTY;
        for triangle_vertices in triangles {
            for v in triangle_vertices {
                aabb.expand(&vertices[*v].position);
            }
        }
        aabb
    }

    pub fn intersects(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<f32> {

        let inv_dir = ray.direction.map(|a| 1.0/a);
        let t0s = mul_element_wise(self.min - ray.origin,inv_dir);
        let t1s = mul_element_wise(self.max - ray.origin,inv_dir);

        let tsmaller = t0s.zip_map(&t1s, |a,b| a.min(b));
        let tbigger  = t0s.zip_map(&t1s, |a,b| a.max(b));

        let tmin = tsmaller.max().max(min_t);
        let tmax = tbigger.min().min(max_t);

        if tmin < tmax  { Some(tmin) } 
        else            { None }
    }

    pub fn pad(mut self) -> Self {
        self.max += Vec3f::new(f32::EPSILON,f32::EPSILON,f32::EPSILON);
        self.min -= Vec3f::new(f32::EPSILON,f32::EPSILON,f32::EPSILON);
        self
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
pub enum NodeContent {
    Children((usize, usize)),
    Triangles((usize, usize)),
}

#[derive(Debug, Clone, Copy)]
pub struct BVHNode {
    pub aabb: AABB,
    pub content: NodeContent,
}

#[derive(Debug, Clone)]
pub struct BVH {
    nodes: Vec<BVHNode>,
}

impl BVH {
    const MAX_DEPTH: usize = 32;

    fn bin_centroids<const BIN_COUNT: usize>(vertices: &[Vertex], centroids: &[Vec3f], triangles: &[[usize;3]], dimension: usize) 
                            -> ([AABB; BIN_COUNT], [usize; BIN_COUNT], f32, f32) {

        let mut bins = [AABB::EMPTY;BIN_COUNT];
        let mut bin_sizes = [0;BIN_COUNT];

        let mut centroid_aabb = AABB::EMPTY;
        for point in centroids {
            centroid_aabb.expand(&point);
        }
        
        let lenght = centroid_aabb.max[dimension] - centroid_aabb.min[dimension];
        let step = lenght / BIN_COUNT as f32;

        for (centroid, triangle) in centroids.iter().zip(triangles) {
            let bin_index = ((centroid[dimension] - centroid_aabb.min[dimension]) / step * (1.0-f32::EPSILON)) as usize;
            bin_sizes[bin_index] += 1;
            for vertex_index in triangle {
                bins[bin_index].expand(&vertices[*vertex_index].position);
            }
        }

        (bins, bin_sizes, step, centroid_aabb.min[dimension])
    }

    fn calc_sah(aabb: AABB, count: usize) -> f32 {
        let [x,y,z]: [f32;3] = (aabb.max - aabb.min).into();

        (count as f32 + f32::EPSILON) * (x*y + y*z + z*x)
    }

    fn find_best_separation(vertices: &[Vertex], triangles: &[[usize;3]], centroids: &[Vec3f]) -> (usize, f32) {
        const BIN_COUNT: usize = 100;
        
        let mut best_cost = f32::INFINITY;
        let mut best_dimension = 0;
        let mut best_plane = f32::INFINITY;
        
        for dimension in 0..3 {
            let (bins, counts, step, start) = Self::bin_centroids::<BIN_COUNT>(vertices, centroids, triangles, dimension);

            let mut left = AABB::EMPTY;
            let mut left_count = 0;

            for i in 0..BIN_COUNT {
                left = AABB::union(&left, &bins[i]);
                left_count += counts[i]; 
                let right = AABB::union_many(&bins[i+1..]);

                let sah_cost = Self::calc_sah(left, left_count) + Self::calc_sah(right, triangles.len() - left_count);
                
                if sah_cost < best_cost {
                    best_cost = sah_cost;
                    best_dimension = dimension;
                    best_plane = step*i as f32 + start;
                }
            }
        }


        (best_dimension, best_plane)
    }

    fn separate(dimension:usize, plane: f32, centroids: &mut [Vec3f], triangles: &mut [ [usize;3] ]) -> usize {
        let mut pivot = 0;

        for i in 0..centroids.len() {
            if centroids[i][dimension] < plane {                
                centroids.swap(i, pivot);
                triangles.swap(i, pivot);
                pivot += 1
            }
        }

        pivot
    }

    fn devide(nodes: &mut Vec<BVHNode>, node_index: usize, vertices: &[Vertex], triangles: &mut [[usize;3]], centroids: &mut [Vec3f]) {
        let node = nodes[node_index];

        match node.content {
            NodeContent::Children(_) => panic!("Attempt to devide an already devided node !!!"),
            NodeContent::Triangles((start,end)) => {
                let (dimension, plane) = Self::find_best_separation(vertices, &triangles[start..end], &centroids[start..end]);

                let left_count = Self::separate(dimension, plane, &mut centroids[start..end], &mut triangles[start..end]);

                if left_count == 0 || left_count == triangles.len() {
                    return;
                }
                
                let left_aabb = AABB::enclose(vertices, &triangles[start..start+left_count]).pad();
                let right_aabb = AABB::enclose(vertices, &triangles[start+left_count..end]).pad();

                let devided_sah = Self::calc_sah(left_aabb, left_count) + Self::calc_sah(right_aabb, end-start-left_count);
                let self_sah = Self::calc_sah(node.aabb, end-start);

                if devided_sah > self_sah {
                    return;
                }
                let left_index = nodes.len();
                let right_index = left_index+1;
                nodes.push(BVHNode{
                    aabb:left_aabb,
                    content: NodeContent::Triangles((start,start + left_count))
                });
                nodes.push(BVHNode{
                    aabb:right_aabb,
                    content: NodeContent::Triangles((start + left_count, end))
                });
                nodes[node_index].content = NodeContent::Children((left_index, right_index));

                Self::devide(nodes, left_index, vertices, triangles, centroids);
                Self::devide(nodes, right_index, vertices, triangles, centroids);
                
            
            }
        }
    }

    pub fn build(vertices: &[Vertex], triangles: &mut [[usize;3]]) -> BVH {
        
        let mut centroids: Vec<_> = triangles.iter()
                                            .map(|[i,j,k]| {
                                                let v1 = &vertices[*i].position;
                                                let v2 = &vertices[*j].position;
                                                let v3 = &vertices[*k].position;
                                                (v1 + v2 + v3) / 3.0
                                            }).collect();
        
        
        let mut root = AABB::EMPTY;
        for point in vertices {
            root.expand(&point.position);
        }

        let mut nodes = vec![BVHNode {
            aabb: root.pad(),
            content: NodeContent::Triangles((0, triangles.len()))
        }];

        Self::devide(&mut nodes, 0, vertices, triangles, &mut centroids);

        let bvh = BVH {
            nodes
        };

        bvh
    }

    pub fn intersects(&self, ray: &Ray) -> BVHIterator {
        BVHIterator {
            bvh: &self,
            stack: [0;BVH::MAX_DEPTH],
            ray: *ray,
            head: 0
        }

    }

    pub fn depth(&self) -> u32 {
        fn depth_of_node(nodes: &Vec<BVHNode>, index: usize) -> u32 {
            let node = &nodes[index];
            match node.content {
                NodeContent::Triangles(_) => 1,
                NodeContent::Children((left,right)) => 1 + depth_of_node(nodes, left).max(depth_of_node(nodes, right))
            }
        }

        depth_of_node(&self.nodes, 0)
    }

    pub fn max_triangle_count(&self) -> u32 {
        self.nodes.iter().map(|node| {
            match node.content {
                NodeContent::Children(_) => 0,
                NodeContent::Triangles((start, end)) => (end - start) as u32
            }
        }).max()
        .unwrap()
    }

    pub fn avg_triangle_count(&self) -> f32 {
        let triangle_count : f32 = self.nodes.iter().map(|node| {
            match node.content {
                NodeContent::Children(_) => 0.0,
                NodeContent::Triangles((start, end)) => (end - start) as f32
            }
        }).sum();

        let leaf_count : f32 = self.nodes.iter().map(|node| {
            match node.content {
                NodeContent::Children(_) => 0.0,
                NodeContent::Triangles(_) => 1.0
            }
        }).sum();

        triangle_count / leaf_count
    }

    pub fn get_nodes(&self) -> &Vec<BVHNode> {
        &self.nodes
    }
}

pub struct BVHIterator<'a> {
    stack: [usize; BVH::MAX_DEPTH],
    ray: Ray,
    head: isize,
    bvh: &'a BVH,
}

impl<'a> BVHIterator<'a> {
    pub fn next(&mut self, min_t: f32, max_t: f32) -> (usize, usize, u64) {
        // dbg!(self.head);
        let mut aabb_tests = 0;
        while self.head >= 0 {
            let node = &self.bvh.nodes[self.stack[self.head as usize]];
            self.head -= 1;
            aabb_tests += 1;
            if let Some(_) = node.aabb.intersects(&self.ray, min_t, max_t) {
                match node.content {
                    NodeContent::Children((left, right)) => {
                        self.stack[(self.head  + 1) as usize] = left;
                        self.stack[(self.head  + 2) as usize] = right;
                        self.head += 2;
                    }
                    NodeContent::Triangles((begin,end)) => {
                        return (begin,end, aabb_tests);
                    }
                }
            }
        };

        (0,0,aabb_tests)
    }
}