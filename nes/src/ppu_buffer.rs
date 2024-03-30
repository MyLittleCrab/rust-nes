use alloc::vec::Vec;

use crate::{capped_vec::CappedVec, ppu};

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
    pub const INIT: Self = CappedVec {
        directives: [BufferDirective::Done; N],
        len: 0,
    };
}

// this is the only way I can think of to provide an interface to a static
// mutable buffer. Implementers define the buffer and provide a ref to it
// by implementing buffer().
pub trait BufferTrait<const N: usize> {
    const BUFFER_SIZE: usize = N;
    unsafe fn buffer() -> &'static mut Buffer<N>;
    fn render() {
        for d in unsafe { Self::buffer() }.directives.iter() {
            match *d {
                BufferDirective::Done => break,
                BufferDirective::Index(a) => ppu::write_addr(a),
                BufferDirective::Tile(t) => ppu::write_data(t),
            }
        }
    }
    fn push(x: BufferDirective) {
        unsafe { Self::buffer() }.push(x)
    }
    fn extend(xs: Vec<BufferDirective>) {
        unsafe { Self::buffer() }.extend(xs)
    }
    fn clear() {
        unsafe { Self::buffer() }.clear()
    }

    fn draw_text(text: &str) {
        for ch in text.chars() {
            Self::push(BufferDirective::Tile(ch as u8 - 32));
        }
    }
}
