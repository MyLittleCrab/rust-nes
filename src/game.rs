use core::ptr::addr_of_mut;

use crate::{
    apu::{self, Sfx},
    io, ppu,
    sprites::{self, SpriteState},
    utils::{debug_value, inc_u8, Addr},
};

// statically allocated memory
static mut STATE: Option<Game> = None;

static mut SEED: u16 = 0x8988;

const ROW: u8 = 0x20;
const N_ROWS: u8 = 24;
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
        Game::new(&mut *addr_of_mut!(STATE));
    }
    let game = state();

    // palettes
    ppu::write_bytes(ppu::PAL_BG_0, &[0x0E, 0x30, 0x12, 0x26]);
    ppu::write_bytes(ppu::PAL_SPRITE_0 + 3, &[0x15]);

    // draw tiles
    ppu::write_addr(ORIGIN);

    for (i, tile) in game.tiles.iter().enumerate() {
        ppu::write_addr(ORIGIN + (i as u16));
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

    sprites.add(
        TOP_MARGIN + game.player.x,
        LEFT_MARGIN + game.player.y - 1,
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

    if let Some(index) = game.grabbed_coin_index {
        ppu::write_addr(ORIGIN + index);
        ppu::write_data(HEART);
        game.grabbed_coin_index = None;
    }

    // let digits = io::u16_to_digits(index as u16);
    // ppu::write_addr(ORIGIN);
    // for x in digits {
    //     ppu::write_data(io::digit_to_ascii(x) - 32);
    // }
    ppu::write_addr(ORIGIN);

    let digits = io::byte_to_digits(game.n_coins);
    ppu::write_data(io::digit_to_ascii(digits[1]) - 32);
    ppu::write_data(io::digit_to_ascii(digits[0]) - 32);

    // print heart location
    //ppu::write_addr(ORIGIN);
    // for x in io::byte_to_digits(game.paddle.x) {
    //     ppu::write_data(io::digit_to_ascii(x) - 32);
    // }
    // ppu::write_addr(ORIGIN + 3);
    // for x in io::byte_to_digits(game.paddle.y) {
    //     ppu::write_data(io::digit_to_ascii(x) - 32);
    // }
}

// game logic
#[inline(never)]
fn cycle_rng() {
    unsafe {
        let new_bit = ((SEED >> 9) ^ (SEED >> 1)) & 1;
        SEED = (new_bit << 15) | (SEED >> 1);
    }
}

fn get_rng() -> u8 {
    unsafe { (SEED >> 8) as u8 }
}

struct Player {
    x: u8,
    y: u8,
}

pub struct Game {
    player: Player,
    tiles: [Tile; GRID_SIZE as usize],
    grabbed_coin_index: Option<u16>,
    n_coins: u8,
}

impl Game {
    pub fn new(some_game: &mut Option<Game>) {
        *some_game = Some(Self {
            player: Player {
                x: WIDTH / 2,
                y: HEIGHT - 10,
            },
            tiles: [Tile::Nothing; GRID_SIZE as usize],
            grabbed_coin_index: None,
            n_coins: 0,
        });
        let game = some_game.as_mut().unwrap();

        for i in 0..GRID_SIZE {
            cycle_rng();
            if get_rng() % 4 == 0 {
                game.tiles[i as usize] = Tile::Wall;
            } else if get_rng() % 41 == 1 {
                game.tiles[i as usize] = Tile::Coin;
            }
        }

        for i in 0..ROW {
            game.tiles[i as usize] = Tile::Wall;
        }
        for i in 0..(N_ROWS as u16) {
            game.tiles[(i as usize) * (ROW as usize) as usize] = Tile::Wall;
            game.tiles[(i as usize + 1) * (ROW as usize) - 1] = Tile::Wall;
        }
        game.n_coins = game
            .tiles
            .iter()
            .map(|t| match t {
                Tile::Coin => 1,
                _ => 0,
            })
            .sum();
    }

    fn step(&mut self, apu: &mut apu::APU) {
        let buttons = io::controller_buttons();

        let mut delta_x: i8 = 0;
        let mut delta_y: i8 = 0;
        //if self.paddle.x

        if buttons & io::LEFT != 0 && self.player.x > 0 {
            delta_x = -2;
        } else if buttons & io::RIGHT != 0 && self.player.x + 8 < 0xe8 {
            delta_x = 2;
        }
        if buttons & io::UP != 0 && self.player.y > 0 {
            delta_y = -2;
        } else if buttons & io::DOWN != 0 && self.player.y + 8 < 0xe8 {
            delta_y = 2;
        }
        for (dx, dy) in [
            (0, 0),
            (0, PLAYER_WIDTH),
            (PLAYER_WIDTH, 0),
            (PLAYER_WIDTH, PLAYER_WIDTH),
        ] {
            if let Tile::Wall = get_tile_at(
                &self.tiles,
                inc_u8(self.player.x + dx, delta_x),
                self.player.y + dy,
            ) {
                delta_x = 0;
                if !apu.is_playing() {
                    apu.play_sfx(Sfx::Lock);
                }
            }
            if let Tile::Wall = get_tile_at(
                &self.tiles,
                self.player.x + dx,
                inc_u8(self.player.y + dy, delta_y),
            ) {
                delta_y = 0;
                if !apu.is_playing() {
                    apu.play_sfx(Sfx::Lock);
                }
            }
        }
        if let Tile::Coin = get_tile_at(&self.tiles, self.player.x + 4, self.player.y + 4) {
            let index = map_pos_to_tile_index(self.player.x + 4, self.player.y + 4);
            self.tiles[index as usize] = Tile::Nothing;
            self.grabbed_coin_index = Some(index);
            self.n_coins -= 1;
            apu.play_sfx(Sfx::LevelUp);
        }

        self.player.x = inc_u8(self.player.x, delta_x);
        self.player.y = inc_u8(self.player.y, delta_y);
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
