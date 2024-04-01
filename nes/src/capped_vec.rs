use core::{
    iter::Take,
    slice::{Iter, IterMut},
};

pub struct CappedVec<T, const N: usize> {
    pub arr: [T; N],
    len: usize,
}
impl<T, const N: usize> CappedVec<T, N> {
    pub const fn new(arr: [T; N]) -> Self {
        Self {
            arr, len: 0
        }
    }
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

impl<T, const N: usize> Extend<T> for CappedVec<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x)
        }
    }
}
