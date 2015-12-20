#![cfg(feature = "driver-postgres")]
use drivers::Driver as DriverTrait;

pub struct Driver {
  target: String,
}

impl Driver {
  fn new(target: String) -> Self { Driver { target: target } }
}

// TODO Implement
impl DriverTrait for Driver {
  fn get_status(&self) -> isize { return 0; }

  fn set_status(&mut self, status: isize) {}

  fn execute(&self, motion: &String) {}
}
