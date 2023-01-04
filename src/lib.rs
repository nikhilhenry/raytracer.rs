mod ray;
mod vector;
use std::io;
use std::io::Write;
use vector::Vec3;

fn hit_sphere(center: &Vec3, radius: f32, r: &ray::Ray) -> f32 {
    let oc = r.origin() - center;
    let a = vector::dot(r.dir(), r.dir());
    let b = 2.0 * vector::dot(&oc, r.dir());
    let c = vector::dot(&oc, &oc) - radius * radius;
    let discriminat = b * b - 4.0 * a * c;
    if discriminat < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminat.sqrt()) / (2.0 * a);
    }
}

fn ray_color(r: &ray::Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = vector::unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return Vec3::new(n['x'] + 1.0, n['y'] + 1.0, n['z'] + 1.0) * 0.5;
    }
    let unit_direction = vector::unit_vector(r.dir());
    let t = 0.5 * (unit_direction['y'] + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vector::Vec3::new(0.5, 0.7, 1.0) * t;
}

pub fn render() {
    //  Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone()
        - horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - Vec3::new(0.0, 0.0, focal_length);

    // Renderer
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        eprint!("\r Scanlines remaining: {j}");
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let dir = lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v
                - origin.clone();
            let r = ray::Ray::new(&origin, &dir);
            let pixel_color = ray_color(&r);
            write_color!(pixel_color);
        }
        j -= 1;
    }
}
