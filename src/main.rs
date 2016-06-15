#![allow(unused_parens)]

extern crate clap;
extern crate colored;
extern crate regex;

#[macro_use]
mod error;
mod motions;
mod accelerator;
mod driver;

use std::env;
use std::path::Path;
use std::process;
use std::io;
use std::io::prelude::*;
use clap::{App, Arg, SubCommand};
use clap::AppSettings::*;
use colored::Colorize;
use error::Error;
use accelerator::Accelerator;
use driver::Driver;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
  match run() {
    Ok(_) => (),
    Err(error) => println!("{} {}", "Error:".red().bold(), error),
  }
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
    .short("c")
    .long("database")
    .value_name("STRING")
    .use_delimiter(false),

    Arg::with_name("driver_name")
    .help("The driver Accelerate will use to execute your motions, can also be set with ACCELERATE_DRIVER")
    .short("t")
    .long("driver")
    .value_name("NAME")
  ];

  let auto_confirm_arg = (
    Arg::with_name("auto_confirm")
    .help("Automatically confirm when removing information, this should only be used in automated environments")
    .short("y")
    .long("yes")
  );

  let matches = (
    App::new("Accelerate")
    .bin_name("accelerate")
    .version(VERSION)
    .author("Caleb Meredith <calebmeredith8@gmail.com>")
    .about("Accelerate your databases back and forth through easy to manage migration files")
    .settings(&[
      SubcommandRequired,
      GlobalVersion,
      VersionlessSubcommands,
      UnifiedHelpMessage,
      ColoredHelp,
      DeriveDisplayOrder,
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
      .arg(&auto_confirm_arg)
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
      .arg(&auto_confirm_arg)
      .arg(&directory_arg)
      .args(&driver_args)
    )
    .subcommand(
      SubCommand::with_name("redo")
      .about("Will sub and then add just the last motion")
      .arg(&auto_confirm_arg)
      .arg(&directory_arg)
      .args(&driver_args)
    )
    .subcommand(
      SubCommand::with_name("reset")
      .about("Will sub all motions that have been applied in the database and then add all of the motions available")
      .arg(&auto_confirm_arg)
      .arg(&directory_arg)
      .args(&driver_args)
    )
  ).get_matches();

  let subcommand_name = matches.subcommand_name().unwrap();
  let matches = matches.subcommand_matches(subcommand_name).unwrap();
  let auto_confirm = matches.is_present("auto_confirm");

  let directory_env = env::var("ACCELERATE_DIRECTORY").ok();
  let driver_name_env = env::var("ACCELERATE_DRIVER").ok();
  let database_env = env::var("ACCELERATE_DATABASE").ok();

  let directory = || Path::new(matches.value_of("directory").or(directory_env.as_ref().map(|s| s.as_str())).unwrap_or("."));
  let motions = || motions::find(&directory());

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
          println!("{} {}", "✔".green().bold(), motion);
        } else {
          println!("{} {}", "𝙭".red().bold(), motion);
        }
      }
    },
    "create" => {
      try!(motions::create(&directory(), matches.value_of("name").unwrap()));
    },
    "add" => {
      let mut accelerator = try!(accelerator());
      let n = try!(matches.value_of("n").unwrap_or("1").parse::<usize>());
      try!(accelerator.add(n));
    },
    "sub" => {
      if !auto_confirm { try!(confirm()); }
      let mut accelerator = try!(accelerator());
      let n = try!(matches.value_of("n").unwrap_or("1").parse::<usize>());
      try!(accelerator.sub(n));
    },
    "up" => {
      let mut accelerator = try!(accelerator());
      try!(accelerator.add(usize::max_value()));
    },
    "down" => {
      if !auto_confirm { try!(confirm()); }
      let mut accelerator = try!(accelerator());
      try!(accelerator.sub(usize::max_value()));
    },
    "redo" => {
      if !auto_confirm { try!(confirm()); }
      let mut accelerator = try!(accelerator());
      try!(accelerator.sub(1));
      try!(accelerator.add(1));
    },
    "reset" => {
      if !auto_confirm { try!(confirm()); }
      let mut accelerator = try!(accelerator());
      let applied = accelerator.applied_count();
      try!(accelerator.sub(usize::max_value()));
      try!(accelerator.add(applied));
    },
    _ => unreachable!(),
  }

  Ok(())
}

fn confirm() -> Result<(), Error> {
  // Display a warning message.
  println!("{} You may be removing information by proceeding. Do you wish to continue? (y/n)", "Warning:".yellow().bold());
  // Read a line from `stdin`.
  let mut line = String::new();
  let stdin = io::stdin();
  try!(stdin.lock().read_line(&mut line));
  // If the line starts with “y” or “Y” continue, otherwise abort.
  if line.starts_with("y") || line.starts_with("Y") {
    Ok(())
  } else {
    process::exit(0);
  }
}
