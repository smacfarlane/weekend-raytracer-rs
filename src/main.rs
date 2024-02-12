use image::ImageBuffer;
use rand::{thread_rng, Rng};
use sphere::Sphere;
use vec3::{Color, Vec3};

use crate::{camera::CameraBuilder, hit::HitList, material::Material};

mod camera;
mod hit;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

// Sample scene used while implementing
#[allow(dead_code)]
fn test_scene() -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    // Materials
    let ground = Material::Lambertian(Color::from(0.8, 0.8, 0.0));
    let center = Material::Lambertian(Color::from(0.1, 0.2, 0.5));
    let left = Material::Dielectric(1.5);
    let right = Material::Metal(Color::from(0.8, 0.6, 0.2), 0.0);

    // World
    let mut world = HitList::new();
    world.push(Sphere::new(Vec3::from(0.0, -100.5, -1.0), 100.0, ground));
    world.push(Sphere::new(Vec3::from(0.0, 0.0, -1.0), 0.5, center));
    world.push(Sphere::new(Vec3::from(-1.0, 0.0, -1.0), 0.5, left.clone()));
    world.push(Sphere::new(Vec3::from(-1.0, 0.0, -1.0), -0.4, left));
    world.push(Sphere::new(Vec3::from(1.0, 0.0, -1.0), 0.5, right));

    // Camera
    let mut camera = CameraBuilder::new();
    camera.image_width(800);
    camera.look_at(Vec3::from(0.0, 0.0, -1.0));
    camera.look_from(Vec3::from(-2.0, 2.0, 1.0));
    camera.v_fov(20.0);
    camera.samples(25);
    camera.max_depth(10);
    camera.defocus_angle(10.0);
    camera.focus_dist(3.4);
    let camera = camera.build();

    // Render
    camera.render(&world)
}

fn cover_art() -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HitList::new();

    let ground = Material::Lambertian(Color::from(0.5, 0.5, 0.5));
    world.push(Sphere::new(Vec3::from(0.0, -1000.0, 0.0), 1000.0, ground));

    let mut rng = thread_rng();
    let center_offset = Vec3::from(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Vec3::from(
                (a as f64) + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                (b as f64) + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - center_offset).length() > 0.9 {
                let mat = match choose_mat {
                    _x if _x < 0.8 => {
                        let albedo = Color::random_color() * Color::random_color();
                        Material::Lambertian(albedo)
                    }
                    _x if _x < 0.95 => {
                        let albedo = Color::random_color(); // TODO: 0.5 -> 1
                        let fuzz = rng.gen_range(0.0..0.5);
                        Material::Metal(albedo, fuzz)
                    }
                    _ => Material::Dielectric(1.5),
                };
                world.push(Sphere::new(center, 0.2, mat));
            }
        }
    }

    let mat1 = Material::Dielectric(1.5);
    let mat2 = Material::Lambertian(Color::from(0.4, 0.2, 0.1));
    let mat3 = Material::Metal(Color::from(0.7, 0.6, 0.5), 0.0);

    world.push(Sphere::new(Vec3::from(0.0, 1.0, 0.0), 1.0, mat1));
    world.push(Sphere::new(Vec3::from(-4.0, 1.0, 0.0), 1.0, mat2));
    world.push(Sphere::new(Vec3::from(4.0, 1.0, 0.0), 1.0, mat3));

    let mut camera = CameraBuilder::new();
    camera.image_width(1200);
    camera.samples(500);
    camera.max_depth(50);
    camera.v_fov(20.0);
    camera.look_from(Vec3::from(13.0, 2.0, 3.0));
    camera.look_at(Vec3::from(0.0, 0.0, 0.0));
    camera.v_up(Vec3::from(0.0, 1.0, 0.0));
    camera.defocus_angle(0.6);
    camera.focus_dist(10.0);

    let camera = camera.build();
    camera.render(&world)
}

fn main() {
    let args = std::env::args();

    if args.into_iter().nth(1).is_some_and(|s| s == "coverart") {
        let image = cover_art();
        let _ = image.save("cover-art.png");
    } else {
        let image = test_scene();
        let _ = image.save("sample.png");
    }
}
