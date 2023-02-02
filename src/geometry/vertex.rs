use glam::{Vec2, Vec3};
use std::ops::{Add, Mul, Sub, MulAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vertx {
    pub pos: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

impl Vertx {
    pub fn new(pos: Vec3, color: Vec3, uv: Vec2) -> Self {
        Self { pos, color, uv }
    }
}

impl MulAssign<f32> for Vertx {
    fn mul_assign(&mut self, rhs: f32) {
        self.pos *= rhs;
        self.color *= rhs;
        self.uv *= rhs;
    }
}

impl Add for Vertx {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.pos + rhs.pos;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self::new(position, color, uv)
    }
}

impl Sub for Vertx {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.pos - rhs.pos;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self::new(position, color, uv)
    }
}

impl Mul<f32> for Vertx {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.pos * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::new(position, color, uv)
    }
}
