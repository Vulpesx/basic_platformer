use crate::consts::*;
use basic_platformer::{resources::TextureMap, tile_map::TileMap, KeyMap, Scene};
use raylib::prelude::*;

const GROUND: f32 = (3.0 * SHEIGHT) / 4.0;
const GRAVITY: f32 = 9.8;

const SPEED: f32 = 500.0;
const JUMP_POWER: f32 = 800.0;
const JUMP_MULTI: f32 = 5.0;

pub struct Player {
    pos: Vector2,
    vel: Vector2,
    rec: Rectangle,
}

pub enum InputMap {
    Up,
    Down,
    Left,
    Right,
}

pub struct TestScene {
    player: Player,
    texture_map: TextureMap,
    tile_map: TileMap,
}

impl TestScene {
    pub fn new(player: Player, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            player,
            texture_map: TextureMap::new(
                rl.load_texture(thread, "assets/tilemap_packed.png")
                    .expect("could not load texture"),
                16,
                16,
            ),
            tile_map: TileMap::new("assets/test_level.tmj", rl, thread),
        }
    }
}

impl Scene for TestScene {
    fn input(&mut self, rl: &mut RaylibHandle) {
        self.player.input(rl);
    }

    fn update(&mut self, rl: &mut RaylibHandle) {
        self.player.update(rl.get_frame_time(), rl);
    }

    fn render(&self, d: &mut RaylibDrawHandle) {
        self.player.render(&self.texture_map, d);
        d.draw_line(0, GROUND as i32, SWIDTH as i32, GROUND as i32, Color::GREEN);
        d.draw_text("WIP", 0, 0, 20, Color::SKYBLUE);
    }
}

impl Player {
    pub fn new(x: f32, y: f32, map: &TextureMap) -> Self {
        Self {
            pos: rvec2(x, y),
            vel: Vector2::default(),
            rec: Rectangle::new(0.0, 0.0, map.tile_width() as f32, map.tile_height() as f32),
        }
    }

    pub fn input(&mut self, rl: &mut RaylibHandle) {
        let on_ground = self.is_on_ground();

        if InputMap::Left.is_down(rl) {
            self.vel.x = -SPEED;
        }
        if InputMap::Right.is_down(rl) {
            self.vel.x = SPEED;
        }

        if InputMap::Up.is_pressed(rl) && on_ground {
            self.vel.y = -JUMP_POWER;
        }
        if InputMap::Up.is_down(rl) && !on_ground && self.vel.y < 0.0 {
            self.vel.y -= JUMP_MULTI;
        }
    }

    pub fn update(&mut self, delta: f32, rl: &mut RaylibHandle) {
        let on_ground = self.is_on_ground();

        if on_ground {
            self.pos.y = GROUND - self.rec.height;
        }

        if !on_ground {
            self.vel.y += GRAVITY;
        } else {
            self.vel.y = self.vel.y.clamp(-JUMP_POWER, 0.0);
        }

        self.vel.x *= 0.98;
        self.vel.x = self.vel.x.clamp(-SPEED, SPEED);
        self.pos += self.vel * delta;
    }

    fn is_on_ground(&self) -> bool {
        self.pos.y + self.rec.height >= GROUND
    }

    pub fn render(&self, map: &TextureMap, d: &mut RaylibDrawHandle) {
        d.draw_texture_rec(map.texture(), map.tile(0, 4), self.pos, Color::WHITE);
        d.draw_rectangle_lines(
            self.pos.x as i32,
            self.pos.y as i32,
            map.tile_width(),
            map.tile_height(),
            Color::GREEN,
        );
    }
}

impl KeyMap for InputMap {
    fn is_down(&self, rl: &RaylibHandle) -> bool {
        use raylib::consts::KeyboardKey::*;
        match self {
            Self::Up => rl.is_key_down(KEY_UP) || rl.is_key_down(KEY_W),
            Self::Down => rl.is_key_down(KEY_DOWN) || rl.is_key_down(KEY_S),
            Self::Left => rl.is_key_down(KEY_LEFT) || rl.is_key_down(KEY_A),
            Self::Right => rl.is_key_down(KEY_RIGHT) || rl.is_key_down(KEY_D),
        }
    }

    fn is_up(&self, rl: &RaylibHandle) -> bool {
        use raylib::consts::KeyboardKey::*;
        match self {
            Self::Up => rl.is_key_up(KEY_UP) || rl.is_key_up(KEY_W),
            Self::Down => rl.is_key_up(KEY_DOWN) || rl.is_key_up(KEY_S),
            Self::Left => rl.is_key_up(KEY_LEFT) || rl.is_key_up(KEY_A),
            Self::Right => rl.is_key_up(KEY_RIGHT) || rl.is_key_up(KEY_D),
        }
    }

    fn is_pressed(&self, rl: &RaylibHandle) -> bool {
        use raylib::consts::KeyboardKey::*;
        match self {
            Self::Up => rl.is_key_pressed(KEY_UP) || rl.is_key_pressed(KEY_W),
            Self::Down => rl.is_key_pressed(KEY_DOWN) || rl.is_key_pressed(KEY_S),
            Self::Left => rl.is_key_pressed(KEY_LEFT) || rl.is_key_pressed(KEY_A),
            Self::Right => rl.is_key_pressed(KEY_RIGHT) || rl.is_key_pressed(KEY_D),
        }
    }

    fn is_released(&self, rl: &RaylibHandle) -> bool {
        use raylib::consts::KeyboardKey::*;
        match self {
            Self::Up => rl.is_key_released(KEY_UP) || rl.is_key_released(KEY_W),
            Self::Down => rl.is_key_released(KEY_DOWN) || rl.is_key_released(KEY_S),
            Self::Left => rl.is_key_released(KEY_LEFT) || rl.is_key_released(KEY_A),
            Self::Right => rl.is_key_released(KEY_RIGHT) || rl.is_key_released(KEY_D),
        }
    }
}
