use crate::utils::inc_u8;

#[derive(Copy, Clone)]
pub enum Orientation {
    Clockwise,
    Widdershins,
}

#[derive(Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub type Pos = Vec2<u8>;
impl Pos {
    pub fn inc(&mut self, delta: &DPos) {
        self.x = inc_u8(self.x, delta.x);
        self.y = inc_u8(self.y, delta.y);
    }
    pub fn shifted(&self, delta: &DPos) -> Pos {
        let mut new_pos = self.clone();
        new_pos.inc(delta);
        new_pos
    }
    pub fn delta(&self, other: &Pos) -> DPos {
        DPos {
            x: (self.x as i8) - (other.x as i8),
            y: (self.y as i8) - (other.y as i8),
        }
    }
    pub fn l1_dist(&self, other: &Pos) -> u8 {
        self.delta(other).l1_norm()
    }
}

impl Orientation {
    pub fn reverse(self) -> Self {
        match self {
            Self::Clockwise => Self::Widdershins,
            Self::Widdershins => Self::Clockwise,
        }
    }
}

pub type DPos = Vec2<i8>;
impl DPos {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn x_vec(&self) -> Self {
        Self { x: self.x, y: 0 }
    }
    pub fn y_vec(&self) -> Self {
        Self { x: 0, y: self.y }
    }
    pub fn scaled(&self, scalar: i8) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
    pub fn rotate(&self, orientation: Orientation) -> Self {
        match orientation {
            Orientation::Clockwise => Self {
                x: -self.y,
                y: self.x,
            },
            Orientation::Widdershins => Self {
                x: self.y,
                y: -self.x,
            },
        }
    }
    pub fn l1_norm(&self) -> u8 {
        (self.x).abs() as u8 + (self.y).abs() as u8
    }
}
