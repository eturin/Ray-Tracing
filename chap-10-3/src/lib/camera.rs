

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera([Vec3; 4]);

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom-lookat).make_unit_vector();
        let u = vup.cross(&w).make_unit_vector();
        let v = w.cross(&u).make_unit_vector();
        let mut lower_left_corner = Vec3::new(-half_width, -half_height, -1.);
        lower_left_corner = lookfrom - half_width*u - half_height*v - w;
        Camera([
            lower_left_corner,
            2.* half_width * u,
            2. * half_height * v,
            lookfrom,
        ])
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
