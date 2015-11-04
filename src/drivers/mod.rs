pub trait Driver{
    fn new(target: String) -> Self;
    fn get_status(&self) -> isize;
    fn set_status(&mut self, status: isize) -> &mut Self;
    fn execute(&self, motion: &String) -> &Self;
}

pub struct DefaultDriver {
    target: String,
    status: isize,
}

impl Driver for DefaultDriver {
    fn new(target: String) -> Self {
        println!("Creating a new DefaultDriver with target: {}", target);
        DefaultDriver {
            target: target,
            status: 0,
        }
    }

    fn get_status(&self) -> isize {
        println!("The Status of {0}\n\t is {1}", self.target, self.status);
        return self.status;
    }

    fn set_status(&mut self, status: isize) -> &mut Self {
        println!("Set Status of {0}\n\t from {1}\n\t to {2}",
                 self.target,
                 self.status,
                 status);
        self.status = status;
        self
    }

    fn execute(&self, motion: &String) -> &Self {
        println!("I am a {0}\n\t that says {1}\n\t while at {2}",
                 self.target,
                 motion,
                 self.status);
        self
    }
}

#[cfg(feature = "driver-postgres")]
pub mod postgres;
