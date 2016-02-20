pub mod driver;
pub mod motions;
pub mod operation;
pub mod drivers;

#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand, ArgMatches, AppSettings};
use std::io::prelude::*;
use std::io::stdin;
use std::process;
use driver::Driver;
use motions::Motion;

fn main() {
  // Create what is used to interpretate user input from the CLI
  let matches = {
    App::new("accelerate")
    .version(crate_version!())
    .author("Caleb Meredith <calebmeredith8@gmail.com>\nVictor M. Suarez <svmnotn@gmail.com>")
    .about("Accelerate back and forth for your database schema or other in-place systems")
    .settings(&[
      AppSettings::GlobalVersion,
      AppSettings::VersionlessSubcommands,
      AppSettings::UnifiedHelpMessage,
      AppSettings::ArgRequiredElseHelp
    ])
    .args(&[
      Arg::with_name("url").takes_value(true).required(true).help("The url to the database which will be accelerated"),
      Arg::with_name("directory").long("directory").short("d").takes_value(true).value_name("PATH").help("The directory holding the motions (defaults to the current dir)"),
      Arg::with_name("yes").long("yes").short("y").help("Bypass the confirmation step for running subtraction motions")
    ])
    .subcommands(vec![
      SubCommand::with_name("ls").about("Lists all motions in the directory").arg(Arg::with_name("verbose").short("v").help("Display all of the motions information.")),
      SubCommand::with_name("create").about("Create a new motion using the template").arg(Arg::with_name("name").takes_value(true).required(true).help("The name to use for the new motion.")),
      SubCommand::with_name("add").about("Adds n motions").arg(Arg::with_name("n").takes_value(true).help("How many motions to add")),
      SubCommand::with_name("sub").about("Subtracts n motions").arg(Arg::with_name("n").takes_value(true).help("How many motions to sub")),
      SubCommand::with_name("redo").about("Subtracts then adds the last motion"),
      SubCommand::with_name("goto").about("Go to the nth motion").arg(Arg::with_name("n").takes_value(true).required(true).help("Motion to accelerate to")),
      SubCommand::with_name("up").about("Adds all remaining motions"),
      SubCommand::with_name("down").about("Subtracts all previous motions"),
      SubCommand::with_name("reset").about("Subtracts then adds all previous motions")
    ])
    .get_matches()
  };

  // Get all the specified variables or set them to their default values.
  let target = value_t_or_exit!(matches.value_of("url"), String);
  let directory = value_t!(matches.value_of("directory"), String).unwrap_or(".".to_string());
  let mots = motions::discover(&directory);

  // TODO Adquire driver properly!
  let mut driver = get_driver(target);

  // Go through and find what matched.
  let result = match matches.subcommand() {
    ("up", Some(_)) => driver.up(&mots),
    ("ls", Some(m)) => Ok(ls(directory, mots, m.is_present("long"))),
    ("create", Some(m)) => motions::create(directory, mots, value_t_or_exit!(m.value_of("name"), String)),
    ("add", Some(m)) => driver.shift(&mots, value_t!(m.value_of("n"), isize).unwrap_or(1)),
    (cmd, m) => gate((cmd, m), driver, mots, matches.is_present("yes"))
  };

  // Handle any generated errors.
  match result {
    Ok(_) => (),
    Err(err) => panic!(err.to_string())
  }
}

fn gate<D: Driver>(matches: (&str, Option<&ArgMatches>), mut driver: Box<D>, mots: Vec<Motion>, gate: bool) -> Result<(), D::E> {
  if !gate {
    println!("You might remove information by doing this action.\nDo you wish to continue? (Y/N)");
    let stdin = stdin();
    for line in stdin.lock().lines() {
      let s: &str = &line.unwrap();
      match s {
        "Yes" | "yes" | "Y" | "y" | "yup" => {
          println!("Continuing at your own risk.");
          break;
        }
        "No" | "no" | "N" | "n" | "nope" => {
          println!("Exiting.");
          process::exit(0);
        }
        _ => {
          println!("Could not understand response, Exiting.");
          process::exit(0);
        }
      }
    }
  }
  match matches {
    ("down", _) => driver.down(&mots),
    ("redo", _) => driver.redo(&mots),
    ("reset", _) => driver.reset(&mots),
    ("sub", Some(m)) => driver.shift(&mots, value_t!(m.value_of("n"), isize).unwrap_or(1) * -1),
    ("shift", Some(m)) => driver.shift(&mots, value_t_or_exit!(m.value_of("n"), isize)),
    ("goto", Some(m)) => driver.goto(&mots, value_t_or_exit!(m.value_of("n"), isize)),
    _ => {
      println!("Nothing to do!\nRe-run with --help for more information");
      Ok(())
    }
  }
}

fn get_driver(target: String) -> Box<drivers::default::DefaultDriver> {
  Box::new(drivers::default::DefaultDriver::new(target))
}

fn ls(dir: String, mots: Vec<Motion>, long: bool) {
  println!("{} contains: {} motions\n", dir, mots.len());
  for mot in mots {
    if long {
      println!("{},", mot);
    } else {
      println!("{}", mot.name);
    }
  }
}
