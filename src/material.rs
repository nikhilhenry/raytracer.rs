use rand::Rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vector::{dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Vec3},
};

type Scatter = Option<(Vec3, Ray)>;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitRecord) -> Scatter {
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
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter {
        let reflected = reflect(&unit_vector(r_in.dir()), &hit.normal);
        let scattered = Ray::new(
            &hit.p,
            &(&reflected + &(&random_in_unit_sphere() * self.fuzz)),
        );
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

impl Dielectric {
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
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
        let cos_theta = dot(&(&unit_direction * -1.0), &hit.p).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng = rand::thread_rng();
        let random_double = rng.gen::<f32>();
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double
        {
            reflect(&unit_direction, &hit.normal)
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(&hit.p, &direction);
        return Some((attenuation, scattered));
    }
}
