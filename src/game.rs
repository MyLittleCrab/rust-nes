use crate::{
    apu, io, ppu,
    sprites::{self, SpriteState},
};

// statically allocated memory
static mut STATE: Option<Game> = None;
static mut SEED: u16 = 0x8988;

/// do not call this more than once in the same scope (!)
fn state() -> &'static mut Game {
    unsafe { STATE.as_mut().unwrap() }
}

enum Tile {
    Nothing,
    Wall
}
const w: Tile = Tile::Wall;
const n: Tile = Tile::Nothing;

const walls: [Tile ; 0x180] = [
    w, w, w, w, w, w, w, w, w, w, w, w, w, w, w, w,
    w, w, w, w, w, w, w, w, w, w, w, w, w, w, w, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, w, w, w, w, w, w, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,

    w, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n,
    n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, w,
];

const ORIGIN: u16 = 0x2020;
const WALL_SPRITE: u8 = 0x60;
const AT_SPRITE: u8 = 0x59;

pub fn init() {
    unsafe {
        STATE = Some(Game::new());
    }

    // palettes and border
    ppu::write_bytes(ppu::PAL_BG_0, &[0x0E, 0x30, 0x12, 0x26]);
    ppu::write_bytes(ppu::PAL_SPRITE_0 + 3, &[0x15]);
    //ppu::draw_box(1, 1, 30, 28);

    ppu::write_addr(ORIGIN);
    let row: u16 = 0x20;
    for tile in walls {
        //ppu::write_addr(origin + row * i);
        match tile {
            Tile::Nothing => ppu::write_data(0x00),
            Tile::Wall => ppu::write_data(0x60)
        }
    }
    
    // text
    ppu::draw_ascii(0x20a3, "HEART-MAN");
}

const HEART: u8 = 0x63;

pub fn frame(apu: &mut apu::APU, sprites: &mut SpriteState) {
    let game = state();
    game.step(apu);

    sprites.add(
        TOP_MARGIN + game.ball.x,
        LEFT_MARGIN + game.ball.y - 1,
        0x80,
        0,
    );
    for i in 0..game.paddle.width {
        sprites.add(
            TOP_MARGIN + game.paddle.x + (i * 8),
            LEFT_MARGIN + game.paddle.y - 1,
            HEART,
            0,
        );
    }
}

pub fn render() {
    let game = state();
    //ppu::draw_ascii(0x20a3, "HEART-MEN");
    // place tile at heart pos
    let index = map_pos_to_tile_index(game.paddle.x, game.paddle.y);
    ppu::write_addr(ORIGIN + index);
    ppu::write_data(HEART);
    // let digits = io::u16_to_digits(index as u16);
    // ppu::write_addr(ORIGIN);
    // for x in digits {
    //     ppu::write_data(io::digit_to_ascii(x) - 32);
    // }
    ppu::write_addr(ORIGIN);
    for x in io::byte_to_digits(game.paddle.x) {
        ppu::write_data(io::digit_to_ascii(x) - 32);
    }
    ppu::write_addr(ORIGIN + 3);
    for x in io::byte_to_digits(game.paddle.y) {
        ppu::write_data(io::digit_to_ascii(x) - 32);
    }
}

// game logic

fn cycle_rng() {
    unsafe {
        let new_bit = ((SEED >> 9) ^ (SEED >> 1)) & 1;
        SEED = (new_bit << 15) | (SEED >> 1);
    }
}

fn get_rng() -> u8 {
    unsafe { (SEED >> 8) as u8 }
}

const WIDTH: u8 = 224;
const HEIGHT: u8 = 208;
const BRICKS_WIDE: usize = 14;
const BRICK_WIDTH: u8 = 16;
const BRICK_HEIGHT: u8 = 8;
const TOP_BRICK_MARGIN: usize = 2;
const BALL_DIAMETER: u8 = 6;
const BALL_RADIUS: u8 = BALL_DIAMETER / 2;
const LEFT_MARGIN: u8 = 16;
const TOP_MARGIN: u8 = 16;

struct Ball {
    x: u8,
    y: u8,
    dx: i8,
    dy: i8,
}
struct Paddle {
    x: u8,
    y: u8,
    width: u8,
}


struct Game {
    paddle: Paddle,
    ball: Ball,
}


impl Game {
    fn new() -> Self {
        let game = Self {
            ball: Ball {
                x: 0,
                y: HEIGHT / 2,
                dx: 2,
                dy: -1,
            },
            paddle: Paddle {
                x: WIDTH / 2,
                y: HEIGHT - 10,
                width: 1,
            },
        };

        game
    }

    fn step(&mut self, apu: &mut apu::APU) {
        let buttons = io::controller_buttons();

        let delta_x: i8 = 0
        let delta_y: i8 = 0
        if self.paddle.x 

        if buttons & io::LEFT != 0 && self.paddle.x > 0 {
            self.paddle.x -= 2;
        } else if buttons & io::RIGHT != 0 && self.paddle.x + self.paddle.width * 8 < 0xe8 {
            self.paddle.x += 2;
        }
        if buttons & io::UP != 0 && self.paddle.y > 0 {
            self.paddle.y -= 2;
        } else if buttons & io::DOWN != 0 && self.paddle.y + self.paddle.width * 8 < 0xe8 {
            self.paddle.y += 2;
        }

        // collision
        self.ball.x = (self.ball.x as i8 + self.ball.dx) as u8;
        self.ball.y = (self.ball.y as i8 + self.ball.dy) as u8;

        // Screen collision
        if self.ball.x == 0 || self.ball.x + BALL_DIAMETER >= WIDTH {
            self.ball.dx = -self.ball.dx;
            apu.play_sfx(apu::Sfx::Lock);
        }
        if self.ball.y == 0 || self.ball.y + BALL_DIAMETER >= HEIGHT {
            self.ball.dy = -self.ball.dy;
            apu.play_sfx(apu::Sfx::Lock);
        }
        // paddle collision
        let y_delta = self.ball.y as i8 - self.paddle.y as i8;
        let x_delta = self.ball.x as i8 - self.paddle.x as i8;
        if (y_delta.abs() as u8) < self.paddle.width * 8 + BALL_DIAMETER &&
        (x_delta.abs() as u8) < self.paddle.width * 8 + BALL_DIAMETER {
            self.ball.dy = self.ball.dy.abs() * y_delta.signum();
            self.ball.dx = self.ball.dx.abs() * x_delta.signum();
            apu.play_sfx(apu::Sfx::Lock);
        }
        
    }
}

const ROW: u8 = 0x20;

fn map_pos_to_tile_index(x: u8, y: u8) -> u16 {
    return (x as u16) / 8 + (y as u16 / 8) * ( ROW as u16)
}
fn map_pos_to_sprite_index(x: u8, y: u8) -> u16 {
    return (x as u16) / 8 + (y as u16 / 8) * (ROW as u16) + 2 + (ROW as u16) * 2
}