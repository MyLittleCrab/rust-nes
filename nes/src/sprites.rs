use crate::addr::Addr;
use crate::vec2::Pos;

const ADDR: Addr = Addr(0x200);
const OAM_DMA: Addr = Addr(0x4014);
const OAM_ADDR: Addr = Addr(0x2003);

pub struct SpriteState {
    index: isize,
}
impl Default for SpriteState {
    fn default() -> Self {
        Self { index: 0 }
    }
}
impl SpriteState {
    pub fn clear(&mut self) {
        for i in 0..256 {
            *ADDR.offset(i) = 0;
        }
        self.index = 0;
    }
    pub fn add(&mut self, x: u8, y: u8, tile: u8, attr: u8) {
        // attr is palette + flags
        *ADDR.offset(self.index) = y;
        *ADDR.offset(self.index + 1) = tile;
        *ADDR.offset(self.index + 2) = attr;
        *ADDR.offset(self.index + 3) = x;
        self.index += 4;
    }
    pub fn add_at_pos(&mut self, pos: &Pos, tile: u8, attr: u8) {
        self.add(pos.x, pos.y, tile, attr)
    }
}

pub unsafe fn dma() {
    OAM_ADDR.write(0);
    OAM_DMA.write((ADDR.addr() >> 8) as u8);
}

#[allow(dead_code)]
pub const PRIORITY: u8 = 0b100000;
#[allow(dead_code)]
pub const HFLIP: u8 = 0b1000000;
#[allow(dead_code)]
pub const VFLIP: u8 = 0b10000000;
