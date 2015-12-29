#![allow(dead_code, unused_variables, unused_imports)]
pub mod accelerator;
pub mod motions;
pub mod drivers;
#[cfg(test)]
mod tests;

#[macro_use(crate_version,value_t, value_t_or_exit)]
extern crate clap;
use clap::{App, SubCommand};

use drivers::Driver;
use motions::Motion;

fn main() {
  // Create what is used to interpretate user input from the CLI
  let matches = App::new("accelerate")
                  .version(&crate_version!())
                  .global_version(true)
                  .unified_help_message(true)
                  .author("Caleb Meredith <calebmeredith8@gmail.com>\nVictor M. Suarez <svmnotn@gmail.com>")
                  .about("Accelerate back and forth through time for your database or other in-place systems")
                  .args_from_usage("--target=<url>     -t 'the targeted url to accelerate'
                                    --directory=[path] -d 'the directory holding the motions (defaults to the current dir)'")
                  .subcommand(SubCommand::with_name("ls").about("lists all motions in the directory"))
                  .subcommand(SubCommand::with_name("redo").about("subtracts then adds the last motion"))
                  .subcommand(SubCommand::with_name("up").about("adds all remaining motions"))
                  .subcommand(SubCommand::with_name("down").about("subtracts all previous motions"))
                  .subcommand(SubCommand::with_name("reset").about("subtracts then adds all previous motions"))
                  .subcommand(SubCommand::with_name("create")
                                .about("create a new motion using the template")
                                .arg_from_usage("<name> 'the name to use for the new motion'"))
                  .subcommand(SubCommand::with_name("add")
                                .about("adds n motions")
                                .arg_from_usage("[n] 'how many motions to add, defaults to 1'"))
                  .subcommand(SubCommand::with_name("sub")
                                .about("subtracts n motions")
                                .arg_from_usage("[n] 'how many motions to substract, defaults to 1'"))
                  .subcommand(SubCommand::with_name("goto")
                                .about("goes to the nth motion relative to the current motion")
                                .arg_from_usage("--n=<n> 'the amount of motions to move relative to the current one,
                                                 must be given as a number in the form of --n=[number]'"))
                  .get_matches();
  // Get all the specified variables or set them to their default values
  let target = matches.value_of("url").unwrap();
  let directory = matches.value_of("path").unwrap_or(".");
  // TODO change for proper get
  let mots = motions::get();
  // TODO Adquire driver properly!
  let mut driver = get_driver(target);
  // Go through and find what matched
  match matches.subcommand() {
    ("ls", Some(_)) => ls(&mots),
    ("up", Some(_)) => accelerator::up(&mut driver, &mots),
    ("down", Some(_)) => accelerator::down(&mut driver, &mots),
    ("redo", Some(_)) => accelerator::redo(&mut driver, &mots),
    ("reset", Some(_)) => accelerator::reset(&mut driver, &mots),
    ("create", Some(m)) => create(directory.to_string(), value_t!(m.value_of("name"), String).unwrap()),
    ("add", Some(m)) => accelerator::shift(&mut driver, &mots, value_t!(m.value_of("n"), isize).unwrap_or(1)),
    ("sub", Some(m)) => accelerator::shift(&mut driver, &mots, value_t!(m.value_of("n"), isize).unwrap_or(1) * -1),
    ("goto", Some(m)) => accelerator::goto(&mut driver, &mots, value_t_or_exit!(m.value_of("n"), isize)),
    _ => println!("Nothing to do!\nRe-run with --help for more information"),
  }
}

fn get_driver(target: &str) -> Box<Driver> { Box::new(drivers::DefaultDriver::new(target.to_string())) }

fn ls(mots: &Vec<Motion>) {
  for mot in mots {
    println!("{}", mot.name);
  }
}

// TODO Implement
fn create(directory: String, name: String) {}
