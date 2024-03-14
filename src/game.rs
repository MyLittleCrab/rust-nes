use crate::{
    apu::{self, Sfx},
    io, ppu,
    sprites::{self, SpriteState},
    utils::inc_u8,
};

// statically allocated memory
static mut STATE: Option<Game> = None;
static mut SEED: u16 = 0x8988;

const ROW: u8 = 0x20;
const N_ROWS: u8 = 20;
const GRID_SIZE: u16 = (ROW as u16) * (N_ROWS as u16);
const PLAYER_WIDTH: u8 = 6;

const ORIGIN: u16 = 0x2020;

const HEART: u8 = 0x63;
const WALL_SPRITE: u8 = 0x60;
const AT_SPRITE: u8 = 0x59;
const COIN_SPRITE: u8 = HEART + 7;

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

/// do not call this more than once in the same scope (!)
fn state() -> &'static mut Game {
    unsafe { STATE.as_mut().unwrap() }
}

#[derive(Copy, Clone)]
enum Tile {
    Nothing,
    Wall,
    Coin,
}

pub fn init() {
    unsafe {
        STATE = Some(Game::new());
    }
    let game = state();

    // palettes and border
    ppu::write_bytes(ppu::PAL_BG_0, &[0x0E, 0x30, 0x12, 0x26]);
    ppu::write_bytes(ppu::PAL_SPRITE_0 + 3, &[0x15]);
    //ppu::draw_box(1, 1, 30, 28);

    ppu::write_addr(ORIGIN);

    for (i, tile) in game.walls.iter().enumerate() {
        ppu::write_addr(ORIGIN + (i as u16));
        //ppu::write_addr(origin + row * i);
        match tile {
            Tile::Nothing => ppu::write_data(0x00),
            Tile::Wall => ppu::write_data(WALL_SPRITE),
            Tile::Coin => ppu::write_data(COIN_SPRITE),
        }
    }

    // text
    ppu::draw_ascii(ORIGIN + 0x06, "HEART-MAN");
}

pub fn frame(apu: &mut apu::APU, sprites: &mut SpriteState) {
    let game = state();
    game.step(apu);

    // sprites.add(
    //     TOP_MARGIN + game.ball.x,
    //     LEFT_MARGIN + game.ball.y - 1,
    //     0x80,
    //     0,
    // );
    sprites.add(
        TOP_MARGIN + game.paddle.x,
        LEFT_MARGIN + game.paddle.y - 1,
        HEART,
        0,
    );
}

fn get_tile_at<const N: usize>(tiles: &[Tile; N], x: u8, y: u8) -> Tile {
    let index = map_pos_to_tile_index(x, y);
    if index < tiles.len() as u16 {
        tiles[index as usize]
    } else {
        Tile::Nothing
    }
}

pub fn render() {
    let game = state();

    if let MyOption::Some(index) = game.grabbed_coin_index {
        ppu::write_addr(ORIGIN + index);
        ppu::write_data(HEART);
        game.grabbed_coin_index = MyOption::None(0);
    }

    //ppu::draw_ascii(0x20a3, "HEART-MEN");
    // place tile at heart pos
    //let index = map_pos_to_tile_index(game.paddle.x, game.paddle.y);

    // if index < game.walls.len() as u16 {
    //     match &game.walls[index as usize] {
    //         Tile::Wall => {
    //             let draw_index = map_pos_to_sprite_index(game.paddle.x, game.paddle.y);
    //             ppu::write_addr(ORIGIN + draw_index);
    //             ppu::write_data(HEART);
    //         }
    //         _ => ()
    //     }
    // }

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

enum MyOption {
    Some(u16),
    None(u16),
}

struct Game {
    paddle: Paddle,
    walls: [Tile; GRID_SIZE as usize],
    grabbed_coin_index: MyOption,
}

impl Game {
    fn new() -> Self {
        let mut walls = [Tile::Nothing; GRID_SIZE as usize];
        for i in 0..GRID_SIZE {
            cycle_rng();
            if get_rng() % 4 == 0 {
                walls[i as usize] = Tile::Wall;
            } else if get_rng() % 41 == 1 {
                walls[i as usize] = Tile::Coin;
            }
        }
        for i in 0..ROW {
            walls[i as usize] = Tile::Wall;
        }
        for i in 0..(N_ROWS as u16) {
            walls[(i as usize) * (ROW as usize) as usize] = Tile::Wall;
            walls[(i as usize + 1) * (ROW as usize) - 1] = Tile::Wall;
        }
        let game = Self {
            paddle: Paddle {
                x: WIDTH / 2,
                y: HEIGHT - 10,
                width: 1,
            },
            walls,
            grabbed_coin_index: MyOption::None(0),
        };

        game
    }

    fn step(&mut self, apu: &mut apu::APU) {
        let buttons = io::controller_buttons();

        let mut delta_x: i8 = 0;
        let mut delta_y: i8 = 0;
        //if self.paddle.x

        if buttons & io::LEFT != 0 && self.paddle.x > 0 {
            delta_x = -2;
        } else if buttons & io::RIGHT != 0 && self.paddle.x + 8 < 0xe8 {
            delta_x = 2;
        }
        if buttons & io::UP != 0 && self.paddle.y > 0 {
            delta_y = -2;
        } else if buttons & io::DOWN != 0 && self.paddle.y + 8 < 0xe8 {
            delta_y = 2;
        }
        for (dx, dy) in [
            (0, 0),
            (0, PLAYER_WIDTH),
            (PLAYER_WIDTH, 0),
            (PLAYER_WIDTH, PLAYER_WIDTH),
        ] {
            if let Tile::Wall = get_tile_at(
                &self.walls,
                inc_u8(self.paddle.x + dx, delta_x),
                self.paddle.y + dy,
            ) {
                delta_x = 0;
                apu.play_sfx(Sfx::Lock);
            }
            if let Tile::Wall = get_tile_at(
                &self.walls,
                self.paddle.x + dx,
                inc_u8(self.paddle.y + dy, delta_y),
            ) {
                delta_y = 0;
                apu.play_sfx(Sfx::Lock);
            }
        }
        if let Tile::Coin = get_tile_at(&self.walls, self.paddle.x + 4, self.paddle.y + 4) {
            let index = map_pos_to_tile_index(self.paddle.x + 4, self.paddle.y + 4);
            self.walls[index as usize] = Tile::Nothing;
            self.grabbed_coin_index = MyOption::Some(index);
            apu.play_sfx(Sfx::LevelUp);
        }

        self.paddle.x = inc_u8(self.paddle.x, delta_x);
        self.paddle.y = inc_u8(self.paddle.y, delta_y);
    }
}

#[inline(never)]
fn map_pos_to_tile_index(x: u8, y: u8) -> u16 {
    let x_shift = x - 0;
    let y_shift = y - 0;
    let x_tile = x_shift / 8 + 2;
    let y_tile = y_shift / 8 + 1;
    return (x_tile as u16) + (y_tile as u16) * (ROW as u16);
}

#[inline(never)]
fn map_pos_to_sprite_index(x: u8, y: u8) -> u16 {
    return (x as u16) / 8 + (y as u16 / 8) * (ROW as u16) + 2 + (ROW as u16) * 2;
}
