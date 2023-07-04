use crate::{
    hittable::HitRecord,
    ray::Ray,
    vector::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3},
};

type Scatter = Option<(Vec3, Ray)>;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let mut scatter_dir = &hit.normal + &random_unit_vector();
        if scatter_dir.near_zero() {
            scatter_dir = hit.normal.clone()
        }
        let scattered = Ray::new(&hit.p, &scatter_dir);
        let attenuation = self.albedo.clone();
        return Some((attenuation, scattered));
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let reflected = reflect(&unit_vector(r_in.dir()), &hit.normal);
        let scattered = Ray::new(&hit.p, &reflected);
        let attenuation = self.albedo.clone();
        if dot(scattered.dir(), &hit.normal) > 0.0 {
            return Some((attenuation, scattered));
        } else {
            return None;
        }
    }
}

pub struct Dielectric {
    pub ir: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = unit_vector(r_in.dir());
        let refracted = refract(&unit_direction, &hit.normal, refraction_ratio);
        let scattered = Ray::new(&hit.p, &refracted);
        if dot(scattered.dir(), &hit.normal) > 0.0 {
            return Some((attenuation, scattered));
        } else {
            return None;
        }
    }
}
