use std::io::prelude::*;
use std::fs::File;
use ansi_term::Colour::*;
use error::Error;
use motions::Motion;
use driver::Driver;

#[derive(Eq, PartialEq, Debug)]
struct State {
  applied: Vec<Motion>,
  unapplied: Vec<Motion>,
}

pub struct Accelerator {
  driver: Box<Driver>,
  state: State,
}

impl Accelerator {
  pub fn new(driver: Box<Driver>, motions: Vec<Motion>) -> Result<Self, Error> {
    let records = try!(driver.get_records());
    let state = try!(diff_motions(records, motions));
    Ok(Accelerator {
      driver: driver,
      state: state,
    })
  }

  pub fn add(&mut self, mut iterations: usize) -> Result<(), Error> {
    loop {
      // If we have finished our iterations break out.
      if iterations == 0 { break; }
      // Subtract one from our iterations.
      iterations -= 1;
      // Pop off the next motion to be applied.
      if let Some(motion) = self.state.unapplied.pop() {
        // Read the contents of our motion file.
        let mut file = try!(File::open(&motion.add_path));
        let mut transaction = String::new();
        try!(file.read_to_string(&mut transaction));
        // Execute the contents of our motion file.
        try!(self.driver.execute(transaction));
        // Add a record that we executed the motion.
        try!(self.driver.add_record(&motion.name));
        // Print our success!
        println!("{} {}", Green.bold().paint("Add"), motion);
        // Update our state to reflect that we’ve applied this motion.
        self.state.applied.push(motion);
      }
      // If we have no more actions to apply, break out.
      else {
        break;
      }
    }
    Ok(())
  }

  pub fn sub(&mut self, mut iterations: usize) -> Result<(), Error> {
    loop {
      // If we have finished our iterations break out.
      if iterations == 0 { break; }
      // Subtract one from our iterations.
      iterations -= 1;
      // Pop off the next motion to be applied.
      if let Some(motion) = self.state.applied.pop() {
        // Read the contents of our motion file.
        let mut file = try!(File::open(&motion.sub_path));
        let mut transaction = String::new();
        try!(file.read_to_string(&mut transaction));
        // Execute the contents of our motion file.
        try!(self.driver.execute(transaction));
        // Add a record that we executed the motion.
        try!(self.driver.sub_record(&motion.name));
        // Print our success!
        println!("{} {}", Red.bold().paint("Sub"), motion);
        // Update our state blah blah blah.
        self.state.unapplied.push(motion);
      }
      // If we have no more actions to unapply, break out.
      else {
        break;
      }
    }
    Ok(())
  }

  pub fn applied_count(&self) -> usize {
    self.state.applied.len()
  }
}

fn diff_motions(mut motion_names: Vec<String>, mut motions: Vec<Motion>) -> Result<State, Error> {
  // Make sure we never have more applied motions than we have expected motions.
  if !(motions.len() >= motion_names.len()) {
    return Err(error!(
      "There are {} motions that have been applied which is more than the {} motions we know of.",
      motion_names.len(),
      motions.len()
    ));
  }

  // Create our state variable.
  let mut state = State {
    applied: Vec::new(),
    unapplied: Vec::new(),
  };

  // We have an index for error reporting purposes. The index starts at -1
  // because we increment when the loop starts so it will then become 0.
  let mut index = -1;

  // Reverse our arrays because we want to pop the first items not the last
  // ones.
  motion_names.reverse();
  motions.reverse();

  // Left the infinite loop, begin!
  loop {
    // Increment our index.
    index += 1;
    // Get the motion that was applied…
    if let Some(motion_name) = motion_names.pop() {
      // Get the motion that we actually expect…
      if let Some(motion) = motions.pop() {
        // If the motion we expected was the one that actually got applied—we
        // good, continue after saving this motion.
        if motion_name == motion.name {
          state.applied.push(motion);
        }
        // Otherwise, if the motions are different, something bad happened.
        // We should let the user know.
        else {
          return Err(error!(
            "The '{}' motion we expected is not the same as the '{}' motion that was actually applied at index {}. Try manually applying this motion with `accelerate apply {}`.",
            motion.name,
            motion_name,
            index,
            motion.add_path.display()
          ));
        }
      }
      // Because of our check earlier, there should always be more expected
      // motions than applied motions.
      else {
        unreachable!();
      }
    }
    // If we have no more applied motions, return the rest of our expected
    // motions.
    else {
      // When we append the list, it will be reversed. We want this however
      // because we will be able to pop the next motion to be applied.
      state.unapplied.append(&mut motions);
      return Ok(state);
    }
  }
}

