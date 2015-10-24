pub trait Driver{
    fn new(target: &'static str) -> Self;
    fn get_status(&self) -> isize;
    fn set_status(&mut self, status: isize);
    fn execute(&self, motion: &'static str);
}

#[cfg(feature = "driver-postgres")]
pub mod postgres;
