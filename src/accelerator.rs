//! This module holds all of the acceleration logic. We don't care about
//! driver implementations in this bit, we only care about moving through a
//! series of motions (driver deltas) to get the final driver state.

use drivers::Driver;
use motions::Motion;
use operation::Operation;
use operation::Operation::*;

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
fn execute<D: Driver>(driver: &mut Box<D>, motions: &Vec<Motion>, mut start: isize, mut finish: isize) -> Result<(), D::E> {
  if start != finish {
    start = clamp(start, 0, motions.len() as isize);
    finish = clamp(finish, 0, motions.len() as isize);
    let operation = Operation::get(finish, start);
    let mut i = start;
    loop {
      if i == finish {
        break;
      }

      if operation == Operation::Sub {
        i += Operation::sub();
      }

      match driver.execute(&motions[i as usize].add) {
        Ok(_) => (),
        // Handle execution error by first trying to set the new status.
        Err(err) => { try!(driver.set_status(i)); return Err(err) }
      }

      if operation == Operation::Add {
        i += Operation::add();
      }
    }
    return driver.set_status(i);
  }
  Ok(())
}

/// Starts at the current driver status and executes n motions in either the
/// "add" or "sub" direction.
pub fn shift<D: Driver>(driver: &mut Box<D>, motions: &Vec<Motion>, n: isize) -> Result<(), D::E> {
  let start = try!(driver.get_status());
  let finish = clamp(start + n, 0, motions.len() as isize);
  execute(driver, motions, start, finish)
}

/// Executes as many motions as necessary to move the driver from its current
/// status to a certain state.
pub fn goto<D: Driver>(driver: &mut Box<D>, motions: &Vec<Motion>, finish: isize) -> Result<(), D::E> {
  let status = try!(driver.get_status());
  execute(driver, motions, status, finish)
}

/// "sub's" the last motion then "add's" it back.
pub fn redo<D: Driver>(driver: &mut Box<D>, motions: &Vec<Motion>) -> Result<(), D::E> {
  shift(driver, motions, -1).and_then(|_| shift(driver, motions, 1))
}

/// Applies all remaining "add" motions to the driver.
pub fn up<D: Driver>(driver: &mut Box<D>, motions: &Vec<Motion>) -> Result<(), D::E> {
  let last = motions.len() as isize;
  goto(driver, motions, last)
}

/// Applies all remaining "sub" motions to the driver.
pub fn down<D: Driver>(driver: &mut Box<D>, motions: &Vec<Motion>) -> Result<(), D::E> {
  goto(driver, motions, 0)
}

/// Applies all remaining "sub" motions to the driver before adding them all
/// the subbed motions back.
pub fn reset<D: Driver>(driver: &mut Box<D>, motions: &Vec<Motion>) -> Result<(), D::E> {
  let status = try!(driver.get_status());
  execute(driver, motions, status, 0).and_then(|_| execute(driver, motions, 0, status))
}
