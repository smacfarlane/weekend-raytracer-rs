use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3([f64; 3]);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn r(&self) -> u8 {
        (255.99 * self[0]) as u8
    }

    pub fn g(&self) -> u8 {
        (255.99 * self[1]) as u8
    }

    pub fn b(&self) -> u8 {
        (255.99 * self[2]) as u8
    }
}

impl Deref for Vec3 {
    type Target = [f64; 3];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3([0.0, 0.0, 0.0])
    }
    pub fn from(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[2] *= rhs;
        self[3] *= rhs;
    }

    pub fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1.0 / rhs);
    }

    pub fn mul(&self, rhs: f64) -> Vec3 {
        Vec3([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }

    pub fn div(&self, rhs: f64) -> Vec3 {
        Vec3([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ])
    }

    pub fn unit(&self) -> Vec3 {
        self.div(self.length())
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3([-self.0[0], -self.0[1], -self.0[2]])
    }
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
    }
}
impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3([self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] = self[0] + rhs[0];
        self[1] = self[1] + rhs[1];
        self[2] = self[2] + rhs[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self[0] = self[0] * rhs[0];
        self[1] = self[1] * rhs[1];
        self[2] = self[2] * rhs[2];
    }
}
