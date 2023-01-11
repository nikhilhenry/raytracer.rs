use crate::ray;
use crate::vector;
use crate::vector::Vec3;
use std::rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: vector::Vec3,
    pub normal: vector::Vec3,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &Vec3) {
        self.front_face = vector::dot(r.dir(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = outward_normal.clone() * -1.0;
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<rc::Rc<dyn Hittable>>,
}
impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        for sphere in &self.objects {
            if let Some(hit) = sphere.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: rc::Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }
}
