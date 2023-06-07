use crate::{ray::Ray, vector::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin.clone()
            - horizontal.clone() / 2.0
            - vertical.clone() / 2.0
            - Vec3::new(0.0, 0.0, focal_length);
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let dir = ((&self.horizontal * u) + (&self.vertical * v))
            + (&self.lower_left_corner - &self.origin);
        Ray::new(&self.origin, &dir)
    }
}
