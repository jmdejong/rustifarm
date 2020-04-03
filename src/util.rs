
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::cmp::{min, max};

pub fn clamp<T: Ord>(val: T, lower: T, upper: T) -> T{
	max(min(val, upper), lower)
}

pub type AnyError = Box<dyn Error + 'static>;
pub type Result<T> = std::result::Result<T, AnyError>;

#[derive(Debug)]
pub struct AError {
	text: String
}

impl AError {
	pub fn new(txt: &str) -> Self{
		AError {
			text: txt.to_string()
		}
	}
}

impl Error for AError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}

impl Display for AError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.text)
    }
}


#[macro_export]
macro_rules! aerr {
	($($description:tt)*) => {Box::new(crate::util::AError::new(&format!($($description)*)))}
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
