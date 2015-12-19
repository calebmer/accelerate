pub struct Motion {
  pub name: String,
  pub add_name: String,
  pub sub_name: String,
  pub add: String,
  pub sub: String,
}

impl Motion {
  fn get_test() -> Self {
    Motion {
      name: "motion".to_string(),
      add_name: "motion.add".to_string(),
      sub_name: "motion.sub".to_string(),
      add: "add".to_string(),
      sub: "sub".to_string(),
    }
  }
}

extern crate regex;
use self::regex::Regex;
use std::fs;

struct Template {
  add_name: String,
  sub_name: String,
  version: Vec<isize>,
  separator: String,
  extension: String,
  add: String,
  sub: String,
  regex: Regex,
}

// TODO implement
impl Template {
  fn get(names: &Vec<&str>) {
    let template = Regex::new("/^([x.]+)([\\-_ ~]+)template\\.(add|sub)(.*)$/i").unwrap();
    let names_iter = names.iter();

  }
}

fn read_dir(directory: &String) -> Vec<String> {
  let mut names = Vec::new();
  if let Ok(entries) = fs::read_dir(directory.to_string()) {
    for entry in entries {
      if let Ok(file_name) = entry.unwrap().file_name().into_string() {
        names.push(file_name);
      } else {
        panic!("File Name did not contain valid Unicode data");
      }
    }
  } else {
    panic!("Directory: '{}' not found!", directory);
  }
  names
}

// pub fn discover(directory: &String) -> Vec<Motion>{
// let names = read_dir(directory);
// let template = Template::get(&names);
// }
//

// TODO remove this in favor of real implementation
pub fn get() -> Vec<Motion> { return vec![Motion::get_test()]; }
