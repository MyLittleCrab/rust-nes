use core::ptr::addr_of_mut;

use alloc::vec::Vec;

use crate::{capped_vec::CappedVec, ppu};

const BUFFER_SIZE: usize = 20;

// needs to be accessible during vblank (nmi call)
// should be the only global variable
static mut PPU_BUFFER: Buffer<BUFFER_SIZE> = Buffer::INIT;

fn buffer() -> &'static mut Buffer<BUFFER_SIZE> {
    unsafe { &mut *addr_of_mut!(PPU_BUFFER) }
}

// TODO: this will take up unnecessary space
// since Index is twice as large as tile
#[derive(Copy, Clone)]
pub enum BufferDirective {
    Index(u16),
    Tile(u8),
    Done,
}

// can't seem to query Vecs from nmi so...
pub type Buffer<const N: usize> = CappedVec<BufferDirective, N>;
// TODO impl iterator
impl<const N: usize> Buffer<N> {
    const INIT: Self = CappedVec {
        directives: [BufferDirective::Done; N],
        len: 0,
    };
}

pub fn render() {
    for d in buffer().directives.iter() {
        match *d {
            BufferDirective::Done => break,
            BufferDirective::Index(a) => ppu::write_addr(a),
            BufferDirective::Tile(t) => ppu::write_data(t),
        }
    }
}

pub fn push(x: BufferDirective) {
    buffer().push(x)
}

pub fn extend(xs: Vec<BufferDirective>) {
    buffer().extend(xs)
}

pub fn clear() {
    buffer().clear()
}

pub fn draw_text(text: &str) {
    for ch in text.chars() {
        push(BufferDirective::Tile(ch as u8 - 32));
    }
}
