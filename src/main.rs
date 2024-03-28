#![no_std]
#![feature(start)]
#![allow(unused_imports, dead_code)]

extern crate alloc;
extern crate mos_alloc;

use game::Game;
use sprites::SpriteState;
use utils::Addr;

mod apu;
mod constants;
mod game;
//mod breakout;
mod io;
mod level;
mod ppu;
mod rng;
mod sprites;
mod utils;

// fixed memory usage;
// 0x80 - nmi check bit
// 0x200 - OAM (reserved in linker)

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    apu::init();
    let mut apu = apu::APU::default();
    let mut sprites = SpriteState::default();
    game::init();

    ppu::enable_nmi();

    loop {
        io::wait_for_vblank();
        io::poll_controller(); // here?
        sprites.clear();
        apu.run_sfx();
        game::frame(&mut apu, &mut sprites);
    }
}

// we are very close to the number of cycles that can fit in
// the vblank
#[no_mangle]
pub extern "C" fn render() {
    //io::poll_controller();
    sprites::dma();
    game::render();
    ppu::reset();
}

#[link_section = ".chr_rom"]
#[no_mangle]
pub static TILES: [u8; 4096] = *include_bytes!("./chr/tiles.chr");

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    let p = &mut Addr(0xE0);
    if let Some(location) = panic_info.location() {
        print_error(location.file(), p);
        print_error(" line: ", p);
        p.write(location.line() as u8)
    } else {
        print_error("No location", p)
    };

    loop {}
}

fn print_error(s: &str, p: &mut Addr) {
    for ch in s.chars() {
        p.write(ch as u8);
        p.add(1);
    }
}
