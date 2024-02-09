use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};
use std::ops::{Deref, DerefMut};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Vec3, normal: Vec3, t: f64, mat: Material) -> Self {
        let front_face = ray.direction().dot(&normal) < 0.0;
        let mut normal = normal;
        if !front_face {
            normal = -normal;
        }

        Self {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut ray_t: Interval = ray_t.to_owned();

        let mut record = None;

        for object in self.0.iter() {
            if let Some(object) = object.hit(ray, &ray_t) {
                record = Some(object.clone());
                ray_t.max = object.t;
            }
        }

        record
    }
}
