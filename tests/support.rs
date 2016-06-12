use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn command() -> Command {
  let mut command = Command::new(path_to_bin());
  command.env_clear().current_dir("tests/fixtures");
  command
}

pub fn assert_output(command: &mut Command, stdout: &str, stderr: &str) {
  let output = command.output().unwrap();
  assert_eq!(String::from_utf8(output.stderr).unwrap(), String::from(stderr));
  assert_eq!(String::from_utf8(output.stdout).unwrap(), String::from(stdout));
}

fn path_to_bin() -> PathBuf {
  env::current_exe().unwrap().parent().unwrap().join("accelerate")
}
