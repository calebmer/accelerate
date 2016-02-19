//! This module deals with the "outside world". Anything which can have a
//! reversible string patch applied may be accelerated.

use std::error::Error;
use motions::Motion;
use operation::Operation;
use operation::Operation::*;

/// A driver is anything which may be accelerated. For example a database like
/// PostgreSQL or MySQL. Even a taco can be accelerated if it adopts this
/// interface.
pub trait Driver {
  /// The error type for the driver. Must inherit from the standard library
  /// error trait.
  type E: Error;

  /// Setup the state within the driver database. Often tables or other
  /// structures will need to be setup. Called every time the accelerate tool
  /// is used.
  fn init_state(&self) -> Result<(), Self::E> {
    Ok(())
  }

  /// Gets the integer for the last motion applied.
  fn get_status(&self) -> Result<isize, Self::E>;

  /// Sets the integer for the last motion applied.
  fn set_status(&mut self, status: isize) -> Result<(), Self::E>;

  /// Executes a motion string. Does not discriminate based on the add or sub
  /// operation.
  fn execute_motion(&self, motion: &String) -> Result<(), Self::E>;

  /// Executes a set range of motions on the driver. This function is not to be
  /// confused with the driver execute function which only executes a single
  /// motion. This function provides the logic for deciding execution order.
  ///
  /// This function must be given the driver implementation and *all* known
  /// motions.
  ///
  /// Motions will always be applied moving from start to finish. So if start is
  /// a larger number then finish we will execute all the "sub" motions until we
  /// reach finish, and if the start number is smaller we will execute all the
  /// "add" motions until we reach finish.
  fn execute(&mut self, motions: &Vec<Motion>, some_start: isize, some_finish: isize) -> Result<(), Self::E> {
    let start = clamp(some_start, 0, motions.len() as isize);
    let finish = clamp(some_finish, 0, motions.len() as isize);
    if start != finish {
      let operation = Operation::get(finish, start);
      let mut i = start;
      loop {
        if i == finish {
          break;
        }

        if operation == Operation::Sub {
          i += Operation::sub();
        }

        match self.execute_motion(&motions[i as usize].add) {
          Ok(_) => (),
          // Handle execution error by first trying to set the new status.
          Err(err) => { try!(self.set_status(i)); return Err(err) }
        }

        if operation == Operation::Add {
          i += Operation::add();
        }
      }
      self.set_status(i)
    } else {
      Ok(())
    }
  }

  /// Starts at the current driver status and executes n motions in either the
  /// "add" or "sub" direction.
  fn shift(&mut self, motions: &Vec<Motion>, n: isize) -> Result<(), Self::E> {
    let start = try!(self.get_status());
    let finish = clamp(start + n, 0, motions.len() as isize);
    self.execute(motions, start, finish)
  }


  /// Executes as many motions as necessary to move the driver from its current
  /// status to a certain state.
  fn goto(&mut self, motions: &Vec<Motion>, finish: isize) -> Result<(), Self::E> {
    let status = try!(self.get_status());
    self.execute(motions, status, finish)
  }

  /// "sub's" the last motion then "add's" it back.
  fn redo(&mut self, motions: &Vec<Motion>) -> Result<(), Self::E> {
    try!(self.shift(motions, -1));
    self.shift(motions, 1)
  }

  /// Applies all remaining "add" motions to the driver.
  fn up(&mut self, motions: &Vec<Motion>) -> Result<(), Self::E> {
    let last = motions.len() as isize;
    self.goto(motions, last)
  }

  /// Applies all remaining "sub" motions to the driver.
  fn down(&mut self, motions: &Vec<Motion>) -> Result<(), Self::E> {
    self.goto(motions, 0)
  }

  /// Applies all remaining "sub" motions to the driver before adding them all
  /// the subbed motions back.
  fn reset(&mut self, motions: &Vec<Motion>) -> Result<(), Self::E> {
    let status = try!(self.get_status());
    try!(self.execute(motions, status, 0));
    self.execute(motions, 0, status)
  }
}

