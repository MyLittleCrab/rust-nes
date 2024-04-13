#![no_std]
#![feature(start)]
#![allow(unused_imports, dead_code)]

use nes::{apu, io, ppu, sprites};

mod draw;
mod game;

// fixed memory usage;
// 0x80 - nmi check bit
// 0x200 - OAM (reserved in linker)

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    apu::init();
    let mut apu = apu::APU::default();
    let mut sprite_state = sprites::SpriteState::default();
    unsafe {
        game::init();
        ppu::enable_nmi();
    }

    loop {
        io::wait_for_vblank();
        sprite_state.clear();
        apu.run_sfx();
        //apu::run_sfx();
        game::frame(&mut apu, &mut sprite_state);
    }
}

#[no_mangle]
pub extern "C" fn render() {
    io::poll_controller();
    unsafe {
        sprites::dma();
        game::render();
        ppu::write_addr(0x2000);
        ppu::scroll(0, 0);
    }
}

#[link_section = ".chr_rom"]
#[no_mangle]
pub static TILES: [u8; 4096] = *include_bytes!("chr/tiles.chr");

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let mut p = 0xE0 as *mut u8;
    for ch in "PANIC".chars() {
        unsafe {
            *p = ch as u8;
            p = p.add(1);
        }
    }
    loop {}
}
