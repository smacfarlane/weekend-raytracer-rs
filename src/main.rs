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

fn main() {
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

    let mut camera = CameraBuilder::new();
    camera.image_width(800);
    camera.look_at(Vec3::from(0.0, 0.0, -1.0));
    camera.look_from(Vec3::from(-2.0, 2.0, 1.0));
    camera.v_fov(20.0);
    camera.samples(25);
    camera.max_depth(10);
    let camera = camera.build();

    let image = camera.render(&world);

    let _ = image.save("test.png");
}
