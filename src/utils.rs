use core::ops::{Deref, DerefMut};

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
    pub fn addr(self) -> u16 {
        self.0
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
