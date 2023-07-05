use crate::{
    deg_to_rad,
    ray::Ray,
    vector::{cross, random_in_unit_disk, unit_vector, Vec3},
};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = deg_to_rad(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(&lookfrom - &lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = &u * viewport_width * focus_dist;
        let vertical = &v * viewport_height * focus_dist;
        let lower_left_corner = &origin - &(&horizontal / 2.0) - &vertical / 2.0 - &w * focus_dist;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = &self.u * rd['x'] + &self.v * rd['y'];
        let origin = &(&self.origin + &offset);
        let dir = ((&self.horizontal * u) + (&self.vertical * v))
            + &(&self.lower_left_corner - &self.origin)
            - offset;
        Ray::new(origin, &dir)
    }
}
