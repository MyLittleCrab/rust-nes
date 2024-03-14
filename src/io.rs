use crate::utils::Addr;
// need to import at least one C function to force the linker to work (?)
extern "C" {
    fn wait_vblank();
}

pub fn wait_for_vblank() {
    unsafe { wait_vblank() };
}

pub fn _set_chr_bank(bank: u8) {
    unsafe {
        *(0x8000 as *mut u8) = bank;
    }
}

pub const RIGHT: u8 = 0x01;
pub const LEFT: u8 = 0x02;
pub const DOWN: u8 = 0x04;
pub const UP: u8 = 0x08;
pub const START: u8 = 0x10;
pub const SELECT: u8 = 0x20;
pub const B: u8 = 0x40;
pub const A: u8 = 0x80;

static JOYPAD1: Addr = Addr(0x4016);
static mut BUTTONS: u8 = 0;

pub fn poll_controller() {
    // TODO: https://www.nesdev.org/wiki/Controller_reading_code#DPCM_Safety_using_Repeated_Reads

    unsafe {
        JOYPAD1.write(1);
        JOYPAD1.write(0);

        for _ in 0..8 {
            let a = JOYPAD1.read();
            BUTTONS <<= 1;
            BUTTONS |= a & 1;
        }
    }
}

pub fn controller_buttons() -> u8 {
    unsafe { BUTTONS }
}

pub fn byte_to_digits(b: u8) -> [u8; 2] {
    let d1 = b / 0x10;
    let d0 = b % 0x10;
    [d0, d1]
}
pub fn u16_to_digits(x: u16) -> [u8; 4] {
    let d10 = byte_to_digits(x as u8);
    let d32 = byte_to_digits((x / 0x100) as u8);
    [d32[1], d32[0], d10[1], d10[0]]
}

pub fn digit_to_ascii(b: u8) -> u8 {
    if b < 10 {
        65 - 19 + b + 2
    } else {
        65 - 2 + b - 10 + 2
        //0x60
    }
}
