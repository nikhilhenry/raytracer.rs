use rand;
use std::rc::Rc;

use crate::{color, hittable, material, pos, sphere::Sphere, vector::Vec3};

pub fn random_scene() -> hittable::HittableList {
    let mut world = hittable::HittableList::new();

    let ground_material = Rc::new(material::Lambertian {
        albedo: color!(0.5, 0.5, 0.5),
    });
    world.add(Rc::new(Sphere::new(
        pos!(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // add random spheres
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            let choose_mat = rand::random::<f32>();

            if (&center - &pos!(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn material::Material> = if choose_mat < 0.8 {
                    // diffuse material
                    let albedo = Vec3::random() * Vec3::random();
                    Rc::new(material::Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_bound(0.5, 1.0);
                    let fuzz: f32 = rand::random::<f32>();
                    Rc::new(material::Metal::new(albedo, fuzz))
                } else {
                    Rc::new(material::Dielectric { ir: 1.5 })
                };

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // add main spheres
    let material1 = Rc::new(material::Dielectric { ir: 1.5 });
    world.add(Rc::new(Sphere::new(pos!(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(material::Lambertian {
        albedo: color!(0.4, 0.2, 0.1),
    });
    world.add(Rc::new(Sphere::new(pos!(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(material::Metal::new(color!(0.4, 0.2, 0.1), 0.0));
    world.add(Rc::new(Sphere::new(pos!(4.0, 1.0, 0.0), 1.0, material3)));

    world
}
