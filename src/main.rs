use std::io;
use std::io::Write;
fn main() {
    // image dims
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        eprint!("\r Scanlines remaining: {j}");
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let r: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let r = (255.999 * r) as u32;
            let g = (255.999 * g) as u32;
            let b = (255.999 * b) as u32;
            print!("{r} {g} {b}\n");
        }
        j -= 1;
    }
}
