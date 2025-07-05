#[cfg(test)]
mod tests;

use crate::hit_record::HitRecord;
use crate::Hitable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Vec3::default(),0.)
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere{center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool {
        todo!()
    }
}