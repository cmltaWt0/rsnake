use crate::wasm4;
use crate::palette::set_draw_color;

const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

const LEFT:  Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const UP:    Point = Point { x: 0, y: -1 };
const DOWN:  Point = Point { x: 0, y: 1 };

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Point,
    pub speed: u8,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Point { x: 2, y: 1},
                Point { x: 1, y: 1},
                Point { x: 0, y: 1},
            ],
            direction: Point { x: 1, y: 0 },
            speed: 1,
        }
    }

    pub fn restart(&mut self) {
        self.body = vec![
            Point { x: 2, y: 1},
            Point { x: 1, y: 1},
            Point { x: 0, y: 1},
        ];
        self.direction = Point { x: 1, y: 0 };
        self.speed = 1;
    }

    pub fn is_dead(&self) -> bool {
        self.body
            .iter()
            .skip(1)
            .any(|&body_section| body_section == self.body[0])
    }

    pub fn draw(&self) {
        set_draw_color(0x43);

        for &Point { x, y } in self.body.iter().skip(1) {
            wasm4::rect(x * 8, y * 8, 8, 8);
        }

        set_draw_color(0x4);
        wasm4::blit(&SMILEY, self.body[0].x * 8, self.body[0].y * 8, 8, 8, wasm4::BLIT_1BPP);
    }

    pub fn update(&mut self) -> Option<Point> {
        self.body.insert(
            0,
            Point {
                x: (self.body[0].x + self.direction.x) % 20,
                y: (self.body[0].y + self.direction.y) % 20,
            },
        );

        if self.body[0].x < 0 {
            self.body[0].x = 19;
        }

        if self.body[0].y < 0 {
            self.body[0].y = 19;
        }

        self.body.pop()
    }

    pub fn left(&mut self) {
        if self.direction.x == 0 {
            self.direction = LEFT;
        }
    }

    pub fn right(&mut self) {
        if self.direction.x == 0 {
            self.direction = RIGHT;
        }
    }

    pub fn up(&mut self) {
        if self.direction.y == 0 {
            self.direction = UP;
        }
    }

    pub fn down(&mut self) {
        if self.direction.y == 0 {
            self.direction = DOWN;
        }
    }

    pub fn is_left(&mut self) -> bool {
        self.direction == LEFT
    }

    pub fn is_right(&mut self) -> bool {
        self.direction == RIGHT
    }

    pub fn is_up(&mut self) -> bool {
        self.direction == UP
    }

    pub fn is_down(&mut self) -> bool {
        self.direction == DOWN
    }
}