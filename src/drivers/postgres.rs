#![cfg(feature = "driver-postgres")]
use drivers::Driver;

pub struct PostgresDriver {
  target: String,
}

impl PostgresDriver {
  fn new(target: String) -> Self { PostgresDriver { target: target } }
}

// TODO Implement
impl Driver for PostgresDriver {
  fn get_status(&self) -> isize { return 0; }

  fn set_status(&mut self, status: isize) {}

  fn execute(&self, motion: &String) {}
}