#[cfg(test)]
mod tests {
  use std::mem;
  use std::path::{Path, PathBuf};
  use motions::Motion;
  use driver::test::TestDriver;
  use super::{State, diff_motions, Accelerator};

  fn pb(path: &str) -> PathBuf {
    Path::new(path).to_path_buf()
  }

  fn motion_a() -> Motion {
    Motion {
      name: "a".to_string(),
      add_path: pb("a.add"),
      sub_path: pb("a.sub"),
    }
  }

  fn motion_b() -> Motion {
    Motion {
      name: "b".to_string(),
      add_path: pb("b.add"),
      sub_path: pb("b.sub"),
    }
  }

  fn motion_c() -> Motion {
    Motion {
      name: "c".to_string(),
      add_path: pb("c.add"),
      sub_path: pb("c.sub"),
    }
  }

  fn motion_foo() -> Motion {
    Motion {
      name: "123456-foo".to_string(),
      add_path: pb("tests/fixtures/basic/123456-foo.add"),
      sub_path: pb("tests/fixtures/basic/123456-foo.sub"),
    }
  }

  fn motion_bar() -> Motion {
    Motion {
      name: "234567-bar".to_string(),
      add_path: pb("tests/fixtures/basic/234567-bar.add"),
      sub_path: pb("tests/fixtures/basic/234567-bar.sub"),
    }
  }

  #[test]
  fn test_diff_motions_extra_names() {
    assert!(diff_motions(vec!["a".to_string(), "b".to_string()], vec![motion_a()]).is_err());
  }

  #[test]
  fn test_diff_motions_unequal() {
    assert!(diff_motions(vec!["c".to_string(), "a".to_string()], vec![motion_a(), motion_b(), motion_c()]).is_err());
  }

  #[test]
  fn test_diff_motions_all() {
    assert_eq!(diff_motions(
      vec!["a".to_string(), "b".to_string(), "c".to_string()],
      vec![motion_a(), motion_b(), motion_c()]
    ).unwrap(), State {
      applied: vec![motion_a(), motion_b(), motion_c()],
      unapplied: vec![],
    });
  }

  #[test]
  fn test_diff_motions_some() {
    assert_eq!(diff_motions(
      vec!["a".to_string(), "b".to_string()],
      vec![motion_a(), motion_b(), motion_c()]
    ).unwrap(), State {
      applied: vec![motion_a(), motion_b()],
      unapplied: vec![motion_c()],
    });
    assert_eq!(diff_motions(
      vec!["a".to_string()],
      vec![motion_a(), motion_b(), motion_c()]
    ).unwrap(), State {
      applied: vec![motion_a()],
      unapplied: vec![motion_c(), motion_b()],
    });
  }

  #[test]
  fn test_diff_motions_none() {
    assert_eq!(diff_motions(
      vec![],
      vec![motion_a(), motion_b(), motion_c()]
    ).unwrap(), State {
      applied: vec![],
      unapplied: vec![motion_c(), motion_b(), motion_a()],
    });
  }

  #[test]
  fn test_accelerator_add_1() {
    let mut accelerator = Accelerator {
      driver: Box::new(TestDriver {
        records: vec![],
        executions: vec![],
      }),
      state: State {
        applied: vec![],
        unapplied: vec![motion_foo(), motion_bar()],
      },
    };

    accelerator.add(1).unwrap();

    let driver: &Box<TestDriver> = unsafe { mem::transmute(&accelerator.driver) };

    assert_eq!(driver.records, vec!["234567-bar".to_string()]);
    assert_eq!(driver.executions, vec!["bar+\n".to_string()]);
    assert_eq!(accelerator.state.applied, vec![motion_bar()]);
    assert_eq!(accelerator.state.unapplied, vec![motion_foo()]);
  }

