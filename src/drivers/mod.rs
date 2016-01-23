//! This module deals with the "outside world". Anything which can have a
//! reversible string patch applied may be accelerated.

pub type MaybeError = super::MaybeError;

/// A driver is anything which may be accelerated. For example a database like
/// PostgreSQL or MySQL. Even a taco can be accelerated if it adopts this
/// interface.
pub trait Driver {
  /// Setup the state within the driver database. Often tables or other
  /// structures will need to be setup. Called every time the accelerate tool
  /// is used.
  fn init_state(&self) -> MaybeError {
    Ok(())
  }

  /// Gets the integer for the last motion applied.
  fn get_status(&self) -> isize;

  /// Sets the integer for the last motion applied.
  fn set_status(&mut self, status: isize) -> MaybeError;

  /// Executes a motion string. Does not discriminate based on the add or sub
  /// operation.
  fn execute(&self, motion: &String) -> MaybeError;
}

pub mod default;

#[cfg(feature = "driver-postgres")]
pub mod postgres;
