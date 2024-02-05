use image::ImageBuffer;
use vec3::Color;

use crate::{camera::Camera, ray::Ray};

mod camera;
mod ray;
mod vec3;

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::from(1.0, 1.0, 1.0).mul(1.0 - a) + Color::from(0.5, 0.7, 1.0).mul(a)
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: u32 = 800;
    let height = (width as f64 / aspect_ratio) as u32;
    assert!(height >= 1);

    // TODO: We calculate image_height seperate from Camera, yet they share aspect_ratio
    let camera = Camera::new(width, height);

    let mut imgbuf = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let ray = Ray::from(*camera.camera_center(), camera.ray_direction(x, y));
            let color = ray_color(&ray);

            let pixel = imgbuf.get_pixel_mut(x, y);

            *pixel = image::Rgb([color.r(), color.g(), color.b()]);
        }
    }

    let _ = imgbuf.save("test.png");
}
