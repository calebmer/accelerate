use drivers::Driver as DriverTrait;

type MaybeError = super::MaybeError;

pub struct Driver {
  target: String,
  status: isize,
}

impl Driver {
  pub fn new(target: String) -> Self {
    println!("Creating a new default driver with target: {}", target);
    Driver {
      target: target,
      status: 0,
    }
  }
}

impl DriverTrait for Driver {
  fn get_status(&self) -> isize {
    println!("The Status of {0}\n\t is {1}", self.target, self.status);
    return self.status;
  }

  fn set_status(&mut self, status: isize) -> MaybeError {
    println!("Set Status of {0}\n\t from {1}\n\t to {2}", self.target, self.status, status);
    self.status = status;
    Ok(())
  }

  fn execute(&self, motion: &String) -> MaybeError {
    println!("I am a {0}\n\t that says {1}\n\t while at {2}", self.target, motion, self.status);
    Ok(())
  }
}
