use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{self, Color},
};
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
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
            Self::Metal(albedo, fuzz) => {
                let reflected = vec3::reflect(&r_in.direction().unit(), &record.normal);
                let scattered =
                    Ray::from(record.p, reflected + vec3::random_unit_vector().mul(*fuzz));

                Some((*albedo, scattered))
            }
            Self::Dielectric(ir) => {
                let refraction_ratio = if record.front_face { 1.0 / *ir } else { *ir };

                let unit_direction = r_in.direction().unit();

                let cos_theta = -unit_direction.dot(&record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > thread_rng().gen_range(0.0..1.0)
                {
                    vec3::reflect(&unit_direction, &record.normal)
                } else {
                    vec3::refract(&unit_direction, &record.normal, refraction_ratio)
                };

                let scattered = Ray::from(record.p, direction);

                Some((Color::white(), scattered))
            }
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
