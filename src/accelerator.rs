use drivers::Driver;
use motions::Motion;

fn clamp(n: isize, min: isize, max: isize) -> isize {
  if n < min {
    return min;
  }
  if n > max {
    return max;
  }
  n
}

fn execute<D: Driver>(driver: &mut D, motions: &Vec<Motion>, mut start: isize, mut finish: isize) {
  if start != finish {
    start = clamp(start, 0, motions.len() as isize);
    finish = clamp(finish, 0, motions.len() as isize);
    let operation = Operation::get(finish, start);
    match operation {
      Operation::Add(op) => {
        let mut i = start;
        loop {
          if i == finish {
            break;
          }
          driver.execute(&motions[i as usize].add);
          i += op;
        }
        driver.set_status(i);
      }
      Operation::Sub(op) => {
        let mut i = start;
        loop {
          if i == finish {
            break;
          }
          i += op;
          driver.execute(&motions[i as usize].sub);
        }
        driver.set_status(i);
      }
    }
  }
}

pub fn shift<D: Driver>(driver: &mut D, motions: &Vec<Motion>, n: isize) {
  let start = driver.get_status();
  let finish = clamp(start + n, 0, motions.len() as isize);
  execute(driver, motions, start, finish);
}

pub fn goto<D: Driver>(driver: &mut D, motions: &Vec<Motion>, finish: isize) {
  let status = driver.get_status();
  execute(driver, motions, status, finish);
}

pub fn redo<D: Driver>(driver: &mut D, motions: &Vec<Motion>) {
  shift(driver, motions, -1);
  shift(driver, motions, 1);
}

pub fn up<D: Driver>(driver: &mut D, motions: &Vec<Motion>) {
  let last = motions.len() as isize;
  goto(driver, motions, last);
}

pub fn down<D: Driver>(driver: &mut D, motions: &Vec<Motion>) { goto(driver, motions, 0); }

pub fn reset<D: Driver>(driver: &mut D, motions: &Vec<Motion>) {
  let status = driver.get_status();
  execute(driver, motions, status, 0);
  execute(driver, motions, 0, status);
}

pub enum Operation {
  Add(isize),
  Sub(isize),
}

impl Operation {
  fn get(finish: isize, start: isize) -> Self {
    if finish < start {
      return Operation::Sub(-1);
    }
    Operation::Add(1)
  }
}
