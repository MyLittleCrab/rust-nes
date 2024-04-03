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
impl<const N: usize> Buffer<N> {
    pub const INIT: Self = CappedVec::new([BufferDirective::Done; N]);
}

// this is the only way I can think of to provide an interface to a static
// mutable buffer. Implementers define the buffer and provide a ref to it
// by implementing buffer().
pub trait BufferTrait<const N: usize> {
    const BUFFER_SIZE: usize = N;
    unsafe fn buffer() -> &'static mut Buffer<N>;
    unsafe fn render() {
        for d in unsafe { Self::buffer() } {
            match *d {
                BufferDirective::Done => break,
                BufferDirective::Index(a) => ppu::write_addr(a),
                BufferDirective::Tile(t) => ppu::write_data(t),
            }
        }
    }
    fn address(a: u16) {
        let buffer = unsafe { Self::buffer() };
        buffer.push(BufferDirective::Index(a));
    }
    fn tile(t: u8) {
        let buffer = unsafe { Self::buffer() };
        buffer.push(BufferDirective::Tile(t));
    }
    fn tiles<I: Iterator<Item = u8>>(xs: I) {
        unsafe { Self::buffer() }.extend(xs.map(BufferDirective::Tile))
    }
    fn clear() {
        unsafe { Self::buffer() }.clear()
    }

    fn draw_text(text: &str) {
        Self::tiles(text.chars().map(|c| c as u8 - 32))
    }
}
