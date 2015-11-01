#![allow(dead_code, unused_variables, unused_imports)]
pub mod accelerator;
pub mod motions;
pub mod drivers;
#[cfg(test)]
mod tests;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("accelerate")
                        .version("2.0.0")
                        .global_version(true)
                        .unified_help_message(true)
                        .author("Caleb Meredith <calebmeredith8@gmail.com>")
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
                        .subcommand(SubCommand::with_name("ls")
                                               .about("list all motions to be used"))
                        .subcommand(SubCommand::with_name("redo")
                                               .about("subtract then add the last motion"))
                        .subcommand(SubCommand::with_name("up")
                                               .about("add all remaining motions"))
                        .subcommand(SubCommand::with_name("down")
                                               .about("subtract all previous motions"))
                        .subcommand(SubCommand::with_name("reset")
                                               .about("subtract then add all previous motions"))
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
}

fn ls(directory: String) {
    let mots = motions::get(directory);
    for mot in mots{
        println!("{}", mot.name);
    }
}

// TODO Implement
fn create(directory: String, name: String){}
