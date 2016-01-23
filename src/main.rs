#![allow(dead_code, unused_variables, unused_imports)]
pub mod accelerator;
pub mod motions;
pub mod operation;
pub mod drivers;
#[cfg(test)]
mod tests;

#[macro_use]
extern crate clap;
use clap::{App, SubCommand, ArgMatches};
use std::io::prelude::*;
use std::io::stdin;
use std::process;

use drivers::Driver;
use motions::Motion;

// TODO: `Err` should not be a string and rather something which implements the
// `std::error::Error` trait.
pub type MaybeError = std::result::Result<(), String>;

fn main() {
  // Create what is used to interpretate user input from the CLI
  let matches = App::new("accelerate")
    .version(&crate_version!())
    .global_version(true)
    .unified_help_message(true)
    .author("Caleb Meredith <calebmeredith8@gmail.com>\nVictor M. Suarez <svmnotn@gmail.com>")
    .about("Accelerate back and forth through time for your database or other in-place systems")
    .args_from_usage(
     "<url>                 'the targeted url to accelerate'
      --directory=[path] -d 'the directory holding the motions (defaults to the current dir)'
      --yes              -y 'auto accept the changes, used for subcommands that remove information (USE WITH CAUTION!)'")
    .subcommand(SubCommand::with_name("redo").about("subtracts then adds the last motion"))
    .subcommand(SubCommand::with_name("up").about("adds all remaining motions"))
    .subcommand(SubCommand::with_name("down").about("subtracts all previous motions"))
    .subcommand(SubCommand::with_name("reset").about("subtracts then adds all previous motions"))
    .subcommand(
      SubCommand::with_name("ls")
      .about("lists all motions in the directory")
      .arg_from_usage("--long -l 'Display all motion information'")
    )
    .subcommand(
      SubCommand::with_name("create")
      .about("create a new motion using the template")
      .arg_from_usage("<name> 'the name to use for the new motion'")
    )
    .subcommand(
      SubCommand::with_name("add")
      .about("adds n motions")
      .arg_from_usage("[n] 'how many motions to add, defaults to 1'")
    )
    .subcommand(
      SubCommand::with_name("sub")
      .about("subtracts n motions")
      .arg_from_usage("[n] 'how many motions to substract, defaults to 1'")
    )
    .subcommand(
      SubCommand::with_name("shift")
      .about("goes to the nth motion relative to the current motion")
      .arg_from_usage("--n=<n> 'the amount of motions to move relative to the current one, must be given as a number in the form of --n=[number]'")
    )
    .subcommand(
      SubCommand::with_name("goto")
      .about("go to the nth motion")
      .arg_from_usage("<n> 'the motion to go to, 0 based'")
    )
    .get_matches();

  // Get all the specified variables or set them to their default values
  let target = value_t_or_exit!(matches.value_of("url"), String);
  let directory = value_t!(matches.value_of("path"), String).unwrap_or(".".to_string());
  let mots = motions::discover(&directory);

  // TODO Adquire driver properly!
  let mut driver = get_driver(target);

  // Go through and find what matched
  let result = match matches.subcommand() {
    ("up", Some(_)) => accelerator::up(&mut driver, &mots),
    ("ls", Some(m)) => ls(directory, mots, m.is_present("long")),
    ("create", Some(m)) => motions::create(directory, mots, value_t_or_exit!(m.value_of("name"), String)),
    ("add", Some(m)) => accelerator::shift(&mut driver, &mots, value_t!(m.value_of("n"), isize).unwrap_or(1)),
    (cmd, m) => gate((cmd, m), driver, mots, matches.is_present("yes")),
  };

  // Handle any errors which occured.
  if result.is_err() {
    // TODO: Nice error message.
    panic!(result.unwrap_err())
  }
}

fn gate(matches: (&str, Option<&ArgMatches>), mut driver: Box<Driver>, mots: Vec<Motion>, gate: bool) -> MaybeError {
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
    ("down", _) => accelerator::down(&mut driver, &mots),
    ("redo", _) => accelerator::redo(&mut driver, &mots),
    ("reset", _) => accelerator::reset(&mut driver, &mots),
    ("sub", Some(m)) => accelerator::shift(&mut driver, &mots, value_t!(m.value_of("n"), isize).unwrap_or(1) * -1),
    ("shift", Some(m)) => accelerator::shift(&mut driver, &mots, value_t_or_exit!(m.value_of("n"), isize)),
    ("goto", Some(m)) => accelerator::goto(&mut driver, &mots, value_t_or_exit!(m.value_of("n"), isize)),
    _ => {
      println!("Nothing to do!\nRe-run with --help for more information");
      Ok(())
    }
  }
}

fn get_driver(target: String) -> Box<Driver> {
  let driver = drivers::default::Driver::new(target);
  let error = driver.init_state();

  match error {
    Ok(_) => Box::new(driver),
    Err(msg) => panic!(msg),
  }
}

fn ls(dir: String, mots: Vec<Motion>, long: bool) -> MaybeError {
  println!("{} contains: {} motions\n", dir, mots.len());
  for mot in mots {
    if long {
      println!("{},", mot);
    } else {
      println!("{}", mot.name);
    }
  }
  Ok(())
}
