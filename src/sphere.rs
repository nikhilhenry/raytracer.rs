use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vector;
use crate::vector::Vec3;
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Sphere {
        return Sphere {
            center: c,
            radius: r,
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.dir().length_squared();
        let half_b = vector::dot(&oc, r.dir());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminat = half_b * half_b - a * c;
        let sqrtd = discriminat.sqrt();

        // finding the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let normal = (&p - &self.center) / self.radius;
        let outward_normal = (&p - &self.center) / self.radius;
        let front_face = vector::dot(r.dir(), &outward_normal) < 0.0;

        Some(HitRecord {
            p,
            normal: if front_face { normal } else { normal * -1.0 },
            t: root,
            front_face,
        })
    }
}
