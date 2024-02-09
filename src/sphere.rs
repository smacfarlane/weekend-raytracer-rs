use crate::{
    hit::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center).div(self.radius);

        Some(HitRecord::new(ray, p, normal, root, self.material.clone()))
    }
}
