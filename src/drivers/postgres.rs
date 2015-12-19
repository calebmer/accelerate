#![cfg(feature = "driver-postgres")]
use drivers::Driver as DriverTrait;

pub struct Driver {
  target: String,
}

// TODO Implement
impl DriverTrait for Driver {
  fn new(target: String) -> Self { Driver { target: target } }

  fn get_status(&self) -> isize { return 0; }

  fn set_status(&mut self, status: isize) -> &mut Self { self }

  fn execute(&self, motion: &String) -> &Self { self }
}
