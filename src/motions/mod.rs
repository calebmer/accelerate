//! This module interacts with motions on the filesystem, turning them into a
//! usable format for both the accelerator and the drivers.
mod template;
use std::fs;
use std::path::*;
use std::fs::File;
use std::io::prelude::*;

/// The motion which will be applied to the driver. Can be either added or
/// subbed.
pub struct Motion {
  /// The disambiguated motion name.
  pub name: String,
  /// The motion semantic version as a vector.
  pub version: Vec<usize>,
  /// The motion’s file extension.
  pub extension: String,
  /// The add file to be executed by the driver.
  pub add: String,
  /// The sub file to be executed by the driver.
  pub sub: String,
}

impl Motion {
  /// Creates a new motion from a set of parameters. Automatically reads the
  /// add and sub files in addition to getting the version.
  fn new(tmp: &template::Template, dir: &String, add: String, sub: String) -> Self {
    Motion {
      name: tmp.get_name(&add),
      add: read_file(&dir, &add),
      sub: read_file(&dir, &sub),
      version: version_from_string(tmp, &add),
      extension: tmp.extension.clone(),
    }
  }

  /// A simple function to be used in tests to get a sample motion object.
  pub fn test(n: usize) -> Self {
    Motion {
      name: "test".to_string(),
      add: "add: ".to_string() + &n.to_string(),
      sub: "sub: ".to_string() + &n.to_string(),
      version: vec![n, n + 1, n + 2],
      extension: String::from(""),
    }
  }
}

/// Searches a directory for all valid motions. The directory must have an add
/// and sub template or else the function cannot validate motion files.
///
/// A template named like `x.x.x-template.add.sql` would match
/// `0.0.1-hello-world.add.sql` and a template like `xxx_template.sub` would
/// match `001_hello_world.sub` and so on.
///
/// All motion files are turned into the special motion structure.
pub fn discover(directory: &String) -> Vec<Motion> {
  let names = read_directory(directory);
  let cookie = template::Template::get(directory, &names);
  let mut motion_names: Vec<String> = names.into_iter().filter(|name| cookie.regex.is_match(name)).collect();
  motion_names.sort();
  let mut motions_add = Vec::new();
  let mut motions_sub = Vec::new();

  while let Some(n) = motion_names.pop() {
    match cookie.get_op(&n) {
      template::Op::Add => motions_add.push(n),
      template::Op::Sub => motions_sub.push(n),
    }
  }

  let mut motions = Vec::new();

  while motions_add.len() != 0 {
    let add_name = motions_add.pop().unwrap();
    let sub_name = motions_sub.pop().unwrap();

    if disambiguate(&cookie, &add_name) == disambiguate(&cookie, &sub_name) {
      motions.push(Motion::new(&cookie, directory, add_name, sub_name));
    }
  }
  motions
}

/// Creates a new motion based off of the directory‘s template file and the
/// last valid motion. The contents of the new pair of add and sub motion files
/// will contain the contents of the template add and sub files respectively.
///
/// This function will increment the semantic version of the last valid motion
/// to be added to the template name. To increment the version the following
/// rules are used:
///
/// - `0.0.001` becomes `0.0.002`.
/// - `0.1.0` becomes `0.1.1`.
/// - `0.01` becomes `0.02`.
/// - `0.0.99` errors because we cannot have more than two minor digits.
///
/// Basically, whichever number in the semantic version represents the smallest
/// change is incremented by 1 and padded with 0s to match the correct length.
/// If there might be more digits then allowed by the template, an error is
/// thrown.
pub fn create(directory: String, name: String) {
  let cookie = template::Template::get(&directory, &read_directory(&directory));
  let motion_last = discover(&directory).pop().unwrap();

  let mut version = motion_last.version.clone();
  let max = version.len();
  version[max - 1] += 1;

  let mut path = PathBuf::from(&directory);
  path.push(version_to_string(&version, &cookie.version) + &cookie.separator + &name + ".add" + &cookie.extension);
  println!("Writting: {:?} to the file system", path);
  {
    let mut f = File::create(path.as_path()).unwrap();
    f.write_all(cookie.add.as_bytes()).unwrap();
  }
  path.pop();
  path.push(version_to_string(&version, &cookie.version) + &cookie.separator + &name + ".sub" + &cookie.extension);
  println!("Writting: {:?} to the file system", path);
  {
    let mut f = File::create(path.as_path()).unwrap();
    f.write_all(cookie.sub.as_bytes()).unwrap();
  }
}

fn version_from_string(tmp: &template::Template, name: &String) -> Vec<usize> {
  let mut version = Vec::new();
  for v in tmp.regex.replace_all(&name, "$1").split('.') {
    version.push(v.parse().unwrap());
  }
  version
}

fn version_to_string(ver: &Vec<usize>, mold: &Vec<usize>) -> String {
  let mut version_str = String::new();
  for i in 0..mold.len() {
    version_str.push_str(&pad_number(ver[i], mold[i]));
    version_str.push('.');
  }
  version_str.pop();
  version_str
}

fn disambiguate(tmp: &template::Template, name: &String) -> String {
  tmp.regex.replace_all(&name, "$1,$2")
}

#[allow(unused_must_use)]
fn read_file(dir: &String, name: &str) -> String {
  let mut f = File::open(dir.clone() + r"\" + name).unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s);
  s
}

fn read_directory(directory: &String) -> Vec<String> {
  let mut names = Vec::new();
  if let Ok(entries) = fs::read_dir(directory.to_string()) {
    for entry in entries {
      if let Ok(file_name) = entry.unwrap().file_name().into_string() {
        names.push(file_name);
      } else {
        panic!("File name did not contain valid Unicode data");
      }
    }
  } else {
    panic!("Directory: '{}' not found!", directory);
  }
  names
}

fn pad_number(num: usize, max: usize) -> String {
  let mut s = num.to_string();
  while s.len() < max {
    s = 0.to_string() + &s;
  }
  s
}
