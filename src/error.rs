use std::error;
use std::fmt;
use std::io;

pub type Error = Box<error::Error>;

#[macro_export]
macro_rules! error {
  ($($item:expr),*) => {{
    Box::new($crate::error::CustomError(format!($($item,)*))) as Error
  }}
}

#[derive(Debug)]
pub struct CustomError(pub String);

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl error::Error for CustomError {
  fn description(&self) -> &str {
    &self.0
  }
}
