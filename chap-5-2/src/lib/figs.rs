use super::hit_record::HitRecord;
use crate::Hitable;
use crate::ray::Ray;

pub struct Figs {
    pub v: Vec<Box<dyn Hitable>>,
}

impl Figs {
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut hs = self
            .v
            .iter()
            .map(|fig| fig.hit(r, tmin, tmax))
            .filter(Option::is_some)
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        hs.sort_by(|a, b| b.t.total_cmp(&a.t));
        hs.pop()
    }
}
