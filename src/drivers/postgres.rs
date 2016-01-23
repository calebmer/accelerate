// #![cfg(feature = "driver-postgres")]
//
// extern crate postgres;
//
// use std::error::Error;
// use self::postgres::{Connection, SslMode};
// use self::postgres::error::ConnectError;
// use drivers::Driver as DriverTrait;
//
// type MaybeError = super::MaybeError;
//
// const CREATE_SCHEMA_QUERY: &'static str = "CREATE SCHEMA IF NOT EXISTS accelerate";
// const CREATE_TABLE_QUERY: &'static str = "CREATE TABLE IF NOT EXISTS accelerate.state ( status INTEGER, inserted TIMESTAMP DEFAULT CURRENT_TIMESTAMP )";
// const SELECT_STATUS_QUERY: &'static str = "SELECT status FROM accelerate.state ORDER BY inserted DESC LIMIT 1";
// const SET_STATUS_QUERY: &'static str = "INSERT INTO accelerate.state (status) VALUES ($1)";
//
// pub struct Driver {
//   conn: Connection,
// }
//
// impl Driver {
//   pub fn new(target: String) -> Result<Self, ConnectError> {
//     Connection::connect(&*target, &SslMode::None).map(|conn| Driver { conn: conn })
//   }
// }
//
// // TODO Implement
// impl DriverTrait for Driver {
//   fn init_state(&self) -> MaybeError {
//     self.conn
//         .execute(CREATE_SCHEMA_QUERY, &[])
//         .and_then(|_| self.conn.execute(CREATE_TABLE_QUERY, &[]))
//         .map(|_| ())
//         .map_err(|error| error.description().to_owned())
//   }
//
//   fn get_status(&self) -> isize {
//     self.conn.execute(SELECT_STATUS_QUERY);
//     return 0;
//   }
//
//   fn set_status(&mut self, status: isize) -> MaybeError {
//     self.conn
//         .execute(SET_STATUS_QUERY, &[status])
//         .map(|_| ())
//         .map_err(|error| error.description().to_owned())
//   }
//
//   // fn execute(&self, motion: &String) {}
// }
