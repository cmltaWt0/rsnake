use crate::wasm4;
use crate::palette::set_draw_color;
use crate::snake::Point;
use fastrand::Rng;

const HEART_FULL: [u8; 8] = [
    0b00000000,
    0b01101100,
    0b11111110,
    0b11111110,
    0b11111110,
    0b01111100,
    0b00111000,
    0b00000000,
];
#[allow(dead_code)]
const HEART_EMPTY: [u8; 8] = [
    0b00000000,
    0b01100110,
    0b01011010,
    0b10000001,
    0b10000001,
    0b01000010,
    0b00100010,
    0b00011000,
];
const FRUIT_SPRITE: [u8; 16] = [
    0x00,
    0xa0,
    0x02,
    0x00,
    0x0e,
    0xf0,
    0x36,
    0x5c,
    0xd6,
    0x57,
    0xd5,
    0x57,
    0x35,
    0x5c,
    0x0f,
    0xf0,
];

pub struct Life {
    pub lifes: Vec<Point>,
    pub lifes_count: u8,
}

impl Life {
    pub fn new() -> Self {
        Self {
            lifes: vec![
                Point { x: 17, y: 0},
                Point { x: 18, y: 0},
                Point { x: 19, y: 0},
            ],
            lifes_count: 3,
        }
    }
    #[allow(dead_code)]
    pub fn restart(&mut self) {
        self.lifes_count = 3;
        self.lifes = vec![
            Point { x: 17, y: 0},
            Point { x: 18, y: 0},
            Point { x: 19, y: 0},
        ];
    }

    pub fn draw(&self) {
        set_draw_color(0x40);

        for &Point { x, y } in self.lifes.iter() {
            wasm4::blit(
                &HEART_FULL,
                x * 8,
                y * 8,
                8,
                8,
                wasm4::BLIT_1BPP
            );
        }
    }

    pub fn lose_life(&mut self) {
        self.lifes_count -= 1;
        self.lifes.pop();
    }
}

pub struct Score {
    pub score: u32,
}

impl Score {    
    pub fn new() -> Self {
        Self {
            score: 0,
        }
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    #[allow(dead_code)]
    pub fn restart(&mut self) {
        self.score = 0;
    }

    pub fn draw(&self) {
        set_draw_color(0x0002);
        wasm4::text("Score:", 1, 1);
        set_draw_color(0x0002);
        wasm4::text(&self.score.to_string(), 6*8, 1);
    }
}

pub struct Fruit {
    pub fruit: Point,
    rng: Rng,
}

impl Fruit {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);

        Self {
            fruit: Point { x: rng.i32(0..20), y: rng.i32(1..20)},
            rng,
        }
    }

    pub fn shuffle(&mut self) {
        self.fruit.x = self.rng.i32(0..20);
        self.fruit.y = self.rng.i32(1..20);
    }

    pub fn draw(&self) {
        set_draw_color(0x4320);
        wasm4::blit(&FRUIT_SPRITE, self.fruit.x * 8, self.fruit.y * 8, 8, 8, wasm4::BLIT_2BPP);
    }
}
