use core::{
    array,
    iter::Take,
    slice::{Iter, IterMut},
};

use alloc::vec::Vec;

pub struct CappedVec<T, const N: usize> {
    pub arr: [T; N],
    pub len: usize,
}
impl<T, const N: usize> CappedVec<T, N> {
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn push(&mut self, x: T) {
        if self.len < N {
            self.arr[self.len] = x;
            self.len += 1;
        } else {
            panic!("Vec full")
        }
    }
    pub fn extend(&mut self, xs: Vec<T>) {
        for x in xs {
            self.push(x)
        }
    }
}
impl<'a, T, const N: usize> IntoIterator for &'a CappedVec<T, N> {
    type Item = &'a T;

    type IntoIter = Take<Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.iter().take(self.len)
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut CappedVec<T, N> {
    type Item = &'a mut T;

    type IntoIter = Take<IterMut<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.iter_mut().take(self.len)
    }
}