  #[test]
  fn test_accelerator_add_2() {
    let mut accelerator = Accelerator {
      driver: Box::new(TestDriver {
        records: vec![],
        executions: vec![],
      }),
      state: State {
        applied: vec![],
        unapplied: vec![motion_foo(), motion_bar()],
      },
    };

    accelerator.add(2).unwrap();

    let driver: &Box<TestDriver> = unsafe { mem::transmute(&accelerator.driver) };

    assert_eq!(driver.records, vec!["234567-bar".to_string(), "123456-foo".to_string()]);
    assert_eq!(driver.executions, vec!["bar+\n".to_string(), "foo+\n".to_string()]);
    assert_eq!(accelerator.state.applied, vec![motion_bar(), motion_foo()]);
    assert_eq!(accelerator.state.unapplied, vec![]);
  }

  #[test]
  fn test_accelerator_add_3() {
    let mut accelerator = Accelerator {
      driver: Box::new(TestDriver {
        records: vec![],
        executions: vec![],
      }),
      state: State {
        applied: vec![],
        unapplied: vec![motion_foo(), motion_bar()],
      },
    };

    accelerator.add(3).unwrap();

    let driver: &Box<TestDriver> = unsafe { mem::transmute(&accelerator.driver) };

    assert_eq!(driver.records, vec!["234567-bar".to_string(), "123456-foo".to_string()]);
    assert_eq!(driver.executions, vec!["bar+\n".to_string(), "foo+\n".to_string()]);
    assert_eq!(accelerator.state.applied, vec![motion_bar(), motion_foo()]);
    assert_eq!(accelerator.state.unapplied, vec![]);
  }

  #[test]
  fn test_accelerator_sub_1() {
    let mut accelerator = Accelerator {
      driver: Box::new(TestDriver {
        records: vec!["234567-bar".to_string(), "123456-foo".to_string()],
        executions: vec![],
      }),
      state: State {
        applied: vec![motion_bar(), motion_foo()],
        unapplied: vec![],
      },
    };

    accelerator.sub(1).unwrap();

    let driver: &Box<TestDriver> = unsafe { mem::transmute(&accelerator.driver) };

    assert_eq!(driver.records, vec!["234567-bar".to_string()]);
    assert_eq!(driver.executions, vec!["foo-\n".to_string()]);
    assert_eq!(accelerator.state.applied, vec![motion_bar()]);
    assert_eq!(accelerator.state.unapplied, vec![motion_foo()]);
  }

  #[test]
  fn test_accelerator_sub_2() {
    let mut accelerator = Accelerator {
      driver: Box::new(TestDriver {
        records: vec!["234567-bar".to_string(), "123456-foo".to_string()],
        executions: vec![],
      }),
      state: State {
        applied: vec![motion_bar(), motion_foo()],
        unapplied: vec![],
      },
    };

    accelerator.sub(2).unwrap();

    let driver: &Box<TestDriver> = unsafe { mem::transmute(&accelerator.driver) };

    assert_eq!(driver.records, vec![] as Vec<String>);
    assert_eq!(driver.executions, vec!["foo-\n".to_string(), "bar-\n".to_string()]);
    assert_eq!(accelerator.state.applied, vec![] as Vec<Motion>);
    assert_eq!(accelerator.state.unapplied, vec![motion_foo(), motion_bar()]);
  }

  #[test]
  fn test_accelerator_sub_3() {
    let mut accelerator = Accelerator {
      driver: Box::new(TestDriver {
        records: vec!["234567-bar".to_string(), "123456-foo".to_string()],
        executions: vec![],
      }),
      state: State {
        applied: vec![motion_bar(), motion_foo()],
        unapplied: vec![],
      },
    };

    accelerator.sub(3).unwrap();

    let driver: &Box<TestDriver> = unsafe { mem::transmute(&accelerator.driver) };

    assert_eq!(driver.records, vec![] as Vec<String>);
    assert_eq!(driver.executions, vec!["foo-\n".to_string(), "bar-\n".to_string()]);
    assert_eq!(accelerator.state.applied, vec![] as Vec<Motion>);
    assert_eq!(accelerator.state.unapplied, vec![motion_foo(), motion_bar()]);
  }
}
