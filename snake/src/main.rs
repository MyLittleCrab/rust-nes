#![no_std]
#![feature(start)]
#![allow(unused_imports, dead_code)]

use core::fmt::Write;

use constants::ORIGIN;
use game::SnakeGame;
use nes::addr::Addr;
use nes::capped_vec::CappedVec;
use nes::ppu_buffer::BufferTrait;
use nes::sprites::SpriteState;
use nes::{apu, io, ppu, ppu_buffer, sprites};
use rng::Rng;
use utils::u8_to_decimal;

mod constants;
mod game;
mod level;
mod rng;
mod utils;

const BUFFER_SIZE: usize = 40;
struct Buffer(ppu_buffer::Buffer<BUFFER_SIZE>);
impl ppu_buffer::BufferTrait<BUFFER_SIZE> for Buffer {
    unsafe fn buffer() -> &'static mut ppu_buffer::Buffer<BUFFER_SIZE> {
        &mut BUFFER.0
    }
}
static mut BUFFER: Buffer = Buffer(ppu_buffer::Buffer::new());

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    apu::init();
    let mut apu = apu::APU::default();
    let mut sprites = SpriteState::default();
    let mut game = None;
    SnakeGame::new(&mut game);

    unsafe {
        game::init();
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

struct PPUWriter {
    start: Addr,
    started: bool,
}
impl PPUWriter {
    fn new(start: Addr) -> Self {
        Self {
            start,
            started: false,
        }
    }
}
impl Write for PPUWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.started {
            unsafe { ppu::draw_text(s) }
        } else {
            unsafe { ppu::draw_ascii(self.start.addr(), s) }
            self.started = true;
        }
        Ok(())
    }
}
struct MemoryWriter {
    start: Addr,
    current: Addr,
}
impl MemoryWriter {
    fn new(start: Addr) -> Self {
        Self {
            start,
            current: start,
        }
    }
}
impl Write for MemoryWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print_to_memory(s, &mut self.current);
        Ok(())
    }
}

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    io::wait_for_vblank();
    apu::silence_all();
    unsafe { ppu::disable_nmi() };
    let mut writer = PPUWriter::new(Addr(ORIGIN));
    //let mut writer = MemoryWriter::new(Addr(0xe0));
    //writeln!(writer, "{}", panic_info).ok();
    if let Some(location) = panic_info.location() {
        write!(writer, "Panic in {} on line ", location.file()).ok();
        draw_digits(location.line() as u8);
    } else {
        unsafe {
            ppu::draw_ascii(ORIGIN, "; no location");
        }
    };

    unsafe { ppu::enable_nmi() }
    loop {}
}

fn draw_digits(x: u8) {
    for d in u8_to_decimal(x)
        .into_iter()
        .rev()
        .map(|d| io::digit_to_ascii(d) - 32)
    {
        unsafe {
            ppu::write_data(d);
        }
    }
}

fn print_to_memory(s: &str, p: &mut Addr) {
    for ch in s.chars() {
        unsafe {
            p.write(ch as u8);
        }
        p.add(1);
    }
}
