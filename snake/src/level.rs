use nes::{constants::ROW, io::LEFT, vec2::Pos};

use crate::{
    constants::{COIN_SPRITE, GRID_SIZE, N_ROWS, ORIGIN, WALL_SPRITE},
    ppu,
    rng::{get_seeds, seed_to_rng, Rng},
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Nothing,
    Wall,
    Coin,
}

// const TEST: &[Tile] = unsafe{
//     transmute::<&[u8], &[Tile]>(include_bytes!("test_level.dat"))
// };
//
pub const fn seed_to_tile(seed: u16) -> Tile {
    let rng_val = seed_to_rng(seed);
    if rng_val % 4 == 0 {
        Tile::Wall
    } else if rng_val % 41 == 1 {
        Tile::Coin
    } else {
        Tile::Nothing
    }
}

pub const fn make_level<const N: usize>(seeds: &[u16; N]) -> [Tile; N] {
    let mut tiles = [Tile::Nothing; N];
    let mut i = 0;
    while i < N {
        tiles[i] = seed_to_tile(seeds[i]);
        i += 1;
    }
    i = 0;
    while i < ROW as usize {
        tiles[i] = Tile::Wall;
        tiles[(ROW as usize) * (N_ROWS as usize - 1) + i as usize] = Tile::Wall;
        i += 1;
    }
    i = 0;
    while i < (N_ROWS as usize) {
        tiles[(i as usize) * (ROW as usize) as usize] = Tile::Wall;
        tiles[(i as usize + 1) * (ROW as usize) - 1] = Tile::Wall;
        i += 1;
    }
    tiles
}

pub unsafe fn draw_level<const N: usize>(tiles: &[Tile; N]) {
    // draw level tiles
    ppu::write_addr(ORIGIN);

    for (i, tile) in tiles.iter().enumerate() {
        ppu::write_addr(ORIGIN + (i as u16));
        match tile {
            Tile::Nothing => ppu::write_data(0x00),
            Tile::Wall => ppu::write_data(WALL_SPRITE),
            Tile::Coin => ppu::write_data(COIN_SPRITE),
        }
    }
}

pub fn get_tile_at(tiles: &[Tile], pos: &Pos) -> Tile {
    let index = map_pos_to_tile_index(pos);
    if index < tiles.len() as u16 {
        tiles[index as usize]
    } else {
        Tile::Wall
    }
}

pub fn map_pos_to_tile_index(pos: &Pos) -> u16 {
    let x_tile = pos.x >> 3;
    let y_tile = (pos.y + 1) >> 3;
    return (x_tile as u16) + (y_tile as u16) * (ROW as u16) - (ROW as u16);
}
