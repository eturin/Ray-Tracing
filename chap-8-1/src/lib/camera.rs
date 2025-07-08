#[cfg(test)]
mod tests;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera([Vec3; 4]);

impl Camera {
    pub fn new(lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Self {
        Camera([lower_left_corner, horizontal, vertical, origin])
    }
    pub fn lower_left_corner(&self) -> &Vec3 {
        &self.0[0]
    }
    pub fn horizontal(&self) -> &Vec3 {
        &self.0[1]
    }
    pub fn vertical(&self) -> &Vec3 {
        &self.0[2]
    }
    pub fn origin(&self) -> &Vec3 {
        &self.0[3]
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            *self.origin(),
            *self.lower_left_corner() + u * *self.horizontal() + v * *self.vertical()
                - *self.origin(),
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let lower_left_corner = Vec3::new(-2., -1., -1.);
        let horizontal = Vec3::new(4., 0., 0.);
        let vertical = Vec3::new(0., 2., 0.);
        let origin = Vec3::default();
        Camera::new(lower_left_corner, horizontal, vertical, origin)
    }
}
