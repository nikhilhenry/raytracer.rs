use raytracer;
use std::io;
use std::io::Write;
fn main() {
    // image dims
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Renderer

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        eprint!("\r Scanlines remaining: {j}");
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_color = raytracer::Vec3::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.25,
            );
            raytracer::write_color!(pixel_color);
        }
        j -= 1;
    }
}
