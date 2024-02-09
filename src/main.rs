use hit::Hittable;
use image::ImageBuffer;
use interval::Interval;
use sphere::Sphere;
use vec3::{Color, Vec3};

use crate::{camera::Camera, hit::HitList, material::Material, ray::Ray};

mod camera;
mod hit;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // Materials
    let ground = Material::Lambertian(Color::from(0.8, 0.8, 0.0));
    let center = Material::Lambertian(Color::from(0.7, 0.3, 0.3));
    let left = Material::Metal(Color::from(0.8, 0.8, 0.8), 0.3);
    let right = Material::Metal(Color::from(0.8, 0.6, 0.2), 1.0);

    // World
    let mut world = HitList::new();
    world.push(Sphere::new(Vec3::from(0.0, -100.5, -1.0), 100.0, ground));
    world.push(Sphere::new(Vec3::from(0.0, 0.0, -1.0), 0.5, center));
    world.push(Sphere::new(Vec3::from(-1.0, 0.0, -1.0), 0.5, left));
    world.push(Sphere::new(Vec3::from(1.0, 0.0, -1.0), 0.5, right));

    let camera = Camera::new(800, 16.0 / 9.0);

    let image = camera.render(&world);

    let _ = image.save("test.png");
}
