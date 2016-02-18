use std::result;
use std::error::Error;
use std::fmt;
use drivers::Driver;

type Result<T> = result::Result<T, NoError>;

pub struct DefaultDriver {
  target: String,
  status: isize
}

impl DefaultDriver {
  pub fn new(target: String) -> Self {
    println!("Creating a new default driver with target: {}", target);
    DefaultDriver {
      target: target,
      status: 0
    }
  }
}

impl Driver for DefaultDriver {
  type E = NoError;

  fn get_status(&self) -> Result<isize> {
    println!("The Status of {0}\n\t is {1}", self.target, self.status);
    Ok(self.status)
  }

  fn set_status(&mut self, status: isize) -> Result<()> {
    println!("Set Status of {0}\n\t from {1}\n\t to {2}", self.target, self.status, status);
    self.status = status;
    Ok(())
  }

  fn execute(&self, motion: &String) -> Result<()> {
    println!("I am a {0}\n\t that says {1}\n\t while at {2}", self.target, motion, self.status);
    Ok(())
  }
}

/// Basic unit struct to appease compiler errors.
pub struct NoError;

impl fmt::Debug for NoError {
  fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
    unreachable!()
  }
}

impl fmt::Display for NoError {
  fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
    unreachable!()
  }
}

impl Error for NoError {
  fn description(&self) -> &str {
    unreachable!()
  }
}
