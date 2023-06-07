mod hittable;
use std::rc;
mod camera;
mod ray;
mod sphere;
mod vector;
use camera::Camera;
use rand::Rng;
use sphere::Sphere;
use std::io;
use std::io::Write;
use vector::Vec3;

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.0, INFINITY) {
        return (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = vector::unit_vector(r.dir());
    let t = 0.5 * (unit_direction['y'] + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vector::Vec3::new(0.5, 0.7, 1.0) * t;
}

const INFINITY: f32 = f32::INFINITY;
const PI: f32 = std::f32::consts::PI;

pub fn deg_to_rad(degress: f32) -> f32 {
    degress / PI
}

pub fn render() {
    //  Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World
    let mut world = hittable::HittableList::new();
    world.add(rc::Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(rc::Rc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let camera = Camera::new();
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

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            vector::write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
        j -= 1;
    }
}
