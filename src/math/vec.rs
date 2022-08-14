use raylib::{ffi, math};
use std::{
    f32::consts::PI,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0., y: 0. }
    }

    pub fn one() -> Vec2 {
        Vec2 { x: 1., y: 1. }
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn lenght_sqr(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn dot(&self, v: Vec2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn distance_to(&self, v: Vec2) -> f32 {
        ((self.x - v.x) * (self.x - v.x) + (self.y - v.y) * (self.y - v.y)).sqrt()
    }

    pub fn angle_to(&self, v: Vec2) -> f32 {
        let mut result = (v.y - self.y).atan2(v.x - self.x);
        if result < 0. {
            result += 2. * PI;
        }
        result
    }

    pub fn scale(&mut self, scale: f32) {
        *self *= scale;
    }

    pub fn scale_by(&self, scale: f32) -> Vec2 {
        *self * scale
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(&self) -> Vec2 {
        let length_sqr = self.lenght_sqr();
        if length_sqr == 0. {
            return *self;
        }
        *self / length_sqr.sqrt()
    }

    pub fn lerp(&self, v: Vec2, amount: f32) -> Vec2 {
        Vec2 {
            x: self.x + amount * (v.x - self.x),
            y: self.y + amount * (v.y - self.y),
        }
    }

    pub fn clamp(&self, min: f32, max: f32) -> Vec2 {
        Vec2 {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(v: (f32, f32)) -> Vec2 {
        let (x, y) = v;
        Vec2 { x, y }
    }
}

impl From<math::Vector2> for Vec2 {
    fn from(v: math::Vector2) -> Vec2 {
        Vec2 { x: v.x, y: v.y }
    }
}

impl Into<math::Vector2> for Vec2 {
    fn into(self) -> math::Vector2 {
        math::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<ffi::Vector2> for Vec2 {
    fn from(v: ffi::Vector2) -> Self {
        Vec2 { x: v.x, y: v.y }
    }
}

impl Into<ffi::Vector2> for Vec2 {
    fn into(self) -> ffi::Vector2 {
        ffi::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a, 'b> Add<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn add(self, rhs: &'b Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<'a> Add<f32> for &'a Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: &'b Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<'a> Sub<f32> for &'a Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<'a, 'b> Mul<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: &'b Vec2) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<'a> Mul<f32> for &'a Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<'a, 'b> Div<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn div(self, rhs: &'b Vec2) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<'a> Div<f32> for &'a Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
