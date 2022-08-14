pub mod rect;
pub mod vec;

use std::ops::{Deref, DerefMut};

pub use rect::*;
pub use vec::*;

pub fn ray_vs_rect(
    ray_origin: &Vec2,
    ray_dir: &Vec2,
    target: &Rect,
    contact_point: &mut Vec2,
    contact_normal: &mut Vec2,
    t_hit_near: &mut f32,
) -> bool {
    let mut t_near = &(&target.pos - ray_origin) / ray_dir;
    let mut t_far = &(&(target.pos + target.size) - ray_origin) / ray_dir;

    if t_near.x > t_far.x {
        let np = t_near.x;
        t_near.x = t_far.x;
        t_far.x = np;
    }
    if t_near.y > t_far.y {
        let np = t_near.y;
        t_near.y = t_far.y;
        t_far.y = np;
    }

    if t_near.x > t_far.y || t_near.y > t_far.x {
        return false;
    }

    *t_hit_near = f32::max(t_near.x, t_near.y);
    let t_hit_far = f32::min(t_far.x, t_far.y);

    if t_hit_far < 0. {
        return false;
    }

    *contact_point = ray_origin + &(ray_dir * *t_hit_near);
    if t_near.x > t_near.y {
        if ray_dir.x < 0. {
            *contact_normal = Vec2::new(1., 0.);
        } else {
            *contact_normal = Vec2::new(-1., 0.);
        }
    } else if t_near.x < t_near.y {
        if ray_dir.y < 0. {
            *contact_normal = Vec2::new(0., 1.);
        } else {
            *contact_normal = Vec2::new(0., -1.);
        }
    }

    true
}

pub fn dynamic_rect_vs_rect(
    r_dynamic: &Rect,
    r_static: &Rect,
    contact_point: &mut Vec2,
    contact_normal: &mut Vec2,
    contact_time: &mut f32,
    elapsed_time: &f32,
) -> bool {
    if r_dynamic.vel.x == 0. && r_dynamic.vel.y == 0. {
        return false;
    }

    let e_target = Rect::new(
        r_static.pos - r_dynamic.size / 2.,
        r_static.size + r_dynamic.size,
    );

    if ray_vs_rect(
        &(r_dynamic.pos + r_dynamic.size / 2.),
        &(r_dynamic.vel * *elapsed_time),
        &e_target,
        contact_point,
        contact_normal,
        contact_time,
    ) {
        if *contact_time < 1. {
            return true;
        }
    }

    return false;
}
