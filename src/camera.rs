use image::ImageBuffer;
use rand::{thread_rng, Rng};

use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{self, Color, Vec3};

#[allow(dead_code)]
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
    look_from: Vec3,
    look_at: Vec3,
    v_up: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&self, world: &impl Hittable) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut imgbuf = ImageBuffer::new(self.image_width, self.image_height);
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut color = Color::from(0.0, 0.0, 0.0);
                for _ in 0..self.samples {
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

    fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        let interval = Interval::new(0.001, f64::INFINITY);

        if depth == 0 {
            return Color::from(0.0, 0.0, 0.0);
        }

        if let Some(object) = world.hit(ray, &interval) {
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

        let origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;

        Ray::from(origin, direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = thread_rng();
        let px = rng.gen_range(-0.5..0.5);
        let py = rng.gen_range(-0.5..0.5);

        self.pixel_delta_u.mul(px) + self.pixel_delta_v.mul(py)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = vec3::random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u.mul(p[0])) + (self.defocus_disk_v.mul(p[1]))
    }
}

#[allow(dead_code)]
pub struct CameraBuilder {
    image_width: u32,
    aspect_ratio: f64,
    samples: u32,
    max_depth: u32,
    v_fov: f64,
    look_at: Vec3,
    look_from: Vec3,
    v_up: Vec3,
    focus_dist: f64,
    defocus_angle: f64,
}

#[allow(dead_code)]
impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            samples: 100,
            max_depth: 50,
            v_fov: 90.0,
            look_from: Vec3::from(0.0, 0.0, -1.0),
            look_at: Vec3::from(0.0, 0.0, 0.0),
            v_up: Vec3::from(0.0, 1.0, 0.0),
            focus_dist: 10.0,
            defocus_angle: 0.0,
        }
    }

    pub fn image_width(&mut self, image_width: u32) {
        self.image_width = image_width;
    }
    pub fn aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
    }
    pub fn samples(&mut self, samples: u32) {
        self.samples = samples;
    }
    pub fn max_depth(&mut self, max_depth: u32) {
        self.max_depth = max_depth;
    }
    pub fn v_fov(&mut self, v_fov: f64) {
        self.v_fov = v_fov;
    }
    pub fn look_at(&mut self, look_at: Vec3) {
        self.look_at = look_at;
    }
    pub fn look_from(&mut self, look_from: Vec3) {
        self.look_from = look_from;
    }
    pub fn v_up(&mut self, v_up: Vec3) {
        self.v_up = v_up;
    }
    pub fn focus_dist(&mut self, focus_dist: f64) {
        self.focus_dist = focus_dist;
    }
    pub fn defocus_angle(&mut self, defocus_angle: f64) {
        self.defocus_angle = defocus_angle
    }

    pub fn build(&self) -> Camera {
        self.into()
    }
}

impl From<&CameraBuilder> for Camera {
    fn from(input: &CameraBuilder) -> Self {
        let mut height = (input.image_width as f64 / input.aspect_ratio) as u32;
        if height < 1 {
            height = 1
        }

        let camera_center = input.look_from;

        // Viewport Dimensions
        let theta = input.v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * input.focus_dist;
        let viewport_width = viewport_height * (input.image_width as f64 / height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        let w = (input.look_from - input.look_at).unit();
        let u = (input.v_up.cross(&w)).unit();
        let v = w.cross(&u);

        // Calculate vectors across horizontal and down vertical viewport edges
        let viewport_u = u.mul(viewport_width);
        let viewport_v = -v.mul(viewport_height);

        // Calculate horizontal and vertical detla vectors from pixel to pixel
        let pixel_delta_u = viewport_u.div(input.image_width as f64);
        let pixel_delta_v = viewport_v.div(height as f64);

        // Calculate location of upper left pixel
        let viewport_upper_left =
            camera_center - w.mul(input.focus_dist) - viewport_u.div(2.0) - viewport_v.div(2.0);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v).mul(0.5);

        let defocus_radius = (input.defocus_angle / 2.0).to_radians().tan() * input.focus_dist;
        Self {
            image_width: input.image_width,
            image_height: height,
            aspect_ratio: input.aspect_ratio,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples: input.samples,
            max_depth: input.max_depth,
            look_from: input.look_from,
            look_at: input.look_at,
            v_up: input.v_up,
            u,
            v,
            w,
            defocus_angle: input.defocus_angle,
            focus_dist: input.focus_dist,
            defocus_disk_u: u.mul(defocus_radius),
            defocus_disk_v: v.mul(defocus_radius),
        }
    }
}
