use crate::Material;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
    fn schlick(&self, cosine: f64) -> f64 {
        let mut r0 = (1. - self.ref_idx) / (1. + self.ref_idx);
        r0 *= r0;
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}
impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let (outward_normal, ni_other_nt, cosine) = if r.direction().dot(&rec.normal) > 0. {
            (
                -rec.normal,
                self.ref_idx,
                self.ref_idx * r.direction().dot(&rec.normal) / r.direction().len(),
            )
        } else {
            (
                rec.normal,
                1. / self.ref_idx,
                -r.direction().dot(&rec.normal) / r.direction().len(),
            )
        };
        let attenuation = Vec3::new(1., 1., 1.);
        if let Some(refracted) = r.direction().refract(&outward_normal, ni_other_nt) {
            let mut rng = rand::rng();
            if self.schlick(cosine) < rng.random_range(0.0..1.0) {
                return Some((attenuation, Ray::new(rec.p, refracted)));
            }
        }

        Some((
            attenuation,
            Ray::new(rec.p, r.direction().make_unit_vector().reflect(&rec.normal)),
        ))
    }
}
