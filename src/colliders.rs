use crate::math::*;
use crate::Ray;
use crate::BVH::BVH;
use std::path::Path;

pub struct CollisionInfo {
    pub point : Vec3f,
    pub normal: Vec3f,
    pub t: f32,
    pub inside: bool,
}

pub trait Collider {
    fn collide(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<CollisionInfo>;
}

pub struct Sphere {
    pub centre: Vec3f,
    pub radius: f32,
}

impl Collider for Sphere {
    fn collide(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<CollisionInfo> {
        let Ray {direction, origin, ..} = ray;
        let relative_origin = origin - self.centre;

        let a = direction.dot(direction);
        let b = 2.0*direction.dot(&relative_origin);
        let c = relative_origin.dot(&relative_origin) - self.radius * self.radius;

        let delta = b*b - 4.0*a*c;

        if delta < 0.0 { return None }

        let delta_sqrt = delta.sqrt();

        let t1 = (-b - delta_sqrt) / (2.0*a);
        let t2 = (-b + delta_sqrt) / (2.0*a);

        
        if min_t <= t1 && t1 <= max_t {
            let point = origin + t1*direction;
            let normal = (point - self.centre) / self.radius;
            return Some(CollisionInfo{
                point,
                normal,
                t: t1,
                inside: false,
            });
        }
        else if min_t <= t2 && t2 <= max_t {
            let point = origin + t2*direction;
            let normal = (point - self.centre) / self.radius;
            return Some(CollisionInfo{
                point,
                normal:-normal,
                t: t2,
                inside: true,
            });
        }

        None
    }
}


pub struct Triangle {
    pub origin: Vec3f,
    pub side1 : Vec3f,
    pub side2 : Vec3f,
}

impl Collider for Triangle {
    fn collide(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<CollisionInfo> {
        // based on https://stackoverflow.com/questions/42740765/intersection-between-line-and-triangle-in-3d

        let origin = ray.origin - self.origin;
        let direction = &ray.direction;
        let n = self.side1.cross(&self.side2);
        let det = direction.dot(&n);

        if det.abs() < 1e-4 {
            return None;
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

            Some(
                CollisionInfo {
                    point: ray.direction*t + ray.origin,
                    normal,
                    t,
                    inside
                }
            )
        }
        else {
            None
        }
    }
}

pub struct Plate {
    pub origin: Vec3f,
    pub side1: Vec3f,
    pub side2 : Vec3f,
}

impl Collider for Plate {
    fn collide(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<CollisionInfo> {
        let origin = ray.origin - self.origin;
        let direction = &ray.direction;
        let n = self.side1.cross(&self.side2);
        let det = direction.dot(&n);

        if det.abs() < 1e-4 {
            return None;
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
        

        if t > min_t && t < max_t && u > -0.5 && v > -0.5
                                  && u <  0.5 && v <  0.5 {
            let inside = n.dot(&direction) > 0.0;
            let normal = if inside {-n.normalize()} else {n.normalize()}; 
            
            Some(
                CollisionInfo {
                    point: ray.direction*t + ray.origin,
                    normal,
                    t,
                    inside 
                }
            )
        }
        else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vec3f>,
    pub triangles: Vec<[usize;3]>,
    pub uvs: Vec<Vec2f>,
    pub bvh: BVH
}

impl Mesh {
    pub fn new(verteces: &[Vec3f], triangles: &[[usize;3]], uvs : &[Vec2f]) -> Mesh{
        let vertices = verteces.to_owned();
        let mut triangles = triangles.to_owned();
        let uvs = uvs.to_owned();

        let bvh = BVH::new(&vertices, &mut triangles);
        
        Mesh {
            vertices,
            triangles,
            uvs,
            bvh
        }
    }

    pub fn load_obj(path: &Path) -> Result<Mesh,tobj::LoadError> {
        let params = tobj::LoadOptions {
            single_index: true,
            ..tobj::LoadOptions::default()
        };
        let (models, _) = tobj::load_obj(path, &params)?;
        if models.len() != 1 {
            Err(tobj::LoadError::InvalidLoadOptionConfig)
        }
        else {
            let tobj::Mesh {positions, indices,texcoords, ..} = &models[0].mesh;
            assert!(positions.len() % 3 == 0, "Position array's length is a not multiple of 3");
            assert!(indices  .len() % 3 == 0, "Position array's length is a not multiple of 3");
            assert!(texcoords.len() % 2 == 0, "Position array's length is a not multiple of 2");
            
            let vertices : Vec<_> = (0..positions.len() / 3).map(|i| (3*i,3*i+1,3*i+2))
                                                            .map(|(i,j,k)| (positions[i], positions[j], positions[k]))
                                                            .map(|(x,y,z)| Vec3f::new(x,y,z)).collect();
            let mut triangles : Vec<_> = (0..indices.len() / 3).map(|i| (3*i,3*i+1,3*i+2))
                                                               .map(|(i,j,k)| (indices[i] as usize, indices[j]as usize, indices[k]as usize))
                                                               .map(|(i,j,k)| [i,j,k]).collect();
            let uvs : Vec<_> = (0..texcoords.len() / 2).map(|i| (2*i,2*i+1))
                                                       .map(|(i,j)| (texcoords[i], texcoords[j]))
                                                       .map(|(u,v)| Vec2f::new(u,v)).collect();
            
            let bvh = BVH::new(&vertices, &mut triangles);
            
            Ok(Mesh {
                vertices,
                triangles,
                uvs,
                bvh
            })
        }
    }
}

impl Collider for Mesh {
    fn collide(&self, ray: &Ray, min_t: f32, mut max_t: f32) -> Option<CollisionInfo> {
        let (begin, count) = self.bvh.intersects(ray, min_t, max_t);

        let triangles = self.triangles[begin.. begin+count].iter().map( |[i1,i2,i3]| 
            Triangle {
                origin: self.vertices[*i1],
                side1:  self.vertices[*i2] - self.vertices[*i1],
                side2:  self.vertices[*i3] - self.vertices[*i1],
            }
        );

        let mut nearest: Option<CollisionInfo> = None;

        for triangle in triangles {
            let collision = triangle.collide(ray, min_t, max_t);
            
            if let Some(collision_info) = &collision {
                if collision_info.t < max_t {
                    max_t = collision_info.t;
                    nearest = collision;
                }
            }
        }
        nearest
    }
}