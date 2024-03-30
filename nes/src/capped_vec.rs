use alloc::vec::Vec;

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
            panic!("Vec full")
        }
    }
    pub fn extend(&mut self, xs: Vec<T>) {
        for x in xs {
            self.push(x)
        }
    }
}
