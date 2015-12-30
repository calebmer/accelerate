mod template;
use std::fs;
use std::path::*;
use std::fs::File;
use std::io::prelude::*;

pub struct Motion {
  pub name: String,
  pub version: Vec<usize>,
  pub extension: String,
  pub add: String,
  pub sub: String,
}

impl Motion {
  fn new(tmp: &template::Template, dir: &String, add: String, sub: String) -> Self {
    Motion {
      name: tmp.get_name(&add),
      add: read_file(&dir, &add),
      sub: read_file(&dir, &sub),
      version: version_from_string(tmp, &add),
      extension: tmp.extension.clone(),
    }
  }

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

fn version_from_string(tmp: &template::Template, name: &String) -> Vec<usize> {
  let mut version = Vec::new();
  for v in tmp.regex.replace_all(&name, "$1").split('.') {
    version.push(v.parse().unwrap());
  }
  version
}

fn disambiguate(tmp: &template::Template, name: &String) -> String { tmp.regex.replace_all(&name, "$1,$2") }

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
  while s.len() < max {
    s = 0.to_string() + &s;
  }
  s
}

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
