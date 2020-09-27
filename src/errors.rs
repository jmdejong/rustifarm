
use std::error::Error;
use std::fmt::{Display, Formatter};

pub type AnyError = Box<dyn Error + 'static>;
pub type Result<T> = std::result::Result<T, AnyError>;

#[derive(Debug)]
pub struct AError {
	pub text: String
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
	($($description:tt)*) => {Box::new(crate::errors::AError{text: format!($($description)*)})}
}


