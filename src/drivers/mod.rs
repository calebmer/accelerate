pub trait Driver{
    fn new(target: String) -> Self;
    fn get_status(&self) -> isize;
    fn set_status(&mut self, status: isize)-> &mut Self;
    fn execute(&self, motion: &String) -> &Self;
}

#[cfg(feature = "driver-postgres")]
pub mod postgres;
