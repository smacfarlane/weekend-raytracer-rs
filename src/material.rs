use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{self, Color},
};

#[derive(Clone)]
pub enum Material {
    Lambertian(Color),
    Metal(Color),
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian(albedo) => {
                let mut direction = record.normal + vec3::random_unit_vector();
                if direction.near_zero() {
                    direction = record.normal;
                }

                let scattered = Ray::from(record.p, direction);

                Some((*albedo, scattered))
            }
            Self::Metal(albedo) => {
                let reflected = vec3::reflect(&r_in.direction().unit(), &record.normal);
                let scattered = Ray::from(record.p, reflected);

                Some((*albedo, scattered))
            }
        }
    }
}
