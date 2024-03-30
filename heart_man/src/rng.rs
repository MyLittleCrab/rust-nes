pub const INIT_SEED: u16 = 0x8988;

pub struct Rng(u16);
impl Rng {
    pub fn new(seed: Option<u16>) -> Self {
        Self(seed.unwrap_or(INIT_SEED))
    }
    pub fn cycle(&mut self) {
        self.0 = next_seed(self.0)
    }
    pub fn get(&mut self) -> u8 {
        seed_to_rng(self.0)
    }
}

fn next_seed(seed: u16) -> u16 {
    let new_bit = ((seed >> 9) ^ (seed >> 1)) & 1;
    (new_bit << 15) | (seed >> 1)
}

fn seed_to_rng(seed: u16) -> u8 {
    (seed >> 8) as u8
}
