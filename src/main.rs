extern crate clap;
extern crate regex;

mod error;
mod motions;
mod accelerator;
mod driver;

use clap::{App, Arg, SubCommand};
use clap::AppSettings::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
  let matches = {
    let directory_arg = {
      Arg::with_name("directory")
      .help("The directory in which accelerate will look for motions")
      .short("d")
      .long("directory")
      .value_name("PATH")
      .default_value(".")
    };
    let conn_str_arg = {
      Arg::with_name("conn_str")
      .help("The connection string to use when connecting to your database")
      .value_name("CONNECTION")
      .required(true)
    };
    let driver_arg = {
      Arg::with_name("driver")
      .help("The driver accelerate will use to execute your motions")
      .short("r")
      .long("driver")
      .value_name("DRIVER")
      .possible_values(&["postgres"])
    };
    let driver_args = &[conn_str_arg, driver_arg];

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
    .arg(directory_arg)
    .subcommand(
      SubCommand::with_name("ls")
      .about("Lists all of your available motions")
    )
    .subcommand(
      SubCommand::with_name("status")
      .about("Informs you about the status of all your motions in the database")
      .args(driver_args)
    )
    .subcommand(
      SubCommand::with_name("create")
      .about("Creates a new motion using your defined template")
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
      .args(driver_args)
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
      .args(driver_args)
      .arg(
        Arg::with_name("n")
        .help("The number of motions to sub in the driver")
        .value_name("N")
        .default_value("1")
      )
    )
    .subcommand(
      SubCommand::with_name("redo")
      .about("Will sub and then add just the last motion")
      .args(driver_args)
    )
    .subcommand(
      SubCommand::with_name("up")
      .about("Will add all motions that have not yet been applied to the database")
      .args(driver_args)
    )
    .subcommand(
      SubCommand::with_name("down")
      .about("Will sub all motions that have been applied in the database")
      .args(driver_args)
    )
    .subcommand(
      SubCommand::with_name("reset")
      .about("Will sub all motions that have been applied in the database and then add all of the motions available")
      .args(driver_args)
    )
  }.get_matches();
}
