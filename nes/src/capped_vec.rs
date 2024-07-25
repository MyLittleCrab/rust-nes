use core::{
    array::IntoIter,
    iter::{Map, Take},
    mem::{self, MaybeUninit},
    slice::{Iter, IterMut},
};

pub struct CappedVec<T, const N: usize> {
    arr: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> CappedVec<T, N> {
    pub const fn new() -> Self {
        Self {
            arr: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn try_push(&mut self, x: T) -> Result<(), ()> {
        if self.len < N {
            self.arr[self.len].write(x);
            self.len += 1;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn push(&mut self, x: T) {
        self.try_push(x).expect("Vec full!")
    }
    // TODO: this is untested
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            let mut value = MaybeUninit::uninit();
            mem::swap(&mut self.arr[self.len], &mut value);
            Some(unsafe { value.assume_init() })
        } else {
            None
        }
    }
    pub fn write(&mut self, index: usize, value: T) -> &mut T {
        if index < self.len {
            self.arr[index].write(value)
        } else {
            panic!("Out of bounds!")
        }
        // TODO is writing twice Bad?
    }
    pub fn read(&mut self, index: usize) -> &T {
        if index < self.len {
            unsafe { self.arr[index].assume_init_ref() }
        } else {
            panic!("Out of bounds!")
        }
    }
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
    // Adding first and last methods
    pub fn first(&self) -> Option<&T> {
        if self.len > 0 {
            Some(unsafe { self.arr[0].assume_init_ref() })
        } else {
            None
        }
    }
    pub fn last(&self) -> Option<&T> {
        if self.len > 0 {
            Some(unsafe { self.arr[self.len - 1].assume_init_ref() })
        } else {
            None
        }
    }
}

impl<T, const N: usize> IntoIterator for CappedVec<T, N> {
    type Item = T;

    type IntoIter = Map<Take<IntoIter<MaybeUninit<T>, N>>, fn(MaybeUninit<T>) -> T>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr
            .into_iter()
            .take(self.len)
            .map(|x| unsafe { x.assume_init() })
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a CappedVec<T, N> {
    type Item = &'a T;

    type IntoIter = Map<Take<Iter<'a, MaybeUninit<T>>>, fn(&'a MaybeUninit<T>) -> &'a T>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr
            .iter()
            .take(self.len)
            .map(|x| unsafe { x.assume_init_ref() })
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut CappedVec<T, N> {
    type Item = &'a mut T;

    type IntoIter = Map<Take<IterMut<'a, MaybeUninit<T>>>, fn(&'a mut MaybeUninit<T>) -> &'a mut T>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr
            .iter_mut()
            .take(self.len)
            .map(|x| unsafe { x.assume_init_mut() })
    }
}

impl<T, const N: usize> Extend<T> for CappedVec<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x)
        }
    }
}

impl<T, const N: usize> FromIterator<T> for CappedVec<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut out = Self::new();
        out.extend(iter);
        out
    }
}
