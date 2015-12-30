//! This module deals with the “outside world”. Anything which can have a
//! reversible string patch applied may be accelerated.

/// A driver is anything which may be accelerated. For example a database like
/// PostgreSQL or MySQL. Even a taco can be accelerated if it adopts this
/// interface.
pub trait Driver{
  /// Gets the integer for the last motion applied.
  fn get_status(&self) -> isize;
  /// Sets the integer for the last motion applied.
  fn set_status(&mut self, status: isize);
  /// Executes a motion string. Does not discriminate based on the add or sub
  /// operation.
  fn execute(&self, motion: &String);
}

// XXX: Should either be in the tests folder, or in another file.
pub struct DefaultDriver {
  target: String,
  status: isize,
}

impl DefaultDriver {
  pub fn new(target: String) -> Self {
    println!("Creating a new DefaultDriver with target: {}", target);
    DefaultDriver {
      target: target,
      status: 0,
    }
  }
}

impl Driver for DefaultDriver {
  fn get_status(&self) -> isize {
    println!("The Status of {0}\n\t is {1}", self.target, self.status);
    return self.status;
  }

  fn set_status(&mut self, status: isize) {
    println!("Set Status of {0}\n\t from {1}\n\t to {2}", self.target, self.status, status);
    self.status = status;
  }

  fn execute(&self, motion: &String) {
    println!("I am a {0}\n\t that says {1}\n\t while at {2}", self.target, motion, self.status);
  }
}

#[cfg(feature = "driver-postgres")]
pub mod postgres;
