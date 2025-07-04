#[cfg(test)]
mod tests;

use std::io::{BufWriter, Write};
use std::ops::{Add, Deref, DerefMut, Index, IndexMut, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vec3([f64; 3]);

impl Deref for Vec3 {
    type Target = [f64];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Default for Vec3 {
    fn default() -> Self {
        Vec3([0., 0., 0.])
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3([x, y, z])
    }
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            0. - self.x() * other.z() + self.z() * other.x(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
    fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    pub fn unit_vector(self) -> Self {
        let r = self.length_squared().sqrt();
        if r > 0.0 { 1. / r * self } else { self }
    }
    pub fn len(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn write_color<T: std::io::Write>(
        &self,
        buf_writer: &mut BufWriter<T>,
    ) -> std::io::Result<()> {
        writeln!(
            buf_writer,
            "{} {} {}",
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8
        )
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.cross(&rhs)
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
