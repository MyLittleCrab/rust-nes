use core::{
    array::from_fn,
    ops::{Deref, DerefMut},
};

use alloc::vec::Vec;

#[derive(Copy, Clone)]
pub struct Addr(pub u16);
impl Addr {
    pub fn as_ptr(self) -> *mut u8 {
        self.0 as *mut u8
    }
    pub fn read(self) -> u8 {
        unsafe { self.as_ptr().read_volatile() }
    }
    pub fn write(self, value: u8) {
        unsafe { self.as_ptr().write_volatile(value) }
    }
    pub fn offset(self, count: isize) -> Self {
        Addr(self.0 + (count as u16))
    }
    pub fn add(&mut self, count: isize) {
        self.0 = self.offset(count).0;
    }

    pub fn addr(self) -> u16 {
        self.0
    }
    pub fn write16(self, value: u16) {
        let bytes = value.to_le_bytes();
        self.write(bytes[0]);
        self.offset(1).write(bytes[1]);
    }
}

impl Deref for Addr {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for Addr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_ptr() }
    }
}

pub fn inc_u8(x: u8, dx: i8) -> u8 {
    ((x as i8) + dx) as u8
}

pub fn debug_value(at: u16, value: u8) {
    Addr(at).write(0xaa);
    Addr(at + 1).write(value);
    Addr(at + 2).write(0xaa);
    Addr(at + 3).write(0xab);
}

#[derive(Copy, Clone)]
pub enum Orientation {
    Clockwise,
    Widdershins,
}

#[derive(Copy, Clone)]
pub enum Sign {
    Plus,
    Minus,
}
impl Sign {
    pub fn to_i8(self) -> i8 {
        match self {
            Self::Plus => 1,
            Self::Minus => -1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub type Pos = Vec2<u8>;
impl Pos {
    pub fn inc(&mut self, delta: &DPos) {
        self.x = inc_u8(self.x, delta.x);
        self.y = inc_u8(self.y, delta.y);
    }
    pub fn shifted(&self, delta: &DPos) -> Pos {
        let mut new_pos = self.clone();
        new_pos.inc(delta);
        new_pos
    }
}

impl Orientation {
    pub fn reverse(self) -> Self {
        match self {
            Self::Clockwise => Self::Widdershins,
            Self::Widdershins => Self::Clockwise,
        }
    }
}

pub type DPos = Vec2<i8>;
impl DPos {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn x_vec(&self) -> Self {
        Self { x: self.x, y: 0 }
    }
    pub fn y_vec(&self) -> Self {
        Self { x: 0, y: self.y }
    }
    pub fn scaled(&self, scalar: i8) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
    pub fn rotate(&self, orientation: Orientation) -> Self {
        match orientation {
            Orientation::Clockwise => Self {
                x: -self.y,
                y: self.x,
            },
            Orientation::Widdershins => Self {
                x: self.y,
                y: -self.x,
            },
        }
    }
}

// this could just return a [u3 ; 3] but i wanted a demo of
// Vecs working properly
pub fn u8_to_decimal(b: u8) -> Vec<u8> {
    let mut digits = Vec::with_capacity(3);
    let mut a = b;
    loop {
        digits.push(a % 10);
        let div = a / 10;
        if div == 0 {
            break;
        } else {
            a = div;
        }
    }
    digits
}

pub struct CappedVec<T, const N: usize> {
    pub directives: [T; N],
    pub len: usize,
}
// TODO impl iterator
impl<T, const N: usize> CappedVec<T, N> {
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn push(&mut self, x: T) {
        if self.len < N {
            self.directives[self.len] = x;
            self.len += 1;
        } else {
            panic!("Buffer full")
        }
    }
    pub fn extend(&mut self, xs: Vec<T>) {
        for x in xs {
            self.push(x)
        }
    }
}
