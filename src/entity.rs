use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use serde::{Deserialize, Serialize};

const EXPONENT: usize = 2;
const GRAVITY: f32 = 9.8 * 1.0 / (10_i32.pow(EXPONENT as u32) as f32);
const MASS: f32 = 1.0;
const TERMINAL_VELOCITY: f32 = 1.0;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
pub type Pos = Vec2;
pub type Vel = Vec2;

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    fn angle(&self, rhs: Self) -> f32 {
        self.dot(rhs)/(self.magnitude() * rhs.magnitude()).acos()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pos: Pos, // [0..=1]
    pub vel: Vel,
    rad: f32,
}

impl Entity {
    pub fn new(pos: Pos, vel: Vel, rad: f32) -> Self {
        Self { pos, vel, rad }
    }

    pub fn step(&mut self, delta: f32) {
        self.gravity(delta);
        // self.drag(delta);
        self.pos += self.vel * delta;

        // bounds check
        if self.pos.y < 0.0 {
            self.collide(Vec2::new(0.0, 1.0));
        }

        if self.vel.x.abs() > TERMINAL_VELOCITY {
            self.vel.x = TERMINAL_VELOCITY * self.vel.x.signum()
        }
        if self.vel.y.abs() > TERMINAL_VELOCITY {
            self.vel.y = TERMINAL_VELOCITY * self.vel.y.signum()
        }
    }

    fn collide(&mut self, normal: Vec2) {
        let normal = normal.normalize();
        self.vel.angle(normal);
        self.vel = self.vel - normal * 2.0*self.vel.dot(normal);
        self.vel = self.vel.normalize() * (self.vel.magnitude()/1.6);
    }

    fn freeze(&mut self) {
        self.vel = Vec2::splat(0.0);
    }

    fn gravity(&mut self, delta: f32) {
        self.vel.y -= GRAVITY * MASS * delta
    }

    fn drag(&mut self, delta: f32) {
        let dir = self.vel.normalize();
        let speed = self.vel.magnitude();
        let drag = 0.01;

        self.vel += -dir * drag / speed * delta;
    }
}
