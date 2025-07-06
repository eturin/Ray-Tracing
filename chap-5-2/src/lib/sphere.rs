#[cfg(test)]
mod tests;

use crate::Hitable;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Vec3::default(), 0.)
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = 2. * oc.dot(r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b * b - 4. * a * c;

        if d > 0. {
            for t in [(-b - d.sqrt()) / (2. * a), (-b + d.sqrt()) / (2. * a)] {
                if tmin < t && t < tmax {
                    let p = r.point_at_parametr(t);
                    return Some(HitRecord {
                        t,
                        p,
                        normal: (p - self.center) / self.radius,
                    });
                }
            }
        }

        None
    }
}
