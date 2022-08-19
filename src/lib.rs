use raylib::prelude::*;

pub trait KeyMap {
    fn is_pressed(&self, rl: &RaylibHandle) -> bool;
    fn is_released(&self, rl: &RaylibHandle) -> bool;
    fn is_down(&self, rl: &RaylibHandle) -> bool;
    fn is_up(&self, rl: &RaylibHandle) -> bool;
}

pub trait Scene {
    fn input(&mut self, rl: &mut RaylibHandle);
    fn update(&mut self, rl: &mut RaylibHandle);
    fn render(&self, d: &mut RaylibDrawHandle);
}
