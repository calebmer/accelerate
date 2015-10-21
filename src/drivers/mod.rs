pub trait Driver{
    fn new(target: &'static str) -> Self;
    fn get_status(&self) -> isize;
    fn set_status(&mut self, status: isize);
    fn execute(&self, motion: &'static str);
}

pub struct TestDriver{
    target: &'static str,
    status: isize,
}

impl Driver for TestDriver {
    fn new(target: &'static str) -> Self {
        TestDriver { target: target, status: 0 }
    }

    fn get_status(&self) -> isize{
        println!("Get Status of {0}, which is \'{1}\'", self.target, self.status);
        return self.status
    }

    fn set_status(&mut self, status: isize){
        println!("Set Status to \'{}\'", status);
        self.status = status;
    }

    fn execute(&self, motion: &'static str){
        println!("I am a {1} that says \'{0}\'", motion, self.target);
    }
}

#[cfg(feature = "driver-postgres")]
pub mod driver_postgres;
