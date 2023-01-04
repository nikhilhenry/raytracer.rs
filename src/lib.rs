mod ray;
use std::io;
use std::io::Write;
mod vector;

fn ray_color(r: ray::Ray) -> vector::Vec3 {
    let unit_direction = vector::unit_vector(r.dir());
    let t = 0.5 * (unit_direction[1] + 1.0);
    vector::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vector::Vec3::new(0.5, 0.7, 1.0) * t
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

    let origin = vector::Vec3::new(0.0, 0.0, 0.0);
    let horizontal = vector::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vector::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone()
        - horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - vector::Vec3::new(0.0, 0.0, focal_length);

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
            let pixel_color = ray_color(r);
            write_color!(pixel_color);
        }
        j -= 1;
    }
}
