use crate::Material;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2. * v.dot(n) * *n
    }
}
impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Metal::reflect(&r.direction().make_unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere());
        if scattered.direction().dot(&rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
