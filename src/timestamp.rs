
use std::ops::{Add, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub i64);

impl Add<i64> for Timestamp {
    type Output = Self;
    fn add(self, other: i64) -> Self {
        Self(self.0 + other)
    }
}

impl Sub<i64> for Timestamp {
    type Output = Self;
    fn sub(self, other: i64) -> Self {
        Self(self.0 - other)
    }
}

impl Sub<Self> for Timestamp {
    type Output = i64;
    fn sub(self, other: Self) -> i64 {
        self.0 - other.0
    }
}
