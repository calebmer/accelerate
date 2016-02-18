//! This module interacts with motions on the file system, turning them into a
//! usable format for both the accelerator and the drivers.

mod template;

use std::error::Error;
use std::fs;
use std::path::*;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use operation::Operation::*;

/// The motion which will be applied to the driver. Can be either added or
/// subbed.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Motion {
  /// The disambiguated motion name.
  pub name: String,
  /// The motion semantic version as a vector.
  pub version: Vec<usize>,
  /// The motion's file extension.
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
}

impl fmt::Display for Motion {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
           "Motion [name: {}, version: {:?}, extension: {}, add: {}, sub: {}]",
           self.name,
           self.version,
           self.extension,
           self.add,
           self.sub)
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
  let cookie = template::Template::get(&names);
  let mut motion_names: Vec<DirFile> = names.into_iter().filter(|a| cookie.regex.is_match(&a.name)).collect();
  motion_names.sort();
  let mut motions_add = Vec::new();
  let mut motions_sub = Vec::new();

  while let Some(dirf) = motion_names.pop() {
    match cookie.get_op(&dirf.name) {
      Add => motions_add.push(dirf),
      Sub => motions_sub.push(dirf),
    }
  }

  let mut motions = Vec::new();

  while motions_add.len() != 0 {
    if let Some(add) = motions_add.pop() {
      if let Some(sub) = motions_sub.pop() {
        if disambiguate(&cookie, &add.name) == disambiguate(&cookie, &sub.name) {
          motions.push(Motion::new(&cookie, &add.dir, add.name, sub.name));
        }
      } else {
        panic!("Sub Name was none");
      }
    } else {
      panic!("Add Name was none");
    }
  }
  motions
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

fn pad_number(num: usize, max: usize) -> String {
  let mut s = num.to_string();
  if s.len() > max {
    panic!("we cannot have any more minor digits.");
  }
  while s.len() < max {
    s = 0.to_string() + &s;
  }
  s
}

/// Creates a new motion based off of the directory's template file and the
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
pub fn create<E: Error>(directory: String, mut motions: Vec<Motion>, name: String) -> Result<(), E> {
  let cookie = template::Template::get(&read_directory(&directory));
  let motion_last = motions.pop().unwrap();

  let mut version = motion_last.version;
  let max = version.len();
  version[max - 1] += 1;
  let name = version_to_string(&version, &cookie.version) + &cookie.separator + &name;

  let mut path = PathBuf::from(&directory);
  path.push(String::new() + &name + ".add" + &cookie.extension);
  println!("Writting: {:?} to the file system", path);
  if let Ok(mut f) = File::create(path.as_path()) {
    if let Err(e) = f.write_all(cookie.add.as_bytes()) {
      // TODO: Make this an `Err`.
      panic!("Could not write {:?} to the file system due to:\n{}", path, e);
    }
  }
  path.pop();
  path.push(String::new() + &name + ".add" + &cookie.extension);
  println!("Writting: {:?} to the file system", path);
  if let Ok(mut f) = File::create(path.as_path()) {
    if let Err(e) = f.write_all(cookie.sub.as_bytes()) {
      // TODO: Make this an `Err`
      panic!("Could not write {:?} to the file system due to:\n{}", path, e);
    }
  }
  Ok(())
}

fn version_from_string(tmp: &template::Template, name: &String) -> Vec<usize> {
  let mut version = Vec::new();
  for v in tmp.regex.replace_all(&name, "$1").split('.') {
    version.push(v.parse().unwrap());
  }
  version
}

fn disambiguate(tmp: &template::Template, name: &String) -> String {
  tmp.regex.replace_all(&name, "$1,$2")
}

#[allow(unused_must_use)]
fn read_file(dir: &String, name: &str) -> String {
  match File::open(dir.clone() + r"\" + name) {
    Ok(mut f) => {
      let mut s = String::new();
      f.read_to_string(&mut s);
      s
    }
    Err(e) => panic!("Could not read: {}\n in: {}\n due to: {}", name, dir, e),
  }
}

trait Visible {
  fn str(&self) -> String;
}

impl Visible for PathBuf {
  fn str(&self) -> String {
    if let Some(s) = self.to_str() {
      return s.to_string();
    } else {
      panic!("String was none");
    }
  }
}

use std::ffi::OsStr;

impl Visible for OsStr {
  fn str(&self) -> String {
    if let Some(file_name) = self.to_str() {
      file_name.to_string()
    } else {
      panic!("File name did not contain valid Unicode data");
    }
  }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct DirFile {
  dir: String,
  name: String,
}

fn read_directory(directory: &String) -> Vec<DirFile> {
  let mut names = Vec::new();
  if let Ok(entries) = fs::read_dir(directory.to_string()) {
    for entry in entries {
      if let Ok(e) = entry {
        let path = e.path();
        if let Ok(meta) = fs::metadata(&path) {
          if meta.is_dir() {
            names.append(&mut read_directory(&path.str()));
          } else {
            if let Some(file_name) = path.file_name() {
              names.push(DirFile {
                dir: directory.clone(),
                name: file_name.str(),
              });
            } else {
              panic!("File name did not contain valid Unicode data");
            }
          }
        } else {
          panic!("Meta not found!");
        }
      } else {
        panic!("Entry not found!");
      }
    }
  } else {
    panic!("Directory: '{}' not found!", directory);
  }
  names
}
