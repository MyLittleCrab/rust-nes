// https://www.nesdev.org/wiki/APU_basics
use crate::utils::{read, write, Addr};
const APU: Addr = Addr(0x4000);
const PULSE1: Addr = Addr(0x4000);
#[allow(dead_code)]
const PULSE2: Addr = Addr(0x4004);

#[derive(PartialEq)]
pub enum Sfx {
    ChangeScreen,
    MenuBoop,
    Pause,
    Shift,
    Rotate,
    Lock,
    LevelUp,
    Burn,
    FourLineClear,
    Topout,
    None,
}


pub fn init() {
        [
            0x30,0x08,0x00,0x00,
            0x30,0x08,0x00,0x00,
            0x80,0x00,0x00,0x00,
            0x30,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,
        ].iter().enumerate().for_each(|(i, byte)|{
            *APU.offset(i as _) = *byte;
        });
        *APU.offset(0x15) = 0xF;
        *APU.offset(0x17) = 0x40;
}

fn sfx_frame(p: Addr, hi: u8, lo: u8, dcvol: u8) -> bool {
    p.offset(2).write(lo);
    p.offset(3).write(hi); // only lower 3 bits matter
    p.write(dcvol);
    true
}

fn sfx_end(p: Addr) -> bool {
    p.write(0);
    false
}

#[allow(dead_code)]
fn noise_frame(tp: u8, vol: u8) -> bool {
    APU.offset(0xC).write(vol);
    APU.offset(0xE).write(tp);
    true
}

#[allow(dead_code)]
fn noise_end() -> bool {
    APU.offset(0xC).write(0b110000);
    APU.offset(0xE).write(0);
    false
}

pub struct APU {
    sfx: Sfx,
    sfx_off: usize
}
impl Default for APU {
    fn default() -> Self {
        Self {
            sfx: Sfx::None,
            sfx_off: 0
        }
    }
}
impl APU {
    pub fn play_sfx(&mut self, type_: Sfx) {
        self.sfx = type_;
        self.sfx_off = 0;
    }
    pub fn run_sfx(&mut self) {
        if self.sfx == Sfx::None {
            return
        }
        let cont = match self.sfx {
            Sfx::ChangeScreen | Sfx::Pause => {
                match self.sfx_off {
                    ..=5 => { sfx_frame(PULSE1, 1, 0x7C, 0b10111111) },
                    ..=10 => { sfx_frame(PULSE1, 1, 0xc4, 0b10111111) },
                    ..=15 => { sfx_frame(PULSE1, 0, 0xbf, 0b10111111) },
                    _ => { PULSE1.offset(3).write(7); sfx_end(PULSE1) }
                }
            },
            Sfx::MenuBoop => {
                match self.sfx_off {
                    ..=2 => { sfx_frame(PULSE1, 0, 0x90, 0b10110111) },
                    _ => { sfx_end(PULSE1) }
                }
            },
            Sfx::Shift => {
                match self.sfx_off {
                    ..=2 => { sfx_frame(PULSE1, 1, 0x7C, 0b10110111) },
                    _ => { sfx_end(PULSE1) }
                }
            },
            Sfx::Lock => {
                match self.sfx_off {
                    ..=2 => { sfx_frame(PULSE1, 5, 0x9d, 0b10110110) },
                    ..=3 => { sfx_frame(PULSE1, 6, 0xad, 0b10110110) },
                    _ => { sfx_end(PULSE1) }
                }
            },
            Sfx::Rotate => {
                match self.sfx_off {
                    ..=1 => { sfx_frame(PULSE1, 1, 0x7c, 0b10110110) },
                    ..=3 => { sfx_frame(PULSE1, 2, 0x1A, 0b10110000) },
                    ..=5 => { sfx_frame(PULSE1, 1, 0x7c, 0b10110110) },
                    _ => { sfx_end(PULSE1) }
                }
            },
            Sfx::Burn | Sfx::FourLineClear | Sfx::LevelUp => {
                const NOTES: &[u8] = &[0xfb,0xc4,0x93,0x67,0x3f,0x1c];

                if self.sfx_off / 4 >= NOTES.len() {
                    PULSE1.offset(3).write(7);
                    sfx_end(PULSE1)
                } else {
                    sfx_frame(PULSE1, 1, NOTES[self.sfx_off/4], 0b10111111)
                }
            },
            Sfx::Topout => {
                match self.sfx_off {
                    ..=5 => { sfx_frame(PULSE1, 4, 0x34, 0b10111110) },
                    ..=15 => { sfx_frame(PULSE1, 4, 0xb8, 0b10111000) },
                    ..=20 => { sfx_frame(PULSE1, 5, 0x4c, 0b10111110) },
                    ..=25 => { sfx_frame(PULSE1, 5, 0xf3, 0b10110110) },
                    _ => { sfx_end(PULSE1) }
                }
            },
            Sfx::None => unreachable!(),
        };

        if cont {
            self.sfx_off += 1;
        } else {
            self.sfx = Sfx::None
        }
    }
}

