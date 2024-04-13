use nes::ppu::{and_ctrl, or_ctrl, write_addr, write_data};

const CTRL_VRAM_INC: u8 = 0b100;

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
