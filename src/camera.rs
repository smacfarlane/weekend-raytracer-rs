use crate::vec3::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    camera_center: Vec3,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (width as f64 / height as f64);
        let camera_center = Vec3::new();
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u.div(width as f64);
        let pixel_delta_v = viewport_v.div(height as f64);

        let viewport_upper_left = camera_center
            - Vec3::from(0.0, 0.0, focal_length)
            - viewport_u.div(2.0)
            - viewport_v.div(2.0);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v).mul(0.5);

        Self {
            aspect_ratio,
            focal_length,
            viewport_height,
            viewport_width,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
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
}
