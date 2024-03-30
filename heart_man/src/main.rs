#![no_std]
#![feature(start)]
#![allow(unused_imports, dead_code)]

extern crate alloc;
extern crate mos_alloc;

use game::Game;
use nes::addr::Addr;
use nes::ppu_buffer::BufferTrait;
use nes::sprites::SpriteState;
use nes::{apu, io, ppu, ppu_buffer, sprites};
use rng::Rng;

mod constants;
mod game;
//mod breakout;
mod level;
mod rng;
mod utils;

struct Buffer(ppu_buffer::Buffer<20>);
impl ppu_buffer::BufferTrait<20> for Buffer {
    unsafe fn buffer() -> &'static mut ppu_buffer::Buffer<20> {
        &mut BUFFER.0
    }
}
static mut BUFFER: Buffer = Buffer(ppu_buffer::Buffer::INIT);

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    apu::init();
    let mut apu = apu::APU::default();
    let mut sprites = SpriteState::default();
    let mut game = None;
    Game::new(&mut game);

    unsafe {
        game::init(game.as_mut().unwrap());
    }

    unsafe {
        ppu::enable_nmi();
    }

    loop {
        io::wait_for_vblank();
        io::poll_controller(); // here?
        sprites.clear();
        apu.run_sfx();
        game::frame(game.as_mut().unwrap(), &mut apu, &mut sprites);
    }
}

// no fancy logic here!! we're in NMI
// in particular, NMI does not play nicely with the heap
// we can read the length of Vecs but not their contents(?)
// likely takes too long
// we are very close to the number of cycles that can fit in
// the vblank
#[no_mangle]
pub extern "C" fn render() {
    //io::poll_controller();
    unsafe {
        sprites::dma();
        Buffer::render();
        ppu::reset();
    }
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
        unsafe { p.write(location.line() as u8) }
    } else {
        print_error("No location", p)
    };

    loop {}
}

fn print_error(s: &str, p: &mut Addr) {
    for ch in s.chars() {
        unsafe {
            p.write(ch as u8);
        }
        p.add(1);
    }
}
