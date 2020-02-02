

use std::cmp::{min, max};
use serde_json::Value;

pub fn clamp<T: Ord>(val: T, lower: T, upper: T) -> T{
	return max(min(val, upper), lower);
}

pub trait ToJson {
	fn to_json(&self) -> Value;
}
