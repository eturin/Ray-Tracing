use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    _w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).make_unit_vector();
        let u = vup.cross(&w).make_unit_vector();
        let v = w.cross(&u).make_unit_vector();
        let horizontal = 2. * half_width * focus_dist * u;
        let vertical = 2. * half_height * focus_dist * v;
        let lower_left_corner = lookfrom - 0.5 * horizontal - 0.5 * vertical - focus_dist * w;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin: lookfrom,
            _w: w,
            u,
            v,
            lens_radius: aperture / 2.,
        }
    }
    pub fn lower_left_corner(&self) -> &Vec3 {
        &self.lower_left_corner
    }
    pub fn horizontal(&self) -> &Vec3 {
        &self.horizontal
    }
    pub fn vertical(&self) -> &Vec3 {
        &self.vertical
    }
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            *self.origin() + offset,
            *self.lower_left_corner() + s * *self.horizontal() + t * *self.vertical()
                - *self.origin()
                - offset,
        )
    }
}
