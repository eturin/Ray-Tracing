use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub mod hit_record;
pub mod ray;
pub mod vec3;
mod sphere;

trait Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool;
}
