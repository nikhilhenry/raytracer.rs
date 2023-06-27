use crate::{
    hittable::HitRecord,
    ray::Ray,
    vector::{dot, random_unit_vector, reflect, unit_vector, Vec3},
};

type Scatter = Option<(Vec3, Ray)>;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let mut scatter_dir = hit.normal + random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = hit.normal;
        }
        let scattered = Ray::new(&hit.p, &scatter_dir);
        let attenuation = self.albedo;
        return Some((attenuation, scattered));
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let reflected = reflect(&unit_vector(r_in.dir()), &hit.normal);
        let scattered = Ray::new(&hit.p, &reflected);
        let attenuation = self.albedo;
        if dot(scattered.dir(), &hit.normal) > 0.0 {
            return Some((attenuation, scattered));
        } else {
            return None;
        }
    }
}
