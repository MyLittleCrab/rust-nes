use crate::addr::Addr;

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
