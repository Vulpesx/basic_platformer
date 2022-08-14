use basic_platformer::{math::*, Scene};
use raylib::prelude::*;

pub struct PhysTest<'a> {
    thread: &'a RaylibThread,
    rects: Vec<Rect>,
}

impl<'a> PhysTest<'a> {
    pub fn new<'b>(thread: &'b RaylibThread) -> PhysTest<'b> {
        PhysTest {
            thread,
            rects: vec![
                Rect::new((10., 10.), (10., 30.)),
                Rect::new((10., 200.), (20., 20.)),
                Rect::new((30., 200.), (20., 20.)),
                Rect::new((50., 200.), (20., 20.)),
                Rect::new((70., 200.), (20., 20.)),
                Rect::new((90., 200.), (20., 20.)),
                Rect::new((110., 200.), (20., 20.)),
                Rect::new((130., 200.), (20., 20.)),
                Rect::new((150., 200.), (20., 20.)),
                Rect::new((170., 200.), (20., 20.)),
                Rect::new((190., 200.), (20., 20.)),
                Rect::new((5., 150.), (5., 70.)),
                Rect::new((210., 150.), (5., 70.)),
                Rect::new((100., 20.), (30., 100.)),
                Rect::new((50., 60.), (100., 30.)),
                Rect::new((150., 30.), (5., 5.)),
            ],
        }
    }
}

impl<'a> Scene for PhysTest<'a> {
    fn input(&mut self, rl: &mut raylib::RaylibHandle) {}
    fn update(&mut self, rl: &mut raylib::RaylibHandle) {
        let delta = rl.get_frame_time();
        let v_mouse: Vec2 = rl.get_mouse_position().into();

        let ray_dir = v_mouse - self.rects[0].pos;

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            self.rects[0].vel += ray_dir.normalized() * 100. * delta;
        }

        let mut cp = Vec2::zero();
        let mut cn = Vec2::zero();
        let mut ct = 0.;

        for i in 1..self.rects.len() {
            if dynamic_rect_vs_rect(
                &self.rects[0],
                &self.rects[i],
                &mut cp,
                &mut cn,
                &mut ct,
                &delta,
            ) {
                // self.rects[0].vel = Vec2::zero();
                let vel = self.rects[0].vel;
                // self.rects[0].vel += cn * Vec2::new(vel.x.abs(), vel.y.abs()) * (1. - ct);
                self.rects[0].vel += vel.dot(cn) * (1. - ct);
            }
        }
        let vel = self.rects[0].vel;

        self.rects[0].pos += vel * delta;

        let mut d = rl.begin_drawing(&self.thread);
        d.clear_background(Color::DARKBLUE);

        for r in self.rects.iter() {
            d.draw_rectangle_lines_ex(*r, 1, Color::WHITE);
        }
    }
    fn render(&self, d: &mut raylib::prelude::RaylibDrawHandle) {}
}
