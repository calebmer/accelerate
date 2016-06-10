#[cfg(test)]
pub mod tests;

use error::Error;

pub trait Driver {
  fn get_records(&self) -> Result<Vec<String>, Error>;
  fn add_record(&mut self, record: &str) -> Result<(), Error>;
  fn sub_record(&mut self, record: &str) -> Result<(), Error>;
  fn execute(&mut self, transaction: String) -> Result<(), Error>;
}
