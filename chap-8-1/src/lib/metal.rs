use crate::Material;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(a: Vec3) -> Self {
        Metal { albedo: a }
    }
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2. * v.dot(n) * *n
    }
}
impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Metal::reflect(&r.direction().make_unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction().dot(&rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
