use crate::Material;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Lambertian { albedo: a }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        Some((self.albedo, Ray::new(rec.p, target - rec.p)))
    }
}
