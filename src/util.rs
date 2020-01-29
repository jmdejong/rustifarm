

use std::cmp::{min, max};

pub fn clamp<T: Ord>(val: T, lower: T, upper: T) -> T{
	return max(min(val, upper), lower);
}

