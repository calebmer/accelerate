use error::Error;
use super::Driver;

#[derive(Debug)]
pub struct TestDriver {
  pub records: Vec<String>,
  pub executions: Vec<String>,
}

impl Driver for TestDriver {
  fn get_records(&self) -> Result<Vec<String>, Error> {
    Ok(self.records.clone())
  }

  fn add_record(&mut self, record: &str) -> Result<(), Error> {
    self.records.push(record.to_string());
    Ok(())
  }

  fn sub_record(&mut self, record: &str) -> Result<(), Error> {
    if let Some(index) = self.records.iter().position(|r| r == record) {
      self.records.remove(index);
      Ok(())
    } else {
      Err(error!("Record '{}' could not be removed because it was never applied.", record))
    }
  }

  fn execute(&mut self, transaction: String) -> Result<(), Error> {
    self.executions.push(transaction);
    Ok(())
  }
}
