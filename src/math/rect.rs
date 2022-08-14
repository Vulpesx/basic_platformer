use crate::math::vec::Vec2;
use raylib::{ffi, math};

#[derive(Clone, Copy)]
pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
    pub vel: Vec2,
}

impl Rect {
    pub fn new<V1: Into<Vec2>, V2: Into<Vec2>>(pos: V1, size: V2) -> Rect {
        Rect {
            pos: pos.into(),
            size: size.into(),
            vel: Vec2::zero(),
        }
    }
    pub fn point_in_rect(&self, p: Vec2) -> bool {
        p.x > self.pos.x
            && p.y > self.pos.y
            && p.x < self.pos.x + self.size.x
            && p.y < self.pos.y + self.size.y
    }

    pub fn rect_intersect(&self, r2: &Rect) -> bool {
        self.pos.x < r2.pos.x + r2.size.x
            && self.pos.x + self.size.x > r2.pos.x
            && self.pos.y < r2.pos.y + r2.size.y
            && self.pos.y + self.size.y > r2.pos.y
    }
}

impl From<math::Rectangle> for Rect {
    fn from(r: math::Rectangle) -> Self {
        Rect {
            pos: (r.x, r.y).into(),
            size: (r.width, r.height).into(),
            vel: Vec2::zero(),
        }
    }
}

impl Into<math::Rectangle> for Rect {
    fn into(self) -> math::Rectangle {
        math::Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.size.x,
            height: self.size.y,
        }
    }
}

impl From<ffi::Rectangle> for Rect {
    fn from(r: ffi::Rectangle) -> Self {
        Rect {
            pos: (r.x, r.y).into(),
            size: (r.width, r.height).into(),
            vel: Vec2::zero(),
        }
    }
}

impl Into<ffi::Rectangle> for Rect {
    fn into(self) -> ffi::Rectangle {
        ffi::Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.size.x,
            height: self.size.y,
        }
    }
}

pub fn point_in_rect(p: Vec2, r: &Rect) -> bool {
    p.x > r.pos.x && p.y > r.pos.y && p.x < r.pos.x + r.size.x && p.y < r.pos.y + r.size.y
}

pub fn rect_intersect(r1: &Rect, r2: &Rect) -> bool {
    r1.pos.x < r2.pos.x + r2.size.x
        && r1.pos.x + r1.size.x > r2.pos.x
        && r1.pos.y < r2.pos.y + r2.size.y
        && r1.pos.y + r1.size.y > r2.pos.y
}
