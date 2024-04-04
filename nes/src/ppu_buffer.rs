use core::iter;

use crate::{addr::Addr, capped_vec::CappedVec, ppu};

enum State {
    GetLength,
    // u8 is the number of remaining tiles to draw
    WriteAddrHigh(u8),
    WriteAddrLow(u8),
    DrawTile(u8),
}

// can't seem to query Vecs from nmi so...
pub type Buffer<const N: usize> = CappedVec<u8, N>;
impl<const N: usize> Buffer<N> {
    pub const INIT: Self = CappedVec::new([0; N]);
}

// this is the only way I can think of to provide an interface to a static
// mutable buffer. Implementers define the buffer and provide a ref to it
// by implementing buffer().
pub trait BufferTrait<const N: usize> {
    const BUFFER_SIZE: usize = N;
    unsafe fn buffer() -> &'static mut Buffer<N>;
    unsafe fn render() {
        // See https://www.nesdev.org/wiki/The_frame_and_NMIs
        // Each segment of consecutive data stored as
        // length | start addr high | start addr low | tile data ...
        // we don't need a terminating byte with len = 0 because we only
        // iterate over the values that have been pushed to the buffer
        let mut state = State::GetLength;
        for b in unsafe { Self::buffer() } {
            state = match state {
                State::GetLength => State::WriteAddrHigh(*b),
                State::WriteAddrHigh(len) => {
                    ppu::write_addr_byte(*b);
                    State::WriteAddrLow(len)
                }
                State::WriteAddrLow(len) => {
                    ppu::write_addr_byte(*b);
                    State::DrawTile(len)
                }
                State::DrawTile(n_remaining) => {
                    ppu::write_data(*b);
                    match n_remaining {
                        1 => State::GetLength,
                        _ => State::DrawTile(n_remaining - 1),
                    }
                }
            };
        }
    }
    fn tiles<I: Iterator<Item = u8>>(addr: Addr, mut xs: I) {
        // if empty do nothing
        let first = match xs.next() {
            Some(t) => t,
            None => return,
        };
        let buffer = unsafe { Self::buffer() };
        let start_index = buffer.len();
        buffer.push(0); // skip for now, write after we've counted the tiles

        // push high then low address bytes
        buffer.push((addr.addr() >> 8) as u8);
        buffer.push(addr.addr() as u8);

        // push first tile
        buffer.push(first);
        let mut n_tiles: u8 = 1;

        // push remaining tiles
        for x in xs {
            buffer.push(x);
            n_tiles += 1;
        }

        // write number of tiles
        buffer.arr[start_index] = n_tiles;
    }
    fn tile(addr: Addr, x: u8) {
        Self::tiles(addr, iter::once(x))
    }
    fn clear() {
        unsafe { Self::buffer() }.clear()
    }

    fn draw_text(addr: Addr, text: &str) {
        Self::tiles(addr, text.chars().map(|c| c as u8 - 32))
    }
}
