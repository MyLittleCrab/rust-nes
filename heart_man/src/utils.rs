use core::{
    array::from_fn,
    ops::{Deref, DerefMut},
};

use nes::capped_vec::CappedVec;

pub fn u8_to_decimal(b: u8) -> CappedVec<u8, 3> {
    let mut digits = CappedVec::new();
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
