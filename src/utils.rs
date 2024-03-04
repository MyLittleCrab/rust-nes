use core::ops::{Deref, DerefMut};

pub fn write(p: *mut u8, value: u8) {
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn read<T>(p: *const T) -> T {
    unsafe {
        core::ptr::read_volatile(p)
    }
}

#[derive(Copy, Clone)]
pub struct Addr(pub *mut u8);
impl Addr {
    pub fn read(self) -> u8 {
        unsafe {
            self.0.read_volatile()
        }
        
    }
    pub fn write(self, value: u8) {
        unsafe {
            self.0.write_volatile(value)
        }
    }
    pub fn offset(self, count: isize) -> Self {
        unsafe {
            Addr(self.0.offset(count) as *mut u8)
        }
    }
}

impl From<u16> for Addr {
    fn from(b: u16) -> Self {
        Addr(b as *mut u8)
    }
}

impl Deref for Addr {
    type Target = u8;

    // Required method
    fn deref(&self) -> &Self::Target {
        unsafe{
            &*self.0
        }
    }
}

impl DerefMut for Addr {

    // Required method
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0
        }
        
    }
}