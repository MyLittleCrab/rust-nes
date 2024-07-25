use core::ptr::addr_of_mut;
use nes::{
    addr::Addr,
    apu::{self, Sfx},
    capped_vec::CappedVec,
    io, ppu,
    ppu_buffer::{self, BufferTrait},
    sprites::SpriteState,
    vec2::{DPos, Pos as OtherPos, Vec2},
};

use crate::{
    constants::{DT, GRID_SIZE, PLAYER_SPEED, PLAYER_WIDTH, WIDTH, HEIGHT, ORIGIN},
    level::{draw_level, get_tile_at, make_level, Tile},
    rng::{get_seeds, Rng},
    Buffer,
};

const SEEDS: [u16; GRID_SIZE as usize] = get_seeds();
const LEVEL_TILES: [Tile; GRID_SIZE as usize] = make_level(&SEEDS);

pub unsafe fn init() {
    ppu::write_bytes(ppu::PAL_BG_0, &[0x0E, 0x29, 0x12, 0x19]);
    ppu::write_bytes(ppu::PAL_SPRITE_0 + 1, &[0x0, 0x0, 0x15]);
    ppu::write_bytes(ppu::PAL_SPRITE_1 + 1, &[0x0, 0x0, 0x21]);

    draw_level(&LEVEL_TILES);
}

pub fn frame(game: &mut SnakeGame, apu: &mut apu::APU, sprites: &mut SpriteState) {
    Buffer::clear();
    game.step(apu);
    game.draw(sprites);
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i8,
    y: i8,
}

impl Pos {
    pub fn shifted(&self, vector: &Vec2<i8>) -> Pos{
        Pos {
            x: self.x + vector.x,
            y: self.y + vector.y
        }
    }
}

#[derive(Clone, Copy)]
struct Segment {
    pos: Pos,
}

struct Snake {
    segments: CappedVec<Segment, 64>,
    direction: DPos,
}

impl Clone for Snake {
    fn clone(&self) -> Self {
        let mut new_segments = CappedVec::new();
        for seg in self.segments.iter() {
            new_segments.push(*seg);
        }
        Self {
            segments: new_segments,
            direction: self.direction,
        }
    }
}

pub struct SnakeGame {
    rng: u8,
    snake: Snake,
    food: Pos,
    alive: bool,
}

impl SnakeGame {
    pub fn new(some_game: &mut Option<SnakeGame>) {
        // let mut rng = Rng::new(None);
        let mut segments = CappedVec::<Segment, 64>::new();
        segments.push(Segment { pos: Pos { x: 16, y: 16 } });

        let mut rng: u8 = 42;

        *some_game = Some(Self {
            rng: rng,
            snake: Snake {
                segments,
                direction: DPos::x_unit(),
            },
            food: random_pos(&mut rng),
            alive: true,
        });
    }

    fn step(&mut self, apu: &mut apu::APU) {
        if !self.alive {
            return;
        }

        self.update_direction();
        self.move_snake();
        self.check_collisions(apu);

        if self.snake.segments.read(0).pos == self.food {
            self.grow_snake();
            self.food = random_pos(&mut self.rng);
            apu.play_sfx(Sfx::LevelUp);
        }
    }

    fn update_direction(&mut self) {
        if io::is_pressed(io::Button::Left) && self.snake.direction != DPos::x_unit() {
            self.snake.direction = DPos::x_unit().scaled(-1);
        }
        if io::is_pressed(io::Button::Right) && self.snake.direction != DPos::x_unit().scaled(-1) {
            self.snake.direction = DPos::x_unit();
        }
        if io::is_pressed(io::Button::Up) && self.snake.direction != DPos::y_unit() {
            self.snake.direction = DPos::y_unit().scaled(-1);
        }
        if io::is_pressed(io::Button::Down) && self.snake.direction != DPos::y_unit().scaled(-1) {
            self.snake.direction = DPos::y_unit();
        }
    }

    fn move_snake(&mut self) {
        let new_head = Segment {
            pos: self.snake.segments.read(0).pos.shifted(&self.snake.direction),
        };

        for i in (1..self.snake.segments.len()).rev() {
            let previous_segment = *self.snake.segments.read(i - 1);
            self.snake.segments.write(i, previous_segment);
        }

        self.snake.segments.write(0, new_head);

    }

    fn check_collisions(&mut self, apu: &mut apu::APU) {
        let head = self.snake.segments.read(0).pos;

        if head.x < 0 || head.x >= WIDTH as i8 || head.y < 0 || head.y >= HEIGHT as i8 {
            self.alive = false;
            apu.play_sfx(Sfx::Topout);
            return;
        }

        for segment in self.snake.segments.iter().skip(1) {
            if head == segment.pos {
                self.alive = false;
                apu.play_sfx(Sfx::Topout);
                return;
            }
        }
    }

    fn grow_snake(&mut self) {
        let tail = *self.snake.segments.last().unwrap();
        self.snake.segments.push(tail);
    }

    fn draw(&self, sprites: &mut SpriteState) {
        for segment in self.snake.segments.iter() {
            sprites.add_at_pos(&OtherPos { x: segment.pos.x as u8, y: segment.pos.y as u8 }, 'o' as u8 - 32, 0);
        }
        sprites.add_at_pos(&OtherPos { x: self.food.x as u8, y: self.food.y as u8 }, '*' as u8 - 32, 1);
    }
}


fn simple_rng(seed: &mut u8) -> u8 {
    // Простая реализация генератора псевдослучайных чисел
    *seed = (*seed << 1) | (*seed >> 7); // Сдвиг влево с переносом старшего бита на место младшего
    *seed = seed.wrapping_add(1); // Добавление 1 к значению
    *seed
}

fn random_pos(seed: &mut u8) -> Pos {
    Pos {
        x: (simple_rng(seed) % (WIDTH as u8)) as i8,
        y: (simple_rng(seed) % (HEIGHT as u8)) as i8,
    }
}