use crate::vec3::Vec3;

pub struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
}
