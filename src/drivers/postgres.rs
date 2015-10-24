#![cfg(feature = "driver-postgres")]
use drivers::Driver as DriverTrait;

pub struct Driver{
    target: &'static str,
}

impl DriverTrait for Driver {
    fn new(target: &'static str) -> Self {
        Driver { target: target }
    }

    fn get_status(&self) -> isize{
        println!("Get Status of {}", self.target);
        return 0
    }

    fn set_status(&mut self, status: isize){
        println!("Set Status to \'{}\'", status);
    }

    fn execute(&self, motion: &'static str){
        println!("I am a thing that says \'{0}\' with {1}", motion, self.target);
    }
}
