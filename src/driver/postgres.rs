extern crate postgres;

use regex::Regex;
use error::Error;
use super::Driver;
use self::postgres::{Connection, SslMode};

const CREATE_SCHEMA_QUERY: &'static str = "create schema if not exists accelerate";
const CREATE_TABLE_QUERY: &'static str = "create table if not exists accelerate.record (name text not null)";
const GET_RECORDS_QUERY: &'static str = "select name from accelerate.record order by name";
const ADD_RECORD_QUERY: &'static str = "insert into accelerate.record (name) values ($1)";
const SUB_RECORD_QUERY: &'static str = "delete from accelerate.record where name = $1";

pub struct PostgresDriver {
  connection: Connection,
}

impl PostgresDriver {
  pub fn connect(conn_string: &str) -> Result<Self, Error> {
    let connection = try!(Connection::connect(conn_string, SslMode::None));

    try!(connection.execute(CREATE_SCHEMA_QUERY, &[]));
    try!(connection.execute(CREATE_TABLE_QUERY, &[]));

    Ok(PostgresDriver {
      connection: connection,
    })
  }

  pub fn will_accept_connection(conn_string: &str) -> bool {
    let conn_string_re = Regex::new(r"^(pg|postgres|postgresql)://").unwrap();
    conn_string_re.is_match(conn_string)
  }
}

impl Driver for PostgresDriver {
  fn get_records(&self) -> Result<Vec<String>, Error> {
    Ok(try!(self.connection.query(GET_RECORDS_QUERY, &[])).iter().map(|row| row.get(0)).collect())
  }

  fn add_record(&mut self, record: &str) -> Result<(), Error> {
    let rows_updated = try!(self.connection.execute(ADD_RECORD_QUERY, &[&record]));
    if rows_updated != 1 {
      Err(error!("The number of rows added to the record table was {}, only 1 should have been added.", rows_updated))
    } else {
      Ok(())
    }
  }

  fn sub_record(&mut self, record: &str) -> Result<(), Error> {
    let rows_updated = try!(self.connection.execute(SUB_RECORD_QUERY, &[&record]));
    if rows_updated != 1 {
      Err(error!("The number of rows removed was {}, only 1 should have been removed.", rows_updated))
    } else {
      Ok(())
    }
  }

  fn execute(&mut self, query: String) -> Result<(), Error> {
    try!(self.connection.batch_execute(&query));
    Ok(())
  }
}
