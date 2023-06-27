use std::rc;

use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector;
use crate::vector::Vec3;
pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat: rc::Rc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32, mat: rc::Rc<dyn Material>) -> Sphere {
        return Sphere {
            center: c,
            radius: r,
            mat,
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.dir().length_squared();
        let half_b = vector::dot(&oc, &r.dir());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = r.at(temp);
                let normal = (&p - &self.center) / self.radius;
                let front_face = vector::dot(r.dir(), &normal) < 0.0;

                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: if front_face { normal } else { normal * -1.0 },
                    front_face,
                    material: self.mat,
                });
            }
        }
        None
    }
}
