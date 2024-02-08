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

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    let interval = Interval::new(0.0, f64::INFINITY);
    if let Some(object) = world.hit(ray, &interval) {
        (object.normal + Color::from(1.0, 1.0, 1.0)).mul(0.5)
    } else {
        let unit_direction = ray.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::from(1.0, 1.0, 1.0).mul(1.0 - a) + Color::from(0.5, 0.7, 1.0).mul(a)
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: u32 = 800;
    let height = (width as f64 / aspect_ratio) as u32;
    assert!(height >= 1);

    // World

    let mut world = HitList::new();
    world.push(Sphere::new(Vec3::from(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::from(0.0, -100.5, -1.0), 100.0));

    // TODO: We calculate image_height seperate from Camera, yet they share aspect_ratio
    let camera = Camera::new(width, height);

    let mut imgbuf = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let ray = Ray::from(*camera.camera_center(), camera.ray_direction(x, y));
            let color = ray_color(&ray, &world);

            let pixel = imgbuf.get_pixel_mut(x, y);

            *pixel = image::Rgb([color.r(), color.g(), color.b()]);
        }
    }

    let _ = imgbuf.save("test.png");
}
