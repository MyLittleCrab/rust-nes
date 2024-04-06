use crate::addr::Addr;

const CTRL_VRAM_INC: u8 = 0b100;
const _CTRL_NMI: u8 = 0b10000000;
const PPU_CTRL: Addr = Addr(0x2000);
const PPU_MASK: Addr = Addr(0x2001);
#[allow(dead_code)]
const PPU_STATUS: Addr = Addr(0x2002);
const PPU_SCROLL: Addr = Addr(0x2005);
const PPU_ADDR: Addr = Addr(0x2006);
const PPU_DATA: Addr = Addr(0x2007);

// all calls to PPU are unsafe because they may only
// be safely made during the vblank interval
pub unsafe fn reset() {
    write_addr(PPU_CTRL.addr());
    scroll(0, 0);
}
pub unsafe fn write_ctrl(value: u8) {
    PPU_CTRL.write(value)
}

pub unsafe fn and_ctrl(value: u8) {
    let next = PPU_CTRL.read() & value;
    PPU_CTRL.write(next);
}

pub unsafe fn or_ctrl(value: u8) {
    let next = PPU_CTRL.read() | value;
    PPU_CTRL.write(next);
}

pub unsafe fn write_mask(value: u8) {
    PPU_MASK.write(value);
}

pub unsafe fn write_addr_byte(value: u8) {
    PPU_ADDR.write(value);
}

pub unsafe fn write_addr(value: u16) {
    write_addr_byte((value >> 8) as u8);
    write_addr_byte(value as u8);
}

pub unsafe fn write_data(value: u8) {
    PPU_DATA.write(value);
}

pub unsafe fn scroll(x: u8, y: u8) {
    PPU_SCROLL.write(x);
    PPU_SCROLL.write(y)
}

// TODO: are enable / disable nmi safe?
pub unsafe fn enable_nmi() {
    write_ctrl(0x80);
    write_mask(0x1E);
}

pub unsafe fn disable_nmi() {
    write_mask(0);
    write_ctrl(0);
}

#[inline(never)]
pub unsafe fn clear_nametable() {
    write_addr(PPU_CTRL.addr());
    for _ in 0..0x400 {
        write_data(0);
    }
}

pub unsafe fn draw_text(text: &str) {
    for ch in text.chars() {
        write_data(ch as u8 - 32);
    }
}

pub unsafe fn draw_ascii(off: u16, ascii: &str) {
    for (i, line) in ascii.split("\n").enumerate() {
        write_addr(off + (0x20 * i as u16));
        draw_text(line);
    }
}

#[inline(never)]
pub unsafe fn draw_box(x: u8, y: u8, w: u8, h: u8) {
    const BOX_TILES: u8 = 0x73;
    let offset = 0x2000 + (x as u16 + (y as u16 * 0x20));
    // -
    write_addr(offset);
    write_data(BOX_TILES);
    for _ in 0..w - 2 {
        write_data(BOX_TILES + 5);
    }
    write_data(BOX_TILES + 1);
    // |
    write_addr(offset + 0x20);
    or_ctrl(CTRL_VRAM_INC);
    for _ in 0..h - 2 {
        write_data(BOX_TILES + 4);
    }
    write_data(BOX_TILES + 2);
    // |
    write_addr(offset + 0x20 + w as u16 - 1);
    for _ in 0..h - 2 {
        write_data(BOX_TILES + 4);
    }
    write_data(BOX_TILES + 3);
    and_ctrl(!CTRL_VRAM_INC);
    // _
    write_addr(offset + ((h as u16 - 1) * 0x20) + 1);
    for _ in 0..w - 2 {
        write_data(BOX_TILES + 5);
    }
}

pub const STR_OFFSET: u8 = 0x10;

pub unsafe fn write_bytes(offset: u16, pal: &[u8]) {
    write_addr(offset);

    pal.iter().for_each(|byte| {
        write_data(*byte);
    });
}

#[allow(dead_code)]
pub const PAL_BG_0: u16 = 0x3f00;
#[allow(dead_code)]
pub const PAL_BG_1: u16 = 0x3f04;
#[allow(dead_code)]
pub const PAL_BG_2: u16 = 0x3f08;
#[allow(dead_code)]
pub const PAL_BG_3: u16 = 0x3f0C;
#[allow(dead_code)]
pub const PAL_SPRITE_0: u16 = 0x3f10;
#[allow(dead_code)]
pub const PAL_SPRITE_1: u16 = 0x3f14;
#[allow(dead_code)]
pub const PAL_SPRITE_2: u16 = 0x3f18;
#[allow(dead_code)]
pub const PAL_SPRITE_3: u16 = 0x3f1C;
