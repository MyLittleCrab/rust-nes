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
pub struct Addr(pub u16);
impl Addr {
    pub fn as_ptr(self) -> *mut u8 {
        return self.0 as *mut u8
    }
    pub fn read(self) -> u8 {
        unsafe {
            self.as_ptr().read_volatile()
        }
        
    }
    pub fn write(self, value: u8) {
        unsafe {
            self.as_ptr().write_volatile(value)
        }
    }
    pub fn offset(self, count: isize) -> Self {
        unsafe {
            Addr(self.0 + (count as u16))
        }
    }
}

impl Deref for Addr {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        unsafe{
            &*self.as_ptr()
        }
    }
}

impl DerefMut for Addr {

    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.as_ptr()
        }
        
    }
}