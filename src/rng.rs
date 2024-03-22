pub const INIT_SEED: u16 = 0x8988;
static mut SEED: u16 = INIT_SEED;

pub fn next_seed(seed: u16) -> u16 {
    let new_bit = ((seed >> 9) ^ (seed >> 1)) & 1;
    (new_bit << 15) | (seed >> 1)
}

pub fn seed_to_rng(seed: u16) -> u8 {
    (seed >> 8) as u8
}

#[inline(never)]
pub fn cycle_rng() {
    unsafe { SEED = next_seed(SEED) }
}

pub fn get_rng() -> u8 {
    unsafe { seed_to_rng(SEED) }
}
