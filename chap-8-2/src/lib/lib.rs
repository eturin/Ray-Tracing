use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod camera;
pub mod figs;
pub mod hit_record;
pub mod lambertian;
pub mod metal;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}
