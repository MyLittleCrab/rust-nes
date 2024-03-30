use core::{
    array::from_fn,
    ops::{Deref, DerefMut},
};

use alloc::vec::Vec;

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
