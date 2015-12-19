#![allow(dead_code, unused_variables, unused_imports)]
pub mod accelerator;
pub mod motions;
pub mod drivers;
#[cfg(test)]
mod tests;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

use drivers::Driver;
use motions::Motion;

fn main() {
  let matches = App::new("accelerate")
                  .version(&crate_version!())
                  .global_version(true)
                  .unified_help_message(true)
                  .author("Caleb Meredith <calebmeredith8@gmail.com>\nVictor M. Suarez <svmnotn@gmail.com>")
                  .about("Accelerate back and forth through time for your database or other in-place systems")
                  .arg(Arg::with_name("target")
                         .short("t")
                         .long("target")
                         .help("the targeted url to accelerate")
                         .takes_value(true)
                         .required(true))
                  .arg(Arg::with_name("directory")
                         .short("d")
                         .long("directory")
                         .help("the directory holding the motions")
                         .takes_value(true))
                  .subcommand(SubCommand::with_name("ls").about("list all motions to be used"))
                  .subcommand(SubCommand::with_name("redo").about("subtract then add the last motion"))
                  .subcommand(SubCommand::with_name("up").about("add all remaining motions"))
                  .subcommand(SubCommand::with_name("down").about("subtract all previous motions"))
                  .subcommand(SubCommand::with_name("reset").about("subtract then add all previous motions"))
                  .subcommand(SubCommand::with_name("create")
                                .about("create a new motion named <name> using the template")
                                .arg(Arg::from_usage("<name>")))
                  .subcommand(SubCommand::with_name("add")
                                .about("add n motions (default n = 1)")
                                .arg(Arg::from_usage("[n]")))
                  .subcommand(SubCommand::with_name("sub")
                                .about("subtract n motions (default n = 1)")
                                .arg(Arg::from_usage("[n]")))
                  .subcommand(SubCommand::with_name("goto")
                                .about("go to the nth motion")
                                .arg(Arg::from_usage("<n>")))
                  .get_matches();

  let target = matches.value_of("target").unwrap();
  let directory = matches.value_of("directory").unwrap_or(".");

  let mots = motions::get(&directory.to_string());
  // TODO Adquire driver properly!
  let mut driver = drivers::DefaultDriver::new(target.to_string());

  if let Some(_) = matches.subcommand_matches("ls") {
    ls(&mots);
  }
  if let Some(matches) = matches.subcommand_matches("create") {
    create(directory.to_string(),
           matches.value_of("name").unwrap().to_string());
  }
  if let Some(_) = matches.subcommand_matches("redo") {
    accelerator::redo(&mut driver, &mots);
  }
  if let Some(_) = matches.subcommand_matches("up") {
    accelerator::up(&mut driver, &mots);
  }
  if let Some(_) = matches.subcommand_matches("down") {
    accelerator::down(&mut driver, &mots);
  }
  if let Some(_) = matches.subcommand_matches("reset") {
    accelerator::reset(&mut driver, &mots);
  }
  if let Some(m) = matches.subcommand_matches("add") {
    accelerator::shift(&mut driver,
                       &mots,
                       value_t!(m.value_of("n"), isize).unwrap_or(1));
  }
  if let Some(m) = matches.subcommand_matches("sub") {
    accelerator::shift(&mut driver,
                       &mots,
                       value_t!(m.value_of("n"), isize).unwrap_or(-1));
  }
  if let Some(m) = matches.subcommand_matches("goto") {
    if let Ok(n) = m.value_of("n").unwrap().parse() {
      accelerator::goto(&mut driver, &mots, n);
    } else {
      println!("Error parsing the number argument for goto!");
    }
  }
}

fn ls(mots: &Vec<Motion>) {
  for mot in mots {
    println!("{}", mot.name);
  }
}

// TODO Implement
fn create(directory: String, name: String) {}
