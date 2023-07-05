use crate::{hittable, vector::unit_vector, vector::Vec3};
#[derive(Clone)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, dir: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            dir: dir.clone(),
        }
    }
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn at(&self, t: f32) -> Vec3 {
        (&self.dir * t) + &self.origin
    }
}

const INFINITY: f32 = f32::INFINITY;
pub fn ray_color(r: &Ray, world: &dyn hittable::Hittable, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Vec3::zero();
        }
    }
    let unit_direction = unit_vector(r.dir());
    let t = 0.5 * (unit_direction['y'] + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}
