//! This module deals with the "outside world". Anything which can have a
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

pub mod default;

#[cfg(feature = "driver-postgres")]
pub mod postgres;
