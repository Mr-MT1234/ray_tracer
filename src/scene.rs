use core::f32;
use std::fmt::Debug;
use std::io::{BufReader, Write};
use std::path::Path;

use crate::{hitables::*, vec3_to_vec4, Camera, Collider, CollisionReport, Mat3f, Mat4f, Material, Mesh, Vec3f};
use crate::commun_types::Ray;
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MeshHandle(usize);
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MatearialHandle(usize);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "MinimalObject", into = "MinimalObject")]
pub struct Object {
    transform: Mat4f,
    inv_transform: Mat4f,
    normal_mat: Mat3f,
    mesh: MeshHandle,
    material: MatearialHandle,
}

impl Object {
    pub fn new(mesh: MeshHandle, transform: Mat4f, material: MatearialHandle) -> Object {
        let inv_transform = transform.try_inverse().unwrap();
        let normal_mat = Mat3f::new(
            inv_transform.m11,inv_transform.m21, inv_transform.m31,
            inv_transform.m12,inv_transform.m22, inv_transform.m32,
            inv_transform.m13,inv_transform.m23, inv_transform.m33
        );
        
        Object {
            mesh,
            material,
            transform,
            inv_transform,
            normal_mat,
        }
    }

    pub fn get_transform(&self) -> &Mat4f {
        &self.transform
    }

    pub fn set_transform(&mut self, new_transform: Mat4f) {
        let inv_transform = new_transform.try_inverse().unwrap();
        let normal_mat = Mat3f::new(
            inv_transform.m11,inv_transform.m21, inv_transform.m31,
            inv_transform.m12,inv_transform.m22, inv_transform.m32,
            inv_transform.m13,inv_transform.m23, inv_transform.m33
        );

        self.transform = new_transform;
        self.inv_transform = inv_transform;
        self.normal_mat = normal_mat;
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct MinimalObject {
    transform: Mat4f,
    mesh: MeshHandle,
    material: MatearialHandle,
}

impl From<MinimalObject> for Object {
    fn from(value: MinimalObject) -> Self {
        let inv_transform = value.transform.try_inverse().unwrap();
        let normal_mat = Mat3f::new(
            inv_transform.m11,inv_transform.m21, inv_transform.m31,
            inv_transform.m12,inv_transform.m22, inv_transform.m32,
            inv_transform.m13,inv_transform.m23, inv_transform.m33
        );
        Object {
            transform: value.transform,
            mesh: value.mesh,
            material: value.material,
            inv_transform,
            normal_mat
        }
    }
}

impl Into<MinimalObject> for Object {
    fn into(self) -> MinimalObject {
        MinimalObject {
            transform: self.transform,
            mesh: self.mesh,
            material: self.material
        }
    }
}

#[typetag::serde(tag="type")]
pub trait Environment : Debug + Sync {
    fn sample(&self, direction: &Vec3f) -> Vec3f;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantEnvironment {
    pub color: Vec3f
}

#[typetag::serde]
impl Environment for ConstantEnvironment {
    fn sample(&self, _direction: &Vec3f) -> Vec3f {
        self.color
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkyEnvironment {
    pub sun_direction: Vec3f,
    pub sun_color: Vec3f,
    pub up_color: Vec3f,
    pub down_color: Vec3f,
    pub sun_size: f32,
}

#[typetag::serde]
impl Environment for SkyEnvironment {
    fn sample(&self, direction: &Vec3f) -> Vec3f {
        let v = -self.sun_direction.dot(&direction.normalize());
        if v > 1. - self.sun_size {
            self.sun_color
        }
        else {
            let u = direction.y;
            let color = u*self.up_color + (1.0 - u)*self.down_color;
            color
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    meshes: Vec<Mesh>,
    materials: Vec<Box<dyn Material>>,
    objects: Vec<Object>,
    pub camera: Camera,
    pub environment: Box<dyn Environment>
} 

impl Scene {
    pub fn new(camera: Camera, environment: Box<dyn Environment>) -> Scene {
        Scene { 
            objects: Vec::new(), 
            meshes: Vec::new(), 
            materials: Vec::new(), 
            camera,
            environment
        }
    }

    pub fn load(path: impl AsRef<Path>) -> std::io::Result<Scene> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let scene = serde_json::from_reader(reader)?;
        Ok(scene)
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
        self.meshes.push(mesh);
        MeshHandle(self.meshes.len() - 1)
    }

    pub fn add_material(&mut self, material: Box<dyn Material>) -> MatearialHandle {
        self.materials.push(material);
        MatearialHandle(self.materials.len() - 1)
    }


    fn hit_object(&self, object: &Object, ray: &Ray, min_t: f32, max_t: f32) -> (Option<HitInfo>, CollisionReport) {
        let mesh = &self.meshes[object.mesh.0];
        let material = &self.materials[object.material.0];

        let local_ray = Ray {
            origin: (object.inv_transform * vec3_to_vec4(&ray.origin,1.0)).xyz(),
            direction: (object.inv_transform * vec3_to_vec4(&ray.direction,0.0)).xyz(),
        };
        let (collision, report) = mesh.collide(&local_ray, min_t, max_t);
        
        let collision = collision.map(|info| HitInfo {
            point: (object.transform * vec3_to_vec4(&info.point,1.0)).xyz(),
            normal: (object.normal_mat * info.normal).normalize(),
            material: material.as_ref(),
            t: info.t,
            inside: info.inside,
            uv: info.uv
        });

        (collision, report)
    }
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write(serialized.as_bytes())?;
        Ok(())

    }
}

impl Optical for Scene {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> (Option<HitInfo>, CollisionReport) {
        let mut hit_info: Option<HitInfo> = None;
        let mut report = CollisionReport::default();
        for object in &self.objects {
            let old_t = hit_info.as_ref().map(|hit| hit.t).unwrap_or(max_t);
            let (new_hit, new_report) = self.hit_object(object, ray, min_t, old_t);
            let new_t = new_hit.as_ref().map(|hit| hit.t).unwrap_or(f32::INFINITY);
            report.aabb_tests += new_report.aabb_tests;
            report.triangle_tests += new_report.triangle_tests;
            if new_t < old_t {
                hit_info = new_hit;
            }
        }

        (hit_info, report)
    }
}