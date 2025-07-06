use crate::hit_record::HitRecord;
use crate::ray::Ray;

pub mod camera;
pub mod figs;
pub mod hit_record;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
