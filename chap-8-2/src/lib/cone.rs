use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{Hitable, Material};

pub struct Cone {
    apex: Vec3,           // Вершина конуса
    axis_dir: Vec3,        // Направление оси (нормированное)
    angle: f64,
    height: f64,
    pub material: Option<Box<dyn Material>>, // ссылка на материал
}

impl Default for Cone {
    fn default() -> Self {
        Cone::new(Vec3::default(),Vec3::default(),0., 0., None)
    }
}

impl Cone {
    pub fn new(apex: Vec3, axis_dir: Vec3, angle: f64, height: f64, material: Option<Box<dyn Material>>) -> Self {
        Cone {
            apex,
            axis_dir,
            angle,
            height,
            material,
        }
    }
}

impl Hitable for Cone {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let v = *r.origin() - self.apex;
        let d = r.direction();
        let a = &self.axis_dir;
        let k = self.angle.tan();
        let k2 = k * k;

        // Коэффициенты квадратного уравнения
        let aa = d.dot(d) - (1. + k2) * d.dot(a).powf(2.);
        let bb = 2. * (v.dot(d) - (1. + k2) * v.dot(a) * d.dot(a));
        let cc = v.dot(&v) - (1. + k2) * v.dot(a).powf(2.);

        // Решаем квадратное уравнение
        let discriminant = bb * bb - 4.* aa * cc;
        if discriminant < 0. { return None; }

        let sqrtd = discriminant.sqrt();
        let mut root = (-bb - sqrtd) / (2.* aa);
        if root < tmin || root > tmax {
            root = (-bb + sqrtd) / (2.* aa);
            if root < tmin || root > tmax { return None; }
        }

        let p = r.point_at_parametr(root);

        // Проверка высоты
        let h = (p - self.apex).dot(a);
        if h < 0. || h > self.height { return None; }

        // Вычисление нормали
        let op = p - self.apex;
        let projection = op.dot(a) * *a;
        let ortho = op - projection;
        let mut normal = (ortho * (1./k2) - projection).make_unit_vector();

        // Корректировка направления нормали
        if normal.dot(d) > 0. {
            normal = -normal;
        }

        Some(HitRecord {
            t: root,
            p,
            normal,
            material: self.material.as_deref(),
        })        
    }
}
