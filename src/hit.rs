use crate::{ray::Ray, vec3::Vec3};
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct HitRecord {
    p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Vec3, normal: Vec3, t: f64) -> Self {
        let front_face = ray.direction().dot(&normal) < 0.0;
        let mut normal = normal;
        if !front_face {
            normal = -normal;
        }

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub struct HitList<T: Hittable>(Vec<T>);

impl<T: Hittable> HitList<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T: Hittable> Deref for HitList<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Hittable> DerefMut for HitList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: Hittable> Hittable for HitList<T> {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest = ray_tmax;

        let mut record = None;

        for object in self.0.iter() {
            if let Some(object) = object.hit(ray, ray_tmin, closest) {
                record = Some(object.clone());
                closest = object.t;
            }
        }

        record
    }
}