/// Forces an integer to be inside a range. Similar to using both the algebraic
/// `min` and `max` functions with the specified values.
fn clamp(n: isize, min: isize, max: isize) -> isize {
  if n < min {
    return min;
  }
  if n > max {
    return max;
  }
  n
}

#[cfg(test)]
mod tests {
  #![allow(unused_must_use)]
  use driver::Driver;
  use drivers::default::DefaultDriver;
  use motions::Motion;

  fn get_motions() -> Vec<Motion> {
    (0..8).map(|n| Motion {
      name: "test".to_string(),
      add: "add: ".to_string() + &n.to_string(),
      sub: "sub: ".to_string() + &n.to_string(),
      version: vec![n, n + 1, n + 2],
      extension: String::from("")
    }).collect()
  }

  fn get_driver() -> DefaultDriver {
    DefaultDriver::new("Test Driver".to_string())
  }

  #[test]
  fn up() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.up(&mots);
    assert_eq!(mots.len() as isize, driver.get_status().unwrap());
  }

  #[test]
  fn down() {
    let mut driver = get_driver();
    driver.down(&get_motions());
    assert_eq!(0, driver.get_status().unwrap());
  }

  #[test]
  fn down_up() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.down(&mots);
    driver.up(&mots);
    assert_eq!(mots.len() as isize, driver.get_status().unwrap());
  }

  #[test]
  fn up_down() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.up(&mots);
    driver.down(&mots);
    assert_eq!(0, driver.get_status().unwrap());
  }

  #[test]
  fn redo() {
    let mut driver = get_driver();
    driver.redo(&get_motions());
    // sub at 0 will do nothing and then add thus the status should be 1
    assert_eq!(1, driver.get_status().unwrap());
  }

  #[test]
  fn shift2_redo() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.shift(&mots, 2);
    driver.redo(&mots);
    assert_eq!(2, driver.get_status().unwrap());
  }

  #[test]
  fn shift() {
    let mut driver = get_driver();
    driver.shift(&get_motions(), 0);
    assert_eq!(0, driver.get_status().unwrap());
  }

  #[test]
  fn shift_n4() {
    let mut driver = get_driver();
    driver.shift(&get_motions(), -4);
    assert_eq!(0, driver.get_status().unwrap());
  }

  #[test]
  fn shift_3() {
    let mut driver = get_driver();
    driver.shift(&get_motions(), 3);
    assert_eq!(3, driver.get_status().unwrap());
  }

  #[test]
  fn shift_max_p2() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.shift(&mots, (mots.len() + 2) as isize);
    assert_eq!(mots.len() as isize, driver.get_status().unwrap());
  }

  #[test]
  fn goto() {
    let mut driver = get_driver();
    driver.goto(&get_motions(), 0);
    assert_eq!(0, driver.get_status().unwrap());
  }

  #[test]
  fn goto_n5() {
    let mut driver = get_driver();
    driver.goto(&get_motions(), -5);
    assert_eq!(0, driver.get_status().unwrap());
  }

  #[test]
  fn goto_max_p2() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.goto(&mots, (mots.len() + 2) as isize);
    assert_eq!(mots.len() as isize, driver.get_status().unwrap());
  }

  #[test]
  fn goto_3() {
    let mut driver = get_driver();
    driver.goto(&get_motions(), 3);
    assert_eq!(3, driver.get_status().unwrap());
  }

  #[test]
  fn goto_6_n1() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.goto(&mots, 6);
    driver.goto(&mots, -1);
    assert_eq!(0, driver.get_status().unwrap());
  }

  #[test]
  fn goto_2_reset() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.goto(&mots, 2);
    driver.reset(&mots);
    assert_eq!(2, driver.get_status().unwrap());
  }

  #[test]
  fn goto_max_p2_reset() {
    let mut driver = get_driver();
    let mots = get_motions();
    driver.goto(&mots, (mots.len() + 2) as isize);
    driver.reset(&mots);
    assert_eq!(mots.len() as isize, driver.get_status().unwrap());
  }

  #[test]
  fn reset() {
    let mut driver = get_driver();
    driver.reset(&get_motions());
    assert_eq!(0, driver.get_status().unwrap());
  }
}
