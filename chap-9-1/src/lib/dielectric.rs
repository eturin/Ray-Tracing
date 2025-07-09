use crate::Material;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let (outward_normal, ni_other_nt) = if r.direction().dot(&rec.normal) > 0. {
            (-rec.normal, self.ref_idx)
        } else {
            (rec.normal, 1. / self.ref_idx)
        };
        let attenuation = Vec3::new(1., 1., 1.);
        if let Some(refracted) = r.direction().refract(&outward_normal, ni_other_nt) {
            Some((attenuation, Ray::new(rec.p, refracted)))
        } else {
            Some((
                attenuation,
                Ray::new(rec.p, r.direction().make_unit_vector().reflect(&rec.normal)),
            ))
        }
    }
}
