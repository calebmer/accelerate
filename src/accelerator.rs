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
fn execute(driver: &mut Box<Driver>, motions: &Vec<Motion>, mut start: isize, mut finish: isize) {
  if start != finish {
    start = clamp(start, 0, motions.len() as isize);
    finish = clamp(finish, 0, motions.len() as isize);
    let operation = Operation::get(finish, start);
    match operation {
      Add => {
        let mut i = start;
        loop {
          if i == finish {
            break;
          }
          driver.execute(&motions[i as usize].add);
          i += Operation::add();
        }
        driver.set_status(i);
      }
      Sub => {
        let mut i = start;
        loop {
          if i == finish {
            break;
          }
          i += Operation::sub();
          driver.execute(&motions[i as usize].sub);
        }
        driver.set_status(i);
      }
    }
  }
}

/// Starts at the current driver status and executes n motions in either the
/// "add" or "sub" direction.
pub fn shift(driver: &mut Box<Driver>, motions: &Vec<Motion>, n: isize) {
  let start = driver.get_status();
  let finish = clamp(start + n, 0, motions.len() as isize);
  execute(driver, motions, start, finish);
}

/// Executes as many motions as necessary to move the driver from its current
/// status to a certain state.
pub fn goto(driver: &mut Box<Driver>, motions: &Vec<Motion>, finish: isize) {
  let status = driver.get_status();
  execute(driver, motions, status, finish);
}

/// "sub's" the last motion then "add's" it back.
pub fn redo(driver: &mut Box<Driver>, motions: &Vec<Motion>) {
  shift(driver, motions, -1);
  shift(driver, motions, 1);
}

/// Applies all remaining "add" motions to the driver.
pub fn up(driver: &mut Box<Driver>, motions: &Vec<Motion>) {
  let last = motions.len() as isize;
  goto(driver, motions, last);
}

/// Applies all remaining "sub" motions to the driver.
pub fn down(driver: &mut Box<Driver>, motions: &Vec<Motion>) { goto(driver, motions, 0); }

/// Applies all remaining "sub" motions to the driver before adding them all
/// the subbed motions back.
pub fn reset(driver: &mut Box<Driver>, motions: &Vec<Motion>) {
  let status = driver.get_status();
  execute(driver, motions, status, 0);
  execute(driver, motions, 0, status);
}
