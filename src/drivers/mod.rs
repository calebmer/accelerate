pub trait Driver{
    fn new(target: &'static str) -> Self;
    fn get_status(&self) -> isize;
    fn set_status(&mut self, status: isize)-> &mut Self;
    fn execute(&self, motion: &'static str) -> &Self;
}

#[cfg(feature = "driver-postgres")]
pub mod postgres;
