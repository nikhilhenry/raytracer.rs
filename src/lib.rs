mod hittable;
mod material;
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
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: u32 = 50;

    let world = random_scene();

    // Camera
    let lookfrom = pos!(13.0, 2.0, 3.0);
    let lookat = pos!(0.0, 0.0, 0.0);
    let vup = pos!(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
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

fn random_scene() -> hittable::HittableList {
    let mut world = hittable::HittableList::new();

    let ground_material = rc::Rc::new(material::Lambertian {
        albedo: color!(0.5, 0.5, 0.5),
    });
    world.add(rc::Rc::new(Sphere::new(
        pos!(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // add random spheres
    for a in -11..11 {
        for b in -11..11 {
            let mut rng = rand::thread_rng();
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (&center - &pos!(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: rc::Rc<dyn material::Material> = if choose_mat < 0.8 {
                    // diffuse material
                    let albedo = Vec3::random() * Vec3::random();
                    rc::Rc::new(material::Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_bound(0.5, 1.0);
                    let fuzz: f32 = rng.gen();
                    rc::Rc::new(material::Metal::new(albedo, fuzz))
                } else {
                    rc::Rc::new(material::Dielectric { ir: 1.5 })
                };

                world.add(rc::Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // add main spheres
    let material1 = rc::Rc::new(material::Dielectric { ir: 1.5 });
    world.add(rc::Rc::new(Sphere::new(
        pos!(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = rc::Rc::new(material::Lambertian {
        albedo: color!(0.4, 0.2, 0.1),
    });
    world.add(rc::Rc::new(Sphere::new(
        pos!(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = rc::Rc::new(material::Metal::new(color!(0.4, 0.2, 0.1), 0.0));
    world.add(rc::Rc::new(Sphere::new(
        pos!(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
