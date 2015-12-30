mod template;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub struct Motion {
  pub name: String,
  pub add_name: String,
  pub sub_name: String,
  pub add: String,
  pub sub: String,
}

impl Motion {
  fn new(tmp: &template::Template, dir: &String, add: String, sub: String) -> Self {
    Motion {
      name: tmp.get_name(&add),
      add: read_file(&dir, &add),
      sub: read_file(&dir, &sub),
      add_name: add,
      sub_name: sub,
    }
  }

  fn get_test() -> Self {
    Motion {
      name: "motion".to_string(),
      add_name: "motion.add".to_string(),
      sub_name: "motion.sub".to_string(),
      add: "add".to_string(),
      sub: "sub".to_string(),
    }
  }

  pub fn test(n: usize) -> Self {
    Motion {
      name: "motion".to_string(),
      add_name: "motion.add".to_string(),
      sub_name: "motion.sub".to_string(),
      add: "add: ".to_string() + &n.to_string(),
      sub: "sub: ".to_string() + &n.to_string(),
    }
  }
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
  let names = read_directory(&directory);
  let cookie = template::Template::get(&directory, &names);
  let mut motion_names: Vec<String> = names.into_iter().filter(|name| cookie.regex.is_match(name)).collect();
  motion_names.sort();
  let mut motions_add: Vec<String> = Vec::new();
  let mut motions_sub: Vec<String> = Vec::new();

  while let Some(n) = motion_names.pop() {
    match cookie.get_op(&n) {
      template::Op::Add => motions_add.push(n),
      template::Op::Sub => motions_sub.push(n),
    }
  }

  let mut motions: Vec<Motion> = Vec::new();

  while motions_add.len() != 0 {
    let add_name = motions_add.pop().unwrap();
    let sub_name = motions_sub.pop().unwrap();

    if disambiguate(&cookie, &add_name) == disambiguate(&cookie, &sub_name) {
      motions.push(Motion::new(&cookie, directory, add_name, sub_name));
    }
  }

  motions
}
