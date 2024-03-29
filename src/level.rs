use crate::{
    constants::{COIN_SPRITE, N_ROWS, ORIGIN, ROW, WALL_SPRITE},
    ppu,
    rng::Rng,
    utils::Pos,
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

pub fn make_level<const N: usize>(tiles: &mut [Tile; N], rng: &mut Rng) {
    for t in tiles.iter_mut() {
        rng.cycle();
        if rng.get() % 4 == 0 {
            *t = Tile::Wall;
        } else if rng.get() % 41 == 1 {
            *t = Tile::Coin;
        }
    }

    for i in 0..ROW {
        tiles[i as usize] = Tile::Wall;
    }
    for i in 0..(N_ROWS as u16) {
        tiles[(i as usize) * (ROW as usize) as usize] = Tile::Wall;
        tiles[(i as usize + 1) * (ROW as usize) - 1] = Tile::Wall;
    }
    for i in 0..ROW {
        tiles[(ROW as usize) * (N_ROWS as usize - 2) + i as usize] = Tile::Wall;
    }
    // for (t, target) in tiles.iter_mut().zip(TEST) {
    //     *t = *target;
    // }
}

pub fn draw_level<const N: usize>(tiles: &mut [Tile; N]) {
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
        Tile::Nothing
    }
}

pub fn map_pos_to_tile_index(pos: &Pos) -> u16 {
    let x_shift = pos.x - 0;
    let y_shift = pos.y - 0;
    let x_tile = x_shift / 8 + 2;
    let y_tile = y_shift / 8 + 1;
    return (x_tile as u16) + (y_tile as u16) * (ROW as u16);
}
