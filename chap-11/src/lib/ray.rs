#[cfg(test)]
mod tests;

use super::vec3::Vec3;
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
    pub fn point_at_parametr(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
// значение по умолчанию
impl Default for Ray {
    fn default() -> Self {
        Ray::new(Vec3::default(), Vec3::default())
    }
}
