mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vector;
use camera::Camera;
use rand::Rng;
mod scene;
use std::io;
use std::io::Write;
use vector::Vec3;

const PI: f32 = std::f32::consts::PI;

#[inline]
pub fn deg_to_rad(degress: f32) -> f32 {
    degress * PI / 180.0
}

pub fn render() {
    //  Image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 5;

    let world = scene::basic_scene();

    // Camera
    let lookfrom = pos!(0.0, 0.6, 0.3);
    let lookat = pos!(0.0, 0.5, -1.0);
    let vup = pos!(0.0, 1.0, 0.0);
    let dist_to_focus = (&lookfrom - &lookat).length();
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        90.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Renderer
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    let mut j = (IMAGE_HEIGHT - 1) as i32;
    let mut rng = rand::thread_rng();
    while j >= 0 {
        eprint!("\r Scanlines remaining: {j}");
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color += ray::ray_color(&r, &world, MAX_DEPTH);
            }
            vector::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
        j -= 1;
    }
}
