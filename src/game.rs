use crate::snake::Snake;
use crate::board::{ Life, Score, Fruit };
use crate::palette::set_draw_color;
use crate::wasm4;

pub struct Game {
    game_over: bool,
    life: Life,
    score: Score,
    snake: Snake,
    frame_count: u32,
    refresh_rate: u8,
    prev_gamepad: u8,
    fruit: Fruit,
    lock_input: bool,
}

impl Game {
    pub fn new() -> Self {

        Self {
            game_over: false,
            life: Life::new(),
            score: Score::new(),
            snake: Snake::new(),
            frame_count: 0,
            refresh_rate: 15,
            prev_gamepad: 0,
            fruit: Fruit::new(),
            lock_input: false,
        }
    }

    fn restart(&mut self) {
        self.game_over = false;
        self.life.restart();
        self.score.restart();
        self.snake.restart();
        self.frame_count = 0;
        self.refresh_rate = 15;
        self.prev_gamepad = 0;
        self.lock_input = false;
        self.fruit.shuffle();
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        self.input();

        if self.game_over {
            set_draw_color(0x0002);
            wasm4::text("Press X to restart", 1 * 8, 10 * 8);
            if unsafe { *wasm4::GAMEPAD1 } & wasm4::BUTTON_1 != 0 {
                self.restart();
            }
        }
        
        // Updating snake's position each 15 frames by default
        // and each 5 frames when speed is 3
        if self.frame_count % ( self.refresh_rate as u32 / self.snake.speed as u32)  == 0 && !self.game_over {

            let droped_pos = self.snake.update();

            self.lock_input = false;

            if self.frame_count % 180 == 0 {
                self.snake.speed = 1;
            }

            if self.snake.bitten() {
                self.life.lose_life();
                if self.life.lifes_count == 0 {
                    self.game_over = true;
                }

            }
            if self.snake.body[0] == self.fruit.fruit {
                if let Some(last_pos) = droped_pos {
                    self.snake.body.push(last_pos);
                    self.score.add_score();
                }

                // Prevent fruit to spawn on snake's body
                loop {
                    self.fruit.shuffle();
                    if !self.snake.body.contains(&self.fruit.fruit) {
                        break;
                    }
                }
            }
        }

        self.snake.draw();
        self.life.draw();
        self.score.draw();
        self.fruit.draw();

    }

    pub fn input(&mut self) {
        if self.lock_input {
            return;
        }
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            self.snake.left();    
            self.lock_input = true;
        }
        
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            self.snake.right();
            self.lock_input = true;
        }
        
        if just_pressed & wasm4::BUTTON_UP != 0 {
            self.snake.up();
            self.lock_input = true;
        }
        
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            self.snake.down();
            self.lock_input = true;
        }

        self.prev_gamepad = gamepad;
    }
}
