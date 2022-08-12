use basic_platformer::{KeyMap, Scene, TextureMap};
use raylib::consts::KeyboardKey as Key;
use raylib::prelude::*;

const SWIDTH: f32 = 640.0;
const SHEIGHT: f32 = 480.0;

const GROUND: f32 = (3.0 * SHEIGHT) / 4.0;
const GRAVITY: f32 = 9.8;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SWIDTH as i32, SHEIGHT as i32)
        .title("BAsic Platformer")
        .build();

    let map = rl
        .load_texture(&thread, "assets/tilemap_packed.png")
        .expect("could not load texture map!");
    let map = {
        let width = map.width / 10;
        let height = map.height / 6;
        TextureMap::new(map, width, height)
    };

    let player = Player::new(SWIDTH / 2.0, SHEIGHT / 2.0, &map);
    let mut scene = TestScene {
        player,
        texture_map: map,
    };

    while !rl.window_should_close() {
        /* --- INPUT --- */
        scene.input(&mut rl);

        /* --- UPDATE --- */
        scene.update(&mut rl);

        /* --- DRAW --- */
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GRAY);
        scene.render(&mut d);
    }
}

struct TestScene {
    player: Player,
    texture_map: TextureMap,
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

enum InputMap {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl KeyMap for InputMap {
    fn is_down(&self, rl: &RaylibHandle) -> bool {
        use raylib::consts::KeyboardKey::*;
        match self {
            Self::UP => rl.is_key_down(KEY_UP) || rl.is_key_down(KEY_W),
            Self::DOWN => rl.is_key_down(KEY_DOWN) || rl.is_key_down(KEY_S),
            Self::LEFT => rl.is_key_down(KEY_LEFT) || rl.is_key_down(KEY_A),
            Self::RIGHT => rl.is_key_down(KEY_RIGHT) || rl.is_key_down(KEY_D),
        }
    }

    fn is_up(&self, rl: &RaylibHandle) -> bool {
        use raylib::consts::KeyboardKey::*;
        match self {
            Self::UP => rl.is_key_up(KEY_UP) || rl.is_key_up(KEY_W),
            Self::DOWN => rl.is_key_up(KEY_DOWN) || rl.is_key_up(KEY_S),
            Self::LEFT => rl.is_key_up(KEY_LEFT) || rl.is_key_up(KEY_A),
            Self::RIGHT => rl.is_key_up(KEY_RIGHT) || rl.is_key_up(KEY_D),
        }
    }

    fn is_pressed(&self, rl: &RaylibHandle) -> bool {
        use raylib::consts::KeyboardKey::*;
        match self {
            Self::UP => rl.is_key_pressed(KEY_UP) || rl.is_key_pressed(KEY_W),
            Self::DOWN => rl.is_key_pressed(KEY_DOWN) || rl.is_key_pressed(KEY_S),
            Self::LEFT => rl.is_key_pressed(KEY_LEFT) || rl.is_key_pressed(KEY_A),
            Self::RIGHT => rl.is_key_pressed(KEY_RIGHT) || rl.is_key_pressed(KEY_D),
        }
    }

    fn is_released(&self, rl: &RaylibHandle) -> bool {
        use raylib::consts::KeyboardKey::*;
        match self {
            Self::UP => rl.is_key_released(KEY_UP) || rl.is_key_released(KEY_W),
            Self::DOWN => rl.is_key_released(KEY_DOWN) || rl.is_key_released(KEY_S),
            Self::LEFT => rl.is_key_released(KEY_LEFT) || rl.is_key_released(KEY_A),
            Self::RIGHT => rl.is_key_released(KEY_RIGHT) || rl.is_key_released(KEY_D),
        }
    }
}

const SPEED: f32 = 500.0;
const JUMP_POWER: f32 = 800.0;
const JUMP_MULTI: f32 = 5.0;

struct Player {
    pos: Vector2,
    vel: Vector2,
    rec: Rectangle,
}

impl Player {
    fn new(x: f32, y: f32, map: &TextureMap) -> Self {
        Self {
            pos: rvec2(x, y),
            vel: Vector2::default(),
            rec: Rectangle::new(0.0, 0.0, map.tile_width() as f32, map.tile_height() as f32),
        }
    }

    fn input(&mut self, rl: &mut RaylibHandle) {
        let on_ground = self.is_on_ground();

        if InputMap::LEFT.is_down(rl) {
            self.vel.x = -SPEED;
        }
        if InputMap::RIGHT.is_down(rl) {
            self.vel.x = SPEED;
        }

        if InputMap::UP.is_pressed(rl) && on_ground {
            self.vel.y = -JUMP_POWER;
        }
        if InputMap::UP.is_down(rl) && !on_ground && self.vel.y < 0.0 {
            self.vel.y -= JUMP_MULTI;
        }
    }

    fn update(&mut self, delta: f32, rl: &mut RaylibHandle) {
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
        if self.pos.y + self.rec.height >= GROUND {
            true
        } else {
            false
        }
    }

    fn render(&self, map: &TextureMap, d: &mut RaylibDrawHandle) {
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
