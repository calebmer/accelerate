#![allow(unused_parens)]

extern crate ansi_term;
extern crate clap;
extern crate regex;

#[macro_use]
mod error;
mod motions;
mod accelerator;
mod driver;

use std::env;
use std::path::Path;
use ansi_term::Colour::*;
use clap::{App, Arg, SubCommand};
use clap::AppSettings::*;
use error::Error;
use accelerator::Accelerator;
use driver::Driver;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
  run().unwrap();
}

fn run() -> Result<(), Error> {
  let directory_arg = (
    Arg::with_name("directory")
    .help("The directory in which accelerate will look for motions, can also be set with ACCELERATE_DIRECTORY")
    .short("d")
    .long("directory")
    .value_name("PATH")
  );

  let driver_args = [
    Arg::with_name("database")
    .help("The connection string to connecting to your database, can also be set with ACCELERATE_DATABASE")
    .short("b")
    .long("database")
    .value_name("STRING"),
    Arg::with_name("driver_name")
    .help("The driver Accelerate will use to execute your motions, can also be set with ACCELERATE_DRIVER")
    .short("r")
    .long("driver")
    .value_name("NAME")
    .possible_values(&["postgres"])
  ];

  let matches = (
    App::new("Accelerate")
    .bin_name("accelerate")
    .version(VERSION)
    .author("Caleb Meredith <calebmeredith8@gmail.com>")
    .about("Accelerate your databases back and forth through easy to manage migration files")
    .settings(&[
      ArgRequiredElseHelp,
      GlobalVersion,
      VersionlessSubcommands,
      UnifiedHelpMessage,
      SubcommandRequiredElseHelp,
      AllowLeadingHyphen
    ])
    .subcommand(
      SubCommand::with_name("ls")
      .about("Lists all of your available motions")
      .arg(&directory_arg)
    )
    .subcommand(
      SubCommand::with_name("status")
      .about("Informs you about the status of all your motions in the database")
      .arg(&directory_arg)
      .args(&driver_args)
    )
    .subcommand(
      SubCommand::with_name("create")
      .about("Creates a new motion using your defined template")
      .arg(&directory_arg)
      .arg(
        Arg::with_name("name")
        .help("The name (with directory path) of the motion you want to create. A timestamp will automatically be added to the last segment")
        .required(true)
        .value_name("NAME")
      )
    )
    .subcommand(
      SubCommand::with_name("add")
      .about("Will add `n` motions to the driver")
      .arg(&directory_arg)
      .args(&driver_args)
      .arg(
        Arg::with_name("n")
        .help("The number of motions to add to the driver")
        .value_name("N")
        .default_value("1")
      )
    )
    .subcommand(
      SubCommand::with_name("sub")
      .about("Will sub `n` motions in the driver")
      .arg(&directory_arg)
      .args(&driver_args)
      .arg(
        Arg::with_name("n")
        .help("The number of motions to sub in the driver")
        .value_name("N")
        .default_value("1")
      )
    )
    .subcommand(
      SubCommand::with_name("up")
      .about("Will add all motions that have not yet been applied to the database")
      .arg(&directory_arg)
      .args(&driver_args)
    )
    .subcommand(
      SubCommand::with_name("down")
      .about("Will sub all motions that have been applied in the database")
      .arg(&directory_arg)
      .args(&driver_args)
    )
    .subcommand(
      SubCommand::with_name("redo")
      .about("Will sub and then add just the last motion")
      .arg(&directory_arg)
      .args(&driver_args)
    )
    .subcommand(
      SubCommand::with_name("reset")
      .about("Will sub all motions that have been applied in the database and then add all of the motions available")
      .arg(&directory_arg)
      .args(&driver_args)
    )
  ).get_matches();

  let subcommand_name = matches.subcommand_name().unwrap();
  let matches = matches.subcommand_matches(subcommand_name).unwrap();

  let directory_env = env::var("ACCELERATE_DIRECTORY").ok();
  let driver_name_env = env::var("ACCELERATE_DRIVER").ok();
  let database_env = env::var("ACCELERATE_DATABASE").ok();

  let motions = || {
    let directory = matches.value_of("directory").or(directory_env.as_ref().map(|s| s.as_str())).unwrap_or(".");
    motions::find(&Path::new(directory))
  };

  let driver = || {
    let driver_name = matches.value_of("driver_name").or(driver_name_env.as_ref().map(|s| s.as_str()));
    let database = matches.value_of("database").or(database_env.as_ref().map(|s| s.as_str()));
    let database = try!(database.ok_or(error!("A database connection string is required and none was found in either the command line arguments or the environment variable `ACCELERATE_DATABASE`.")));
    driver::get(driver_name, database)
  };

  let accelerator = || Accelerator::new(try!(driver()), try!(motions()));

  match subcommand_name {
    "ls" => {
      for motion in try!(motions()) {
        println!("{}", motion);
      }
    },
    "status" => {
      let driver = try!(driver());
      let records = try!(driver.get_records());
      for motion in try!(motions()) {
        if records.contains(&motion.name) {
          println!("{} {}", Green.bold().paint("✔"), motion);
        } else {
          println!("{} {}", Red.bold().paint("𝙭"), motion);
        }
      }
    },
    "create" => {
      unimplemented!();
    },
    "add" => {
      let mut accelerator = try!(accelerator());
      let n = try!(matches.value_of("n").unwrap_or("1").parse::<usize>());
      try!(accelerator.add(n));
    },
    "sub" => {
      let mut accelerator = try!(accelerator());
      let n = try!(matches.value_of("n").unwrap_or("1").parse::<usize>());
      try!(accelerator.sub(n));
    },
    "up" => {
      let mut accelerator = try!(accelerator());
      try!(accelerator.add(usize::max_value()));
    },
    "down" => {
      let mut accelerator = try!(accelerator());
      try!(accelerator.sub(usize::max_value()));
    },
    "redo" => {
      let mut accelerator = try!(accelerator());
      try!(accelerator.sub(1));
      try!(accelerator.add(1));
    },
    "reset" => {
      let mut accelerator = try!(accelerator());
      let applied = accelerator.applied_count();
      try!(accelerator.sub(usize::max_value()));
      try!(accelerator.add(applied));
    },
    _ => unreachable!(),
  }

  Ok(())
}
