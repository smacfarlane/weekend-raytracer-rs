use hit::Hittable;
use image::ImageBuffer;
use interval::Interval;
use sphere::Sphere;
use vec3::{Color, Vec3};

use crate::{camera::Camera, hit::HitList, ray::Ray};

mod camera;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // World
    let mut world = HitList::new();
    world.push(Sphere::new(Vec3::from(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::from(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new(800, 16.0 / 9.0);

    let image = camera.render(&world);

    let _ = image.save("test.png");
}
