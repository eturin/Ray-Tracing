use crate::Material;
use crate::vec3::Vec3;
// сведения о точке пересечения луча с фигурой
pub struct HitRecord<'a> {
    pub t: f64,                             // значение параметра для уравнения прямой
    pub p: Vec3,                            // точка пересечения луча с фигурой
    pub normal: Vec3,                       // нормаль к поверхности фигуры в точке пересечения
    pub material: Option<&'a dyn Material>, // ссылка на материал
}
