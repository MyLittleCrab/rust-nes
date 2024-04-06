pub const INIT_SEED: u16 = next_seed(0x8988); // holds a special place in my heart

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

const fn next_seed(seed: u16) -> u16 {
    let new_bit = ((seed >> 9) ^ (seed >> 1)) & 1;
    (new_bit << 15) | (seed >> 1)
}

pub const fn seed_to_rng(seed: u16) -> u8 {
    (seed >> 8) as u8
}

pub const fn get_seeds<const N: usize>() -> [u16; N] {
    let mut thing = [0; N];
    let mut i = 0;
    let mut seed = INIT_SEED;
    while i < N {
        thing[i] = seed;
        seed = next_seed(seed);
        i += 1;
    }
    thing
}
