pub trait Driver{
    fn new(target: String) -> Self;
    fn get_status(&self) -> isize;
    fn set_status(&mut self, status: isize)-> &mut Self;
    fn execute(&self, motion: &String) -> &Self;
}

pub struct DefaultDriver{
    target: String,
    status: isize,
}

impl Driver for DefaultDriver {
    fn new(target: String) -> Self {
        DefaultDriver { target: target, status: 0 }
    }

    fn get_status(&self) -> isize{
        return self.status
    }

    fn set_status(&mut self, status: isize)-> &mut Self{
        self.status = status;
        self
    }

    fn execute(&self, motion: &String)-> &Self{
        println!("Running \"{0}\" @ {1}", motion, self.target);
        self
    }
}

#[cfg(feature = "driver-postgres")]
pub mod postgres;
