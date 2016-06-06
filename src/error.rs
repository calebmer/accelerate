use std::io;

#[derive(Debug)]
pub enum Error {
  IO(io::Error),
  Other(String),
}

impl Error {
  pub fn new<S: Into<String>>(message: S) -> Self {
    Error::Other(message.into())
  }
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Error::IO(error)
  }
}
