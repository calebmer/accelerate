#[cfg(test)]
pub mod test;
#[cfg(feature = "driver-postgres")]
pub mod postgres;

use error::Error;

pub trait Driver {
  fn get_records(&self) -> Result<Vec<String>, Error>;
  fn add_record(&mut self, record: &str) -> Result<(), Error>;
  fn sub_record(&mut self, record: &str) -> Result<(), Error>;
  fn execute(&mut self, query: String) -> Result<(), Error>;
}

pub fn get(driver_name: Option<&str>, conn_str: &str) -> Result<Box<Driver>, Error> {
  if let Some(driver_name) = driver_name {
    get_by_name(driver_name, conn_str)
  } else {
    get_by_conn_str(conn_str)
  }
}

fn get_by_name(driver_name: &str, conn_str: &str) -> Result<Box<Driver>, Error> {
  match driver_name {
    #[cfg(feature = "driver-postgres")]
    "postgres" => Ok(Box::new(try!(postgres::PostgresDriver::connect(conn_str)))),

    _ => Err(error!("Driver for name '{}' could not be found.", driver_name)),
  }
}

fn get_by_conn_str(conn_str: &str) -> Result<Box<Driver>, Error> {
  match () {
    #[cfg(feature = "driver-postgres")]
    () if postgres::PostgresDriver::will_accept_connection(conn_str) =>
      Ok(Box::new(try!(postgres::PostgresDriver::connect(conn_str)))),

    _ => Err(error!("No driver will accept connection string '{}'. Try disabling driver inference by defining the driver type.", conn_str)),
  }
}
