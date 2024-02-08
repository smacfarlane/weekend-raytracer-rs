use image::ImageBuffer;

use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    aspect_ratio: f64,
    // focal_length: f64,
    // viewport_height: f64,
    // viewport_width: f64,
    camera_center: Vec3,
    // viewport_u: Vec3,
    // viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        Self::initialize(width, aspect_ratio)
    }

    pub fn render(&self, world: &impl Hittable) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut imgbuf = ImageBuffer::new(self.image_width, self.image_height);
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let ray = Ray::from(self.camera_center, self.ray_direction(x, y));
                let color = Self::ray_color(&ray, world);

                let pixel = imgbuf.get_pixel_mut(x, y);

                *pixel = image::Rgb([color.r(), color.g(), color.b()]);
            }
        }

        imgbuf
    }

    fn initialize(width: u32, aspect_ratio: f64) -> Self {
        let camera_center = Vec3::new();
        let mut height = (width as f64 / aspect_ratio) as u32;
        if height < 1 {
            height = 1
        }

        // Viewport Dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (width as f64 / height as f64);

        // Calculate vectors across horizontal and down vertical viewport edges
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

        // Calculate horizontal and vertical detla vectors from pixel to pixel
        let pixel_delta_u = viewport_u.div(width as f64);
        let pixel_delta_v = viewport_v.div(height as f64);

        // Calculate location of upper left pixel
        let viewport_upper_left = camera_center
            - Vec3::from(0.0, 0.0, focal_length)
            - viewport_u.div(2.0)
            - viewport_v.div(2.0);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v).mul(0.5);

        Self {
            image_width: width,
            image_height: height,
            aspect_ratio,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }

    pub fn camera_center(&self) -> &Vec3 {
        &self.camera_center
    }

    pub fn ray_direction(&self, x: u32, y: u32) -> Vec3 {
        let pixel_center =
            self.pixel00_loc + self.pixel_delta_u.mul(x as f64) + self.pixel_delta_v.mul(y as f64);
        pixel_center - self.camera_center
    }

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
}
impl Default for Camera {
    fn default() -> Self {
        Self::initialize(100, 1.0)
    }
}
