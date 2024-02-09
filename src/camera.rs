use image::ImageBuffer;
use rand::{thread_rng, Rng};

use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{self, Color, Vec3};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    aspect_ratio: f64,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    samples: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        Self::initialize(width, aspect_ratio)
    }

    pub fn render(&self, world: &impl Hittable) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut imgbuf = ImageBuffer::new(self.image_width, self.image_height);
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut color = Color::from(0.0, 0.0, 0.0);
                for sample in 0..self.samples {
                    let ray = self.get_ray(x, y);
                    color += Self::ray_color(&ray, self.max_depth, world);
                }

                color.scale(1.0 / self.samples as f64);
                color.to_gamma_space();

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
            samples: 25,
            max_depth: 10,
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

    fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        let interval = Interval::new(0.001, f64::INFINITY);

        if depth == 0 {
            return Color::from(0.0, 0.0, 0.0);
        }

        if let Some(object) = world.hit(ray, &interval) {
            // let direction = object.normal + vec3::random_unit_vector();
            // Self::ray_color(&Ray::from(object.p, direction), depth - 1, world).mul(0.5)

            match object.mat.scatter(ray, &object) {
                Some((attenuation, scattered)) => {
                    attenuation * Self::ray_color(&scattered, depth - 1, world)
                }
                None => Color::black(),
            }
        } else {
            let unit_direction = ray.direction().unit();
            let a = 0.5 * (unit_direction.y() + 1.0);
            Color::white().mul(1.0 - a) + Color::from(0.5, 0.7, 1.0).mul(a)
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + self.pixel_delta_u.mul(i as f64) + self.pixel_delta_v.mul(j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let origin = self.camera_center;
        let direction = pixel_sample - origin;

        Ray::from(origin, direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = thread_rng();
        let px = rng.gen_range(-0.5..0.5);
        let py = rng.gen_range(-0.5..0.5);

        self.pixel_delta_u.mul(px) + self.pixel_delta_v.mul(py)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::initialize(100, 1.0)
    }
}
