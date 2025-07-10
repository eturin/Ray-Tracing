use super::hit_record::HitRecord;
use crate::Hitable;
use crate::ray::Ray;

pub struct Figs {
    pub v: Vec<Box<dyn Hitable>>,
}

impl Figs {
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        self.v
            .iter()
            .filter_map(|object| object.hit(r, tmin, tmax))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
