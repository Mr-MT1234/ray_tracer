use crate::math::*;
use crate::Ray;
use crate::bvhs::BVH;
use crate::Vertex;
use std::ffi::OsStr;
use std::path::Path;

pub struct CollisionInfo {
    pub point : Vec3f,
    pub normal: Vec3f,
    pub t: f32,
    pub inside: bool,
    pub uv: Vec2f
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CollisionReport {
    pub triangle_tests: u64,
    pub aabb_tests: u64,
}


pub trait Collider {
    fn collide(&self, ray: &Ray, min_t: f32, max_t: f32) -> (Option<CollisionInfo>, CollisionReport);
}

pub struct Triangle {
    pub origin: Vec3f,
    pub side1 : Vec3f,
    pub side2 : Vec3f,
}

impl Collider for Triangle {
    fn collide(&self, ray: &Ray, min_t: f32, max_t: f32) -> (Option<CollisionInfo>, CollisionReport) {
        // based on https://stackoverflow.com/questions/42740765/intersection-between-line-and-triangle-in-3d

        let origin = ray.origin - self.origin;
        let direction = &ray.direction;
        let n = self.side1.cross(&self.side2);
        let det = direction.dot(&n);

        if det.abs() < 1e-4 {
            return (None, CollisionReport {aabb_tests:0, triangle_tests: 1});
        }
        
        // v =  (E1,O-A,D)  / (D,E1,E2)
        // u = -(E2,O-A,D)  / (D,E1,E2)
        // t = -(O-A,E1,E2) / (D,E1,E2)
        //where (A,B,C) is the determinant of the matrix with columns A,B and C.

        let od = origin.cross(&direction);
        let invdet = 1.0/det;
        let t = -origin.dot(&n) * invdet;
        let u = -self.side2.dot(&od) * invdet;
        let v = self.side1.dot(&od) * invdet;
        

        if t > min_t && t < max_t && u >= 0.0 && v >= 0.0
        && u+v <= 1.0 {
            let inside = n.dot(&direction) > 0.0;
            let normal = if inside {-n.normalize()} else {n.normalize()}; 

            (
                Some(
                    CollisionInfo {
                        point: ray.direction*t + ray.origin,
                        normal,
                        t,
                        inside,
                        uv: Vec2f::new(u,v)
                    }
                ), 
                CollisionReport::default()
            )
        }
        else {
            (None, CollisionReport::default())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<[usize;3]>,
    pub bvh: BVH,
}

impl Mesh {
    pub fn new(vertices: impl Into<Vec<Vertex>>, triangles: impl Into<Vec<[usize;3]>>) -> Mesh{
        let vertices: Vec<_> = vertices.into();
        let mut triangles: Vec<_> = triangles.into();

        let bvh = BVH::build(&vertices, &mut triangles);
        
        Mesh {
            vertices,
            triangles,
            bvh
        }
    }

    pub fn load_obj(path: &(impl AsRef<OsStr> + ?Sized)) -> Result<Mesh,tobj::LoadError> {
        let path = Path::new(path);
        let params = tobj::LoadOptions {
            single_index: true,
            ..tobj::LoadOptions::default()
        };
        let (models, _) = tobj::load_obj(path, &params)?;
        if models.len() != 1 {
            Err(tobj::LoadError::InvalidLoadOptionConfig)
        }
        else {
            let tobj::Mesh {positions, indices,texcoords, normals,..} = &models[0].mesh;
            assert!(positions.len() % 3 == 0, "Position array's length is a not multiple of 3");
            assert!(indices  .len() % 3 == 0, "Position array's length is a not multiple of 3");
            assert!(texcoords.len() % 2 == 0, "Position array's length is a not multiple of 2");
            
            let positions_iter = (0..positions.len() / 3).map(|i| (3*i,3*i+1,3*i+2))
                                                            .map(|(i,j,k)| (positions[i], positions[j], positions[k]))
                                                            .map(|(x,y,z)| Vec3f::new(x,y,z));

            let uvs_iter = (0..texcoords.len() / 2).map(|i| (2*i,2*i+1))
                                                       .map(|(i,j)| (texcoords[i], texcoords[j]))
                                                       .map(|(u,v)| Vec2f::new(u,v));
            let normals_iter = (0..normals.len() / 3).map(|i| (3*i,3*i+1,3*i+2))
                                                       .map(|(i,j,k)| (normals[i], normals[j], normals[k]))
                                                       .map(|(x,y,z)| Vec3f::new(x,y,z));

            let vertices: Vec<_> = positions_iter.zip(normals_iter).zip(uvs_iter)
                                    .map(|((position, normal), uv_coord)| Vertex {position, normal, uv_coord})
                                    .collect();

            let mut triangles : Vec<_> = (0..indices.len() / 3).map(|i| (3*i,3*i+1,3*i+2))
                                                       .map(|(i,j,k)| [indices[i] as usize, indices[j]as usize, indices[k]as usize])
                                                       .collect();
            
            let bvh = BVH::build(&vertices, &mut triangles);
            
            Ok(Mesh {
                vertices,
                triangles,
                bvh
            })
        }
    }
}

impl Collider for Mesh {
    fn collide(&self, ray: &Ray, min_t: f32, mut max_t: f32) -> (Option<CollisionInfo>, CollisionReport) {
        let mut closest_dist = max_t;
        let mut closest_hit = None;

        let mut triangle_iter = self.bvh.intersects(ray, min_t, max_t);
        let mut report = CollisionReport::default();

        let (mut begin, mut end, mut aabb_tests) = triangle_iter.next(min_t, max_t);
        report.aabb_tests += aabb_tests;

        while (begin,end) != (0,0){

            for &[i,j,k] in &self.triangles[begin..end] {
                let triangle = Triangle {
                    origin: self.vertices[i].position,
                    side1: self.vertices[j].position - self.vertices[i].position,
                    side2: self.vertices[k].position - self.vertices[i].position,
                };

                if let (Some(collision), _) = triangle.collide(ray, min_t, max_t) {
                    if collision.t < closest_dist && collision.t > min_t {
                        closest_dist = collision.t;
                        let [u,v]:[f32;2] = collision.uv.into();
                        let w = 1.0 - u - v;

                        let uv = w*self.vertices[i].uv_coord + u*self.vertices[j].uv_coord + v*self.vertices[k].uv_coord;
                        let mut normal = (w*self.vertices[i].normal + u*self.vertices[j].normal + v*self.vertices[k].normal).normalize();
                        if collision.inside {normal = -normal;}
                        closest_hit = Some(CollisionInfo {
                            point: collision.point,
                            t: collision.t,
                            inside: collision.inside,
                            normal,
                            uv: collision.uv
                        });

                        max_t = collision.t;
                    }
                }
            }

            (begin,end, aabb_tests) = triangle_iter.next(min_t, max_t);
            report.aabb_tests += aabb_tests;
            report.triangle_tests += (end - begin) as u64;
        }

        (closest_hit, report)
    
    }

}