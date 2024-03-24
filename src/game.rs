use core::{mem::transmute, ptr::addr_of_mut};

use alloc::vec;
use alloc::vec::Vec;

use crate::{
    apu::{self, Sfx},
    constants::{AT_SPRITE, DT, GRID_SIZE, HEART_SPRITE, HEIGHT, ORIGIN, PLAYER_WIDTH, ROW, WIDTH},
    io,
    level::{draw_level, get_tile_at, make_level, map_pos_to_tile_index, Tile},
    ppu,
    rng::{cycle_rng, get_rng},
    sprites::{self, SpriteState},
    utils::{debug_value, inc_u8, Addr, DPos, Orientation, Pos, Sign, Vec2},
};

// statically allocated memory
static mut STATE: Option<Game> = None;

/// do not call this more than once in the same scope (!)
fn state() -> &'static mut Game {
    unsafe { STATE.as_mut().unwrap() }
}

pub fn init() {
    unsafe {
        Game::new(&mut *addr_of_mut!(STATE));
    }
    let game = state();

    // palettes
    ppu::write_bytes(ppu::PAL_BG_0, &[0x0E, 0x30, 0x12, 0x26]);
    ppu::write_bytes(ppu::PAL_SPRITE_0 + 3, &[0x15]);

    draw_level(&mut game.tiles);

    // text
    ppu::draw_ascii(ORIGIN + 0x06, "HEART-MAN");
}

pub fn frame(apu: &mut apu::APU, sprites: &mut SpriteState) {
    let game = state();
    game.step(apu);

    sprites.add(&game.player.pos, HEART_SPRITE, 0);

    for meanie in game.meanies.iter() {
        sprites.add(&meanie.pos, AT_SPRITE, 0)
    }
}

pub fn render() {
    let game = state();

    if let Some(index) = game.grabbed_coin_index {
        ppu::write_addr(ORIGIN + index);
        ppu::write_data(HEART_SPRITE);
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

struct Player {
    pos: Pos,
}

//const N_MEANIES: usize = 1;

struct Meanie {
    pos: Pos,
    vel: DPos,
    orientation: Orientation,
    n_turns: u8,
}

type Collision = Vec2<Option<Sign>>;

pub struct Game {
    player: Player,
    tiles: [Tile; GRID_SIZE as usize],
    grabbed_coin_index: Option<u16>,
    n_coins: u8,
    meanies: Vec<Meanie>,
}

impl Game {
    pub fn new(some_game: &mut Option<Game>) {
        *some_game = Some(Self {
            player: Player {
                pos: Pos {
                    x: WIDTH / 2,
                    y: HEIGHT - 10,
                },
            },
            tiles: [Tile::Nothing; GRID_SIZE as usize],
            grabbed_coin_index: None,
            n_coins: 0,
            meanies: vec![
                Meanie {
                    pos: Pos {
                        x: WIDTH / 2 + 16,
                        y: HEIGHT - 20,
                    },
                    vel: DPos::new(-1, 0),
                    orientation: Orientation::Widdershins,
                    n_turns: 0,
                },
                // Meanie {
                //     pos: Pos {
                //         x: WIDTH / 3,
                //         y: HEIGHT - 20,
                //     },
                //     vel: DPos::new(0, 1),
                //     orientation: Orientation::Widdershins,
                //     n_turns: 0,
                // },
            ],
        });
        let game = some_game.as_mut().unwrap();
        make_level(&mut game.tiles);

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
        let mut player_delta = player_movement_delta(io::controller_buttons(), &self.player.pos);

        let collision = check_box_collision(
            &self.tiles,
            Tile::Wall,
            PLAYER_WIDTH as i8,
            &self.player.pos,
            &player_delta,
        );
        if let Some(_) = collision.x {
            player_delta.x = 0;
            if !apu.is_playing() {
                apu.play_sfx(Sfx::Lock);
            }
        }
        if let Some(_) = collision.y {
            player_delta.y = 0;
            if !apu.is_playing() {
                apu.play_sfx(Sfx::Lock);
            }
        }

        let player_center = self.player.pos.shifted(&DPos::new(4, 4));
        if let Tile::Coin = get_tile_at(&self.tiles, &player_center) {
            let index = map_pos_to_tile_index(&player_center);
            self.tiles[index as usize] = Tile::Nothing;
            self.grabbed_coin_index = Some(index);
            self.n_coins -= 1;
            apu.play_sfx(Sfx::LevelUp);
        }

        self.player.pos.inc(&player_delta);

        for meanie in self.meanies.iter_mut() {
            update_meanie(&self.tiles, meanie)
        }
    }
}

fn player_movement_delta(buttons: u8, player_pos: &Pos) -> DPos {
    let mut delta = DPos::zero();

    if buttons & io::LEFT != 0 && player_pos.x > 0 {
        delta.x = -2;
    }
    if buttons & io::RIGHT != 0 && player_pos.x + 8 < WIDTH {
        delta.x = 2;
    }
    if buttons & io::UP != 0 && player_pos.y > 0 {
        delta.y = -2;
    }
    if buttons & io::DOWN != 0 && player_pos.y + 8 < WIDTH {
        delta.y = 2;
    }

    delta
}

fn i8_to_sign(i: i8) -> Option<Sign> {
    if i > 0 {
        Some(Sign::Plus)
    } else if i < 0 {
        Some(Sign::Minus)
    } else {
        None
    }
}

fn check_box_collision(
    tiles: &[Tile],
    colliding_tile: Tile,
    width: i8,
    pos: &Pos,
    pos_delta: &DPos,
) -> Collision {
    let mut collision = Collision { x: None, y: None };
    for box_delta in [
        DPos::new(0, 0),
        DPos::new(0, width),
        DPos::new(width, 0),
        DPos::new(width, width),
    ] {
        let box_pos = pos.shifted(&box_delta);
        if get_tile_at(tiles, &box_pos.shifted(&pos_delta.x_vec())) == colliding_tile {
            collision.x = i8_to_sign(pos_delta.x);
        }
        if get_tile_at(tiles, &box_pos.shifted(&pos_delta.y_vec())) == colliding_tile {
            collision.y = i8_to_sign(pos_delta.y);
        }
    }
    collision
}

fn update_meanie(tiles: &[Tile], meanie: &mut Meanie) {
    let mut delta;
    loop {
        delta = meanie.vel.scaled(DT as i8);
        let collision =
            check_box_collision(tiles, Tile::Wall, PLAYER_WIDTH as i8, &meanie.pos, &delta);
        if let Vec2 { x: None, y: None } = collision {
            break;
        }
        delta = delta.rotate(meanie.orientation);
        meanie.vel = delta;
        meanie.n_turns += 1;
    }

    meanie.pos.inc(&delta);

    if meanie.n_turns > 50 {
        meanie.orientation = meanie.orientation.reverse();
        meanie.n_turns = 0
    }
}
