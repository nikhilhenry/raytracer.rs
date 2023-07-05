use crate::material::Material;
use crate::ray;
use crate::vector;
use std::rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: vector::Vec3,
    pub normal: vector::Vec3,
    pub material: rc::Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
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

impl Drop for HittableList {
    fn drop(&mut self) {
        self.clear();
    }
}
