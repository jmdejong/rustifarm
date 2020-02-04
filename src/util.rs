

use std::cmp::{min, max};
use serde_json::Value;

pub fn clamp<T: Ord>(val: T, lower: T, upper: T) -> T{
	return max(min(val, upper), lower);
}

pub trait ToJson {
	fn to_json(&self) -> Value;
}


#[macro_export]
macro_rules! hashmap {
	( $($key:expr => $value:expr ),* ) => {{
		#[allow(unused_mut)]
		let mut h = std::collections::HashMap::new();
		$(
			h.insert($key, $value);
		)*
		h
	}}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	#[test]
	fn test_hashmap_macro() {
		let mut h = hashmap!("hello" => 1, "world" => 2);
		assert_eq!(h.remove("hello"), Some(1));
		assert_eq!(h.remove("world"), Some(2));
		assert!(h.is_empty());
		let h2: HashMap<i32, usize> = hashmap!();
		assert!(h2.is_empty());
		assert_eq!(h2, HashMap::new());
		
	}
}
