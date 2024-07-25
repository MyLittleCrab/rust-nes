use nes::constants::ROW;

pub const N_ROWS: u8 = 28; // 28 fills screen
pub const GRID_SIZE: u16 = (ROW as u16) * (N_ROWS as u16);

pub const ORIGIN: u16 = 0x2020;

pub const PLAYER_WIDTH: u8 = 6;
pub const PLAYER_SPEED: i8 = 1;

pub const WIDTH: u8 = 224;
pub const HEIGHT: u8 = 208;

pub const HEART_SPRITE: u8 = 0x63;
pub const WALL_SPRITE: u8 = 0x60;
pub const COIN_SPRITE: u8 = HEART_SPRITE + 7;
pub const AT_SPRITE: u8 = HEART_SPRITE - 4;

pub const DT: u8 = 1;
